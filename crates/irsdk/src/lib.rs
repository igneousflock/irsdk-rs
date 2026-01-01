use ibt::raw;
use ibt::telemetry::{Header, Sample, VarBufInfo, VarHeader, VarSet};
use itertools::Itertools;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Memory::{
    FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile, OpenFileMappingW, UnmapViewOfFile,
};
use windows::Win32::System::Threading::{
    OpenEventW, SYNCHRONIZATION_SYNCHRONIZE, WaitForSingleObject,
};
use windows::core::{PCWSTR, w};

const MEM_MAP_FILE_NAME: PCWSTR = w!(r"Local\IRSDKMemMapFileName");
const DATA_VALID_EVENT_NAME: PCWSTR = w!(r"Local\IRSDKDataValidEvent");
const TIMEOUT_MS: u32 = 1000;

#[derive(Clone, Debug, thiserror::Error)]
pub enum IRacingClientError {
    #[error(transparent)]
    Windows(#[from] windows::core::Error),

    #[error(transparent)]
    RawTelemError(#[from] ibt::RawTelemError),

    #[error(transparent)]
    RawConversionError(#[from] ibt::telemetry::RawConversionError),
}

#[derive(Debug)]
pub struct IRacingClient {
    file_mapping_handle: HANDLE,
    mem_map_address: MEMORY_MAPPED_VIEW_ADDRESS,
    event_handle: HANDLE,

    vars: VarSet,
    buf_len: usize,
}

impl IRacingClient {
    pub fn connect() -> Result<Self, IRacingClientError> {
        let file_mapping_handle =
            unsafe { OpenFileMappingW(FILE_MAP_READ.0, false, MEM_MAP_FILE_NAME)? };
        let mem_map_address = unsafe { MapViewOfFile(file_mapping_handle, FILE_MAP_READ, 0, 0, 0) };
        let event_handle =
            unsafe { OpenEventW(SYNCHRONIZATION_SYNCHRONIZE, false, DATA_VALID_EVENT_NAME)? };

        let mut client = Self {
            file_mapping_handle,
            mem_map_address,
            event_handle,
            vars: VarSet::new(vec![]),
            buf_len: 0,
        };

        let raw_header = client.next_raw_header()?;
        let header = Header::from_raw(&raw_header)?;
        client.buf_len = header.buf_len;

        // Read the var headers once
        let vh_offset = raw_header.var_header_offset as usize;
        let vh_len = raw::VAR_HEADER_SIZE * raw_header.num_vars as usize;
        let vh_slice = unsafe { client.slice(vh_offset, vh_len) };

        let var_headers = raw::VarHeader::slice_from_fraw_bytes(vh_slice)
            .iter()
            .map(VarHeader::from_raw)
            .collect();
        client.vars = VarSet::new(var_headers);

        Ok(client)
    }

    unsafe fn slice(&self, offset: usize, len: usize) -> &[u8] {
        unsafe {
            let ptr = (self.mem_map_address.Value as *const u8).add(offset);
            std::slice::from_raw_parts(ptr, len)
        }
    }

    fn next_raw_header(&self) -> Result<raw::Header, IRacingClientError> {
        // wait for event signal
        unsafe { WaitForSingleObject(self.event_handle, TIMEOUT_MS) };

        // read the header
        let ptr = self.mem_map_address.Value as *const raw::Header;
        let raw_header = unsafe { raw::Header::from_raw_ptr(ptr)? };

        Ok(raw_header)
    }

    pub fn next_header(&self) -> Result<Header, IRacingClientError> {
        let raw_header = self.next_raw_header()?;
        let header = Header::from_raw(&raw_header)?;

        Ok(header)
    }

    pub fn next_sample(&self) -> Result<Sample<'_>, IRacingClientError> {
        let raw_header = self.next_raw_header()?;
        let header = Header::from_raw(&raw_header)?;

        let newest_var_buf = raw_header
            .var_bufs
            .iter()
            .map(VarBufInfo::from_raw)
            .process_results(|a| a.max_by_key(|vb| vb.tick_count))?
            .expect("there are always four var bufs");

        let sample_slice = unsafe { self.slice(newest_var_buf.buf_offset, header.buf_len) };
        Ok(Sample::new_as_owned(sample_slice))
    }

    pub fn next_sample_into_buf<'buf>(
        &self,
        buf: &'buf mut [u8],
    ) -> Result<Sample<'buf>, IRacingClientError> {
        let raw_header = self.next_raw_header()?;
        let header = Header::from_raw(&raw_header)?;

        let newest_var_buf = raw_header
            .var_bufs
            .iter()
            .map(VarBufInfo::from_raw)
            .process_results(|a| a.max_by_key(|vb| vb.tick_count))?
            .expect("there are always four var bufs");

        let sample_slice = unsafe { self.slice(newest_var_buf.buf_offset, header.buf_len) };

        // copy the slice into the buffer
        buf.clone_from_slice(sample_slice);
        Ok(Sample::new(buf))
    }

    pub fn vars(&self) -> &VarSet {
        &self.vars
    }

    pub fn buf_len(&self) -> usize {
        self.buf_len
    }
}

impl Drop for IRacingClient {
    fn drop(&mut self) {
        unsafe {
            // TODO: verify safety. if one of these fails, can we still close the others?
            let _ = CloseHandle(self.event_handle);
            let _ = UnmapViewOfFile(self.mem_map_address);
            let _ = CloseHandle(self.file_mapping_handle);
        }
    }
}
