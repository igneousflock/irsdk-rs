use std::ffi::{c_char, c_double, c_int};

use bytemuck::{AnyBitPattern, Pod, Zeroable};

#[expect(non_camel_case_types, reason = "this mirrors the definition in `libc`")]
type time_t = i64;

const IRSDK_MAX_BUFS: usize = 4;
const IRSDK_MAX_STRING: usize = 32;
const IRSDK_MAX_DESC: usize = 64;

/// The alignment of the [`Header`] type, should always be 16
///
/// ```
/// assert_eq!(irsdk::raw::ALIGNMENT, 16);
/// ```
pub const ALIGNMENT: usize = std::mem::align_of::<Header>();

pub const HEADER_SIZE: usize = std::mem::size_of::<Header>();
pub const SUB_HEADER_SIZE: usize = std::mem::size_of::<DiskSubHeader>();
pub const VAR_HEADER_SIZE: usize = std::mem::size_of::<VarHeader>();

#[derive(Clone, Copy, Debug, AnyBitPattern)]
#[repr(C, align(16))]
pub struct Header {
    // TODO: add assertions on this field
    /// API header version, should always be 2
    pub ver: c_int,
    /// Connected status, should always be 1
    pub status: c_int,
    /// Ticks per second (60 or 360)
    pub tick_rate: c_int,
    /// Incremented when session info changes
    pub session_info_update: c_int,
    /// Length in bytes of the session info string
    pub session_info_len: c_int,
    /// Session info, encoded in YAML
    pub session_info_offset: c_int,

    /// Length of the array pointed to by varHeaderOffset
    pub num_vars: c_int,
    /// Offset to the `VarHeader` array, which describes the variables in [`VarBuf`]
    pub var_header_offset: c_int,

    /// Number of variable buffers
    pub num_buf: c_int,
    /// Length of each variable buffer
    pub buf_len: c_int,

    /// Offsets to each of the variable buffers
    pub var_bufs: [VarBuf; IRSDK_MAX_BUFS],
}

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C, align(16))]
pub struct VarBuf {
    /// Which tick this buffer represents
    pub tick_count: c_int,
    /// Offset from the header
    pub buf_offset: c_int,
    _pad: [c_int; 2],
}

#[derive(Clone, Copy, Debug, AnyBitPattern)]
#[repr(C, align(16))]
pub struct DiskSubHeader {
    /// Timestamp for the start of the session, seconds since epoch
    pub session_start_date: time_t,
    /// How long into the session the run started, in seconds
    pub session_start_time: c_double,
    /// How long into the session the run ended, in seconds
    pub session_end_time: c_double,
    /// Number of laps run in the session
    pub session_lap_count: c_int,
    /// Number of records in the file
    pub session_record_count: c_int,
}

#[derive(Clone, Copy, Debug, AnyBitPattern)]
#[repr(C, align(16))]
pub struct VarHeader {
    pub ty: c_int,
    pub offset: c_int,
    pub count: c_int,
    pub count_as_time: c_int,

    pub name: [c_char; IRSDK_MAX_STRING],
    pub desc: [c_char; IRSDK_MAX_DESC],
    pub unit: [c_char; IRSDK_MAX_STRING],
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

impl VarHeader {
    pub fn slice_from_fraw_bytes(bytes: &[u8]) -> &[Self] {
        bytemuck::cast_slice(bytes)
    }
}
