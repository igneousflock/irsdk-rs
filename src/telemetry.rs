use chrono::{DateTime, Utc};

use crate::raw;

#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub tick_rate: u32,
    pub session_info_update: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct DiskSubHeader {
    pub session_date: DateTime<Utc>,

    pub session_start_time: f64,
    pub session_end_time: f64,
    pub session_lap_count: u32,
}

impl Header {
    pub fn from_raw(raw: &raw::Header) -> Self {
        Self {
            tick_rate: raw.tick_rate.try_into().unwrap(),
            session_info_update: raw.session_info_update.try_into().unwrap(),
        }
    }
}

impl DiskSubHeader {
    pub fn from_raw(raw: &raw::DiskSubHeader) -> Self {
        Self {
            session_date: DateTime::from_timestamp_secs(raw.session_start_date).unwrap(),
            session_start_time: raw.session_start_time,
            session_end_time: raw.session_end_time,
            session_lap_count: raw.session_lap_count.try_into().unwrap(),
        }
    }
}
