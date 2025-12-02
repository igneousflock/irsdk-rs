use std::ffi::{c_double, c_int};

use bytemuck::{AnyBitPattern, Pod, Zeroable};

#[expect(non_camel_case_types, reason = "this mirrors the definition in `libc`")]
type time_t = i64;

const IRSDK_MAX_BUFS: usize = 4;

/// The alignment of the [`Header`] type, should always be 16
///
/// ```
/// assert_eq!(irsdk::raw::ALIGNMENT, 16);
/// ```
pub const ALIGNMENT: usize = std::mem::align_of::<Header>();

pub const HEADER_SIZE: usize = std::mem::size_of::<Header>();
pub const SUB_HEADER_SIZE: usize = std::mem::size_of::<DiskSubHeader>();

#[derive(Clone, Copy, Debug, AnyBitPattern)]
#[repr(C, align(16))]
pub struct Header {
    ver: c_int,
    status: c_int,
    pub tick_rate: c_int,
    pub session_info_update: c_int,
    session_info_len: c_int,
    session_info_offset: c_int,

    num_vars: c_int,
    var_header_offset: c_int,

    num_buf: c_int,
    buf_len: c_int,

    var_bufs: [VarBuf; IRSDK_MAX_BUFS],
}

#[derive(Clone, Copy, Debug, AnyBitPattern)]
#[repr(C, align(16))]
pub struct DiskSubHeader {
    pub session_start_date: time_t,
    pub session_start_time: c_double,
    pub session_end_time: c_double,
    pub session_lap_count: c_int,
    pub session_record_count: c_int,
}

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C, align(16))]
pub struct VarBuf {
    tick_count: c_int,
    buf_offset: c_int,
    _pad: [c_int; 2],
}

impl Header {
    pub fn from_raw_bytes(bytes: &[u8]) -> Self {
        *bytemuck::from_bytes(bytes)
    }
}

impl DiskSubHeader {
    pub fn from_raw_bytes(bytes: &[u8]) -> Self {
        *bytemuck::from_bytes(bytes)
    }
}
