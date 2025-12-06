use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::raw;

#[derive(Clone, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct VarBufInfo {
    pub tick_count: usize,
    pub buf_offset: usize,
}

impl Header {
    pub fn from_raw(raw: &raw::Header) -> Self {
        Self {
            tick_rate: raw
                .tick_rate
                .try_into()
                .expect("`tick_rate` should be positive"),
            session_info_update: raw
                .session_info_update
                .try_into()
                .expect("`session_info_update` should be positive"),
            session_info_len: raw
                .session_info_len
                .try_into()
                .expect("`session_info_len` should be positive"),
            session_info_offset: raw
                .session_info_offset
                .try_into()
                .expect("`session_info_offset` should be positive"),
            num_vars: raw
                .num_vars
                .try_into()
                .expect("`num_vars` should be positive"),
            buf_len: raw
                .buf_len
                .try_into()
                .expect("`buf_len` should be positive"),
        }
    }

    pub fn session_info_len(&self) -> usize {
        self.session_info_len
    }

    pub fn session_info_offset(&self) -> usize {
        self.session_info_offset
    }
}

impl DiskSubHeader {
    pub fn from_raw(raw: &raw::DiskSubHeader) -> Self {
        Self {
            date: DateTime::from_timestamp_secs(raw.session_start_date)
                .expect("`session_start_date` should be a valid timestamp"),
            start_time: Duration::from_secs_f64(raw.session_start_time),
            end_time: Duration::from_secs_f64(raw.session_end_time),
            lap_count: raw
                .session_lap_count
                .try_into()
                .expect("`session_lap_count` should be positive"),
            record_count: raw
                .session_record_count
                .try_into()
                .expect("`session_record_count` should be positive"),
        }
    }
}

impl VarBufInfo {
    pub fn from_raw(raw: &raw::VarBuf) -> Self {
        Self {
            tick_count: raw
                .tick_count
                .try_into()
                .expect("`tick_count` to be positive"),
            buf_offset: raw
                .buf_offset
                .try_into()
                .expect("`buf_offset` to be positive"),
        }
    }
}
