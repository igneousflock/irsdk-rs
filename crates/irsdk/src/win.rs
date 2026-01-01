use std::time::Duration;

use ibt::{RawTelemError, raw};
use windows::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};
use windows::Win32::System::Memory::{
    FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile, OpenFileMappingW, UnmapViewOfFile,
};
use windows::Win32::System::Threading::{
    OpenEventW, SYNCHRONIZATION_SYNCHRONIZE, WaitForSingleObject,
};
use windows::core::{PCWSTR, w};

const MEM_MAP_FILE_NAME: PCWSTR = w!(r"Local\IRSDKMemMapFileName");
const DATA_VALID_EVENT_NAME: PCWSTR = w!(r"Local\IRSDKDataValidEvent");
const FILE_NOT_FOUND_CODE: i32 = 0x80070002u32 as i32;

#[derive(Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub struct WindowsError(#[from] windows::core::Error);

impl WindowsError {
    fn from_last_error() -> Self {
        let code = unsafe { GetLastError().to_hresult() };
        let err = windows::core::Error::from_hresult(code);
        err.into()
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum SignalError {
    #[error("Timeout waiting for signal")]
    Timeout,
    #[error(transparent)]
    Windows(WindowsError),
}

impl WindowsError {
    pub fn is_file_not_found(&self) -> bool {
        self.0.code().0 == FILE_NOT_FOUND_CODE
    }
}

#[derive(Debug)]
pub struct TelemetryMemMap {
    file_mapping_handle: HANDLE,
    mem_map_address: MEMORY_MAPPED_VIEW_ADDRESS,
    event_handle: HANDLE,
}

impl TelemetryMemMap {
    pub fn connect() -> Result<Self, WindowsError> {
        let file_mapping_handle =
            unsafe { OpenFileMappingW(FILE_MAP_READ.0, false, MEM_MAP_FILE_NAME)? };
        let mem_map_address = unsafe { MapViewOfFile(file_mapping_handle, FILE_MAP_READ, 0, 0, 0) };
        let event_handle =
            unsafe { OpenEventW(SYNCHRONIZATION_SYNCHRONIZE, false, DATA_VALID_EVENT_NAME)? };

        Ok(Self {
            file_mapping_handle,
            mem_map_address,
            event_handle,
        })
    }

    pub fn wait_for_event_signal(&self, timeout: Duration) -> Result<(), SignalError> {
        let result = unsafe { WaitForSingleObject(self.event_handle, timeout.as_millis() as u32) };
        // see https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject#return-value
        match result.0 {
            0x0 => Ok(()),
            0x102 => Err(SignalError::Timeout),
            0xFFFFFFFF => Err(SignalError::Windows(WindowsError::from_last_error())),
            _ => unreachable!(),
        }
    }

    pub unsafe fn as_raw_header(&self) -> Result<raw::Header, RawTelemError> {
        let ptr = self.mem_map_address.Value as *const raw::Header;
        unsafe { raw::Header::from_raw_ptr(ptr) }
    }

    pub unsafe fn as_slice(&self, offset: usize, len: usize) -> &[u8] {
        unsafe {
            let ptr = (self.mem_map_address.Value as *const u8).add(offset);
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl Drop for TelemetryMemMap {
    fn drop(&mut self) {
        unsafe {
            // TODO: verify safety. if one of these fails, can we still close the others?
            let _ = CloseHandle(self.event_handle);
            let _ = UnmapViewOfFile(self.mem_map_address);
            let _ = CloseHandle(self.file_mapping_handle);
        }
    }
}
