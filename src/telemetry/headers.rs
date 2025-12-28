use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::raw;

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("field at struct offset `{offset}` could not be cast")]
pub struct CastError {
    offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub tick_rate: u32,

    /// Incremented when session info changes
    pub session_info_update: u32,

    /// Length in bytes of the session info string
    pub session_info_len: usize,
    /// Session info, encoded in YAML
    pub session_info_offset: usize,

    pub num_vars: usize,
    pub buf_len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiskSubHeader {
    /// Timestamp for the start of the session
    pub date: DateTime<Utc>,

    /// How long into the session the run started
    pub start_time: Duration,
    /// How long into the session the run ended
    pub end_time: Duration,

    /// Number of laps run in the session
    pub lap_count: u32,

    // Number of records in this file
    pub record_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VarBufInfo {
    pub tick_count: usize,
    pub buf_offset: usize,
}

/// Calls `try_into` on `<raw as raw_container>.field`, returning an error with the byte offset of
/// the field if the conversion fails. Relies on type inference to determine output type.
macro_rules! cast_field {
    ($raw:ident, $field:ident, $raw_container:ty) => {{
        use crate::telemetry::CastError;
        $raw.$field.try_into().map_err(|_| CastError {
            offset: std::mem::offset_of!($raw_container, $field),
        })
    }};
}

impl Header {
    pub fn from_raw(raw: &raw::Header) -> Result<Self, CastError> {
        Ok(Self {
            tick_rate: cast_field!(raw, tick_rate, raw::Header)?,
            session_info_update: cast_field!(raw, session_info_update, raw::Header)?,
            session_info_len: cast_field!(raw, session_info_len, raw::Header)?,
            session_info_offset: cast_field!(raw, session_info_offset, raw::Header)?,
            num_vars: cast_field!(raw, num_vars, raw::Header)?,
            buf_len: cast_field!(raw, buf_len, raw::Header)?,
        })
    }
}

impl DiskSubHeader {
    pub fn from_raw(raw: &raw::DiskSubHeader) -> Result<Self, CastError> {
        Ok(Self {
            date: DateTime::from_timestamp_secs(raw.start_date)
                .expect("`session_start_date` should be a valid timestamp"),
            start_time: Duration::from_secs_f64(raw.start_time),
            end_time: Duration::from_secs_f64(raw.end_time),
            lap_count: cast_field!(raw, lap_count, raw::DiskSubHeader)?,
            record_count: cast_field!(raw, record_count, raw::DiskSubHeader)?,
        })
    }
}

impl VarBufInfo {
    pub fn from_raw(raw: &raw::VarBuf) -> Result<Self, CastError> {
        Ok(Self {
            tick_count: cast_field!(raw, tick_count, raw::VarBuf)?,
            buf_offset: cast_field!(raw, buf_offset, raw::VarBuf)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::{DateTime, Utc};
    use claims::assert_ok_eq;

    use crate::{
        raw,
        telemetry::{DiskSubHeader, Header, VarBufInfo},
    };

    #[test]
    fn decodes_header() {
        let raw = raw::Header {
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
            var_bufs: [raw::VarBuf::new(0, 0); 4],
        };
        let header = Header::from_raw(&raw);

        assert_ok_eq!(
            header,
            Header {
                tick_rate: 60,
                session_info_update: 0,
                session_info_len: 15654,
                session_info_offset: 40320,
                num_vars: 279,
                buf_len: 1081,
            }
        );
    }

    #[test]
    fn descodes_var_buf_info() {
        let raw = raw::VarBuf::new(1234, 5678);
        let info = VarBufInfo::from_raw(&raw);

        assert_ok_eq!(
            info,
            VarBufInfo {
                tick_count: 1234,
                buf_offset: 5678,
            }
        );
    }

    #[test]
    fn decodes_disk_sub_header() {
        let now = Utc::now();
        let raw = raw::DiskSubHeader {
            start_date: now.timestamp(),
            start_time: 100.0,
            end_time: 200.0,
            lap_count: 3,
            record_count: 9759,
        };
        let sub_header = DiskSubHeader::from_raw(&raw);

        assert_ok_eq!(
            sub_header,
            DiskSubHeader {
                date: DateTime::from_timestamp_secs(now.timestamp()).unwrap(),
                start_time: Duration::from_secs(100),
                end_time: Duration::from_secs(200),
                lap_count: 3,
                record_count: 9759
            }
        );
    }
}
