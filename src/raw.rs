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

#[derive(Clone, Copy, Debug, PartialEq, Eq, AnyBitPattern)]
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

#[derive(Clone, Copy, Debug, Eq, Pod, Zeroable)]
#[repr(C, align(16))]
pub struct VarBuf {
    /// Which tick this buffer represents
    pub tick_count: c_int,
    /// Offset from the header
    pub buf_offset: c_int,
    _pad: [c_int; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, AnyBitPattern)]
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

#[derive(Clone, Copy, Debug, Eq, AnyBitPattern)]
#[repr(C, align(16))]
pub struct VarHeader {
    pub ty: c_int,
    pub offset: c_int,
    pub count: c_int,
    pub count_as_time: c_char,
    _pad: [c_char; 3],
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

impl PartialEq for VarBuf {
    fn eq(&self, other: &Self) -> bool {
        self.tick_count == other.tick_count && self.buf_offset == other.buf_offset
    }
}

impl PartialEq for VarHeader {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
            && self.offset == other.offset
            && self.count == other.count
            && self.count_as_time == other.count_as_time
            && self.name == other.name
            && self.desc == other.desc
            && self.unit == other.unit
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        include_bytes_aligned,
        raw::{DiskSubHeader, Header, VarBuf, VarHeader},
    };

    #[test]
    fn decodes_raw_header() {
        // sampled from an IBT file
        let raw = include_bytes_aligned!("../test-data/raw_header");
        let header = Header::from_raw_bytes(&raw);

        assert_eq!(
            header,
            Header {
                ver: 2,
                status: 1,
                tick_rate: 60,
                session_info_update: 0,
                session_info_len: 15654,
                session_info_offset: 40320,
                num_vars: 279,
                var_header_offset: 144,
                num_buf: 1,
                buf_len: 1081,
                var_bufs: [
                    VarBuf {
                        tick_count: 2122,
                        buf_offset: 55974,
                        _pad: [0, 0],
                    },
                    VarBuf {
                        tick_count: 0,
                        buf_offset: 0,
                        _pad: [0, 0],
                    },
                    VarBuf {
                        tick_count: 0,
                        buf_offset: 0,
                        _pad: [0, 0],
                    },
                    VarBuf {
                        tick_count: 0,
                        buf_offset: 0,
                        _pad: [0, 0],
                    },
                ],
            }
        );
    }

    #[test]
    fn decodes_raw_disk_sub_header() {
        // sampled from an IBT file
        let raw = include_bytes_aligned!("../test-data/raw_sub_header");
        let disk_sub_header = DiskSubHeader::from_raw_bytes(&raw);

        assert_eq!(
            disk_sub_header,
            DiskSubHeader {
                session_start_date: 1764642265,
                session_start_time: 52.116666030881774,
                session_end_time: 219.34999949144233,
                session_lap_count: 3,
                session_record_count: 9759,
            }
        );
    }

    #[test]
    fn decodes_var_header() {
        // sampled from an IBT file
        let raw = include_bytes_aligned!("../test-data/raw_var_header");
        let var_header = VarHeader::slice_from_fraw_bytes(&raw);

        assert_eq!(
            var_header,
            [VarHeader {
                ty: 5,
                offset: 0,
                count: 1,
                count_as_time: 0,
                _pad: [0, 0, 0],
                name: test_string(b"SessionTime"),
                desc: test_string(b"Seconds since session start"),
                unit: test_string(b"s"),
            }]
        );
    }

    /// Copies the given bytes to an array and fills the leftovers with `b'\0''`
    fn test_string<const N: usize>(val: &[u8]) -> [i8; N] {
        assert!(N > val.len());
        let mut arr = [0; N];
        arr[0..val.len()].clone_from_slice(val);

        arr.map(|b| b as i8)
    }
}
