//! Utilities for decoding [iRacing][ir] telemetry data from `.ibt` files or the telemetry API
//!
//! [ir]: https://iracing.com

mod aligned;
mod file;
pub mod raw;
pub mod telemetry;

#[cfg(test)]
mod test_utils;

pub use file::{IbtFile, IbtFileError};
pub use raw::RawTelemError;
pub use saphyr;
