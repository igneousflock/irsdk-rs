use std::ffi::{c_char, c_double, c_int};

use bytemuck::{AnyBitPattern, Pod, Zeroable};

#[expect(non_camel_case_types, reason = "this mirrors the definition in `libc`")]
type time_t = i64;

const IRSDK_MAX_BUFS: usize = 4;
const IRSDK_MAX_STRING: usize = 32;
const IRSDK_MAX_DESC: usize = 64;

/// The alignment of the [`Header`] type, should always be 16
pub const ALIGNMENT: usize = std::mem::align_of::<Header>();

pub const HEADER_SIZE: usize = std::mem::size_of::<Header>();
pub const SUB_HEADER_SIZE: usize = std::mem::size_of::<DiskSubHeader>();
pub const VAR_HEADER_SIZE: usize = std::mem::size_of::<VarHeader>();

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum RawTelemError {
    /// API version (first four bytes) should always be `2`
    #[error("API version (first four bytes) should always be `2`, got `{0}`")]
    InvalidApiVersion(
        /// the detected API version
        c_int,
    ),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, AnyBitPattern)]
#[repr(C, align(16))]
pub struct Header {
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
    pub(crate) _pad: [c_int; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, AnyBitPattern)]
#[repr(C, align(16))]
pub struct DiskSubHeader {
    /// Timestamp for the start of the session, seconds since epoch
    pub start_date: time_t,
    /// How long into the session the run started, in seconds
    pub start_time: c_double,
    /// How long into the session the run ended, in seconds
    pub end_time: c_double,
    /// Number of laps run in the session
    pub lap_count: c_int,
    /// Number of records in the file
    pub record_count: c_int,
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
    pub fn from_raw_bytes(bytes: &[u8]) -> Result<Self, RawTelemError> {
        let header = *bytemuck::from_bytes::<Self>(bytes);
        if header.ver != 2 {
            return Err(RawTelemError::InvalidApiVersion(header.ver));
        }

        Ok(header)
    }
}

impl DiskSubHeader {
    pub fn from_raw_bytes(bytes: &[u8]) -> Self {
        *bytemuck::from_bytes(bytes)
    }
}

#[cfg(test)]
impl VarBuf {
    pub(crate) fn new(tick_count: c_int, buf_offset: c_int) -> Self {
        Self {
            tick_count,
            buf_offset,
            _pad: [0; 2],
        }
    }
}

impl VarHeader {
    pub fn slice_from_fraw_bytes(bytes: &[u8]) -> &[Self] {
        bytemuck::cast_slice(bytes)
    }
}

#[cfg(test)]
impl VarHeader {
    pub(crate) fn new(
        ty: c_int,
        offset: c_int,
        count: c_int,
        count_as_time: c_char,
        name: &[u8],
        desc: &[u8],
        unit: &[u8],
    ) -> Self {
        use crate::test_utils::test_string;
        Self {
            ty,
            offset,
            count,
            count_as_time,
            _pad: [0; 3],
            name: test_string(name),
            desc: test_string(desc),
            unit: test_string(unit),
        }
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
    use claims::assert_ok_eq;

    use crate::{
        include_bytes_aligned,
        raw::{DiskSubHeader, Header, VarBuf, VarHeader},
        test_utils::test_string,
    };

    #[test]
    fn decodes_raw_header() {
        // sampled from an IBT file
        let raw = include_bytes_aligned!("../test-data/raw_header");
        let header = Header::from_raw_bytes(&raw);

        assert_ok_eq!(
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
                start_date: 1764642265,
                start_time: 52.116666030881774,
                end_time: 219.34999949144233,
                lap_count: 3,
                record_count: 9759,
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
}
