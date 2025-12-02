use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::raw;

#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub tick_rate: u32,

    /// Incremented when session info changes
    pub session_info_update: u32,

    /// Length in bytes of the session info string
    session_info_len: usize,
    /// Session info, encoded in YAML
    session_info_offset: usize,
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
}

impl Header {
    pub fn from_raw(raw: &raw::Header) -> Self {
        Self {
            tick_rate: raw.tick_rate.try_into().unwrap(),
            session_info_update: raw.session_info_update.try_into().unwrap(),
            session_info_len: raw.session_info_len().try_into().unwrap(),
            session_info_offset: raw.session_info_offset().try_into().unwrap(),
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
            date: DateTime::from_timestamp_secs(raw.session_start_date).unwrap(),
            start_time: Duration::from_secs_f64(raw.session_start_time),
            end_time: Duration::from_secs_f64(raw.session_end_time),
            lap_count: raw.session_lap_count.try_into().unwrap(),
        }
    }
}
