use crate::win;
use crate::win::{TelemetryMemMap, WindowsError};
use ibt::raw;
use ibt::telemetry::{Header, Sample, VarBufInfo, VarHeader, VarSet};
use itertools::Itertools;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_millis(1000);

#[derive(Clone, Debug, thiserror::Error)]
pub enum IRacingClientError {
    #[error("Unknown windows error")]
    Windows(#[source] WindowsError),

    #[error("iRacing is not running")]
    Disconnected,

    #[error(transparent)]
    RawTelemError(#[from] ibt::RawTelemError),

    #[error(transparent)]
    RawConversionError(#[from] ibt::telemetry::RawConversionError),

    #[error(transparent)]
    SignalError(#[from] win::SignalError),
}

impl From<WindowsError> for IRacingClientError {
    fn from(err: WindowsError) -> Self {
        if err.is_file_not_found() {
            Self::Disconnected
        } else {
            Self::Windows(err)
        }
    }
}

#[derive(Debug)]
pub struct IRacingClient {
    mem_map: TelemetryMemMap,

    vars: VarSet,
    buf_len: usize,
}

impl IRacingClient {
    pub fn connect() -> Result<Self, IRacingClientError> {
        let mem_map = TelemetryMemMap::connect()?;

        mem_map.wait_for_event_signal(TIMEOUT)?;
        // SAFETY: we've waited on the signal
        let raw_header = unsafe { mem_map.as_raw_header()? };
        let header = Header::from_raw(&raw_header)?;

        // Read the var headers once
        let vh_offset = raw_header.var_header_offset as usize;
        let vh_len = raw::VAR_HEADER_SIZE * raw_header.num_vars as usize;
        // SAFETY: we've waited on the signal. offset and len come from the header.
        // Data is copied immediately after.
        let vh_slice = unsafe { mem_map.as_slice(vh_offset, vh_len) };

        let var_headers = raw::VarHeader::slice_from_fraw_bytes(vh_slice)
            .iter()
            .map(VarHeader::from_raw)
            .collect();

        Ok(Self {
            mem_map,
            vars: VarSet::new(var_headers),
            buf_len: header.buf_len,
        })
    }

    fn next_raw_header(&self) -> Result<raw::Header, IRacingClientError> {
        self.mem_map.wait_for_event_signal(TIMEOUT)?;
        // SAFETY: we've waited on the signal
        let raw_header = unsafe { self.mem_map.as_raw_header() }?;

        if raw_header.status != 1 {
            return Err(IRacingClientError::Disconnected);
        }

        Ok(raw_header)
    }

    pub fn next_header(&self) -> Result<Header, IRacingClientError> {
        let raw_header = self.next_raw_header()?;
        let header = Header::from_raw(&raw_header)?;

        Ok(header)
    }

    pub fn next_sample(&self) -> Result<Sample<'_>, IRacingClientError> {
        let raw_header = self.next_raw_header()?;

        let newest_var_buf = raw_header
            .var_bufs
            .iter()
            .map(VarBufInfo::from_raw)
            .process_results(|a| a.max_by_key(|vb| vb.tick_count))?
            .expect("there are always four var bufs");

        // SAFETY:
        // - We waited on the signal in `self.next_raw_header()`
        // - Offset and len come from the `VarBuf` in the header
        // - We copy the data with `Sample::new_as_owned`
        let sample_slice = unsafe {
            self.mem_map
                .as_slice(newest_var_buf.buf_offset, self.buf_len)
        };
        Ok(Sample::new_as_owned(sample_slice))
    }

    pub fn next_sample_into_buf<'buf>(
        &self,
        buf: &'buf mut [u8],
    ) -> Result<Sample<'buf>, IRacingClientError> {
        let raw_header = self.next_raw_header()?;

        let newest_var_buf = raw_header
            .var_bufs
            .iter()
            .map(VarBufInfo::from_raw)
            .process_results(|a| a.max_by_key(|vb| vb.tick_count))?
            .expect("there are always four var bufs");

        // SAFETY:
        // - We waited on the signal in `self.next_raw_header()`
        // - Offset and len come from the `VarBuf` in the header
        // - We copy the data into the given buffer before returning
        let sample_slice = unsafe {
            self.mem_map
                .as_slice(newest_var_buf.buf_offset, self.buf_len)
        };

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
