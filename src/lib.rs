use std::path::Path;

use aligned_vec::{AVec, ConstAlign};

pub mod raw;

#[derive(Clone, Debug)]
pub struct IbtFile {
    data: AVec<u8, ConstAlign<{ crate::raw::ALIGNMENT }>>,

    pub header: raw::Header,
    pub sub_header: raw::DiskSubHeader,
}

impl IbtFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let data = AVec::from_slice(raw::ALIGNMENT, &std::fs::read(&path)?);

        let header = raw::Header::from_raw_bytes(&data[..raw::HEADER_SIZE]);
        let sub_header = raw::DiskSubHeader::from_raw_bytes(
            &data[raw::HEADER_SIZE..raw::HEADER_SIZE + raw::SUB_HEADER_SIZE],
        );

        Ok(Self {
            data,
            header,
            sub_header,
        })
    }
}
