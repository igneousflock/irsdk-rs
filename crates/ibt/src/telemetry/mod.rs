//! Structured telemetry data

pub mod bitfields;
pub mod enums;
mod headers;
mod sample;
mod var;

pub use headers::{DiskSubHeader, Header, RawConversionError, VarBufInfo};
pub use sample::{Sample, Value};
pub use var::{VarHeader, VarSet, VarType};
