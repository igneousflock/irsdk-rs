use std::path::Path;

use aligned_vec::{AVec, ConstAlign};

use crate::{
    raw,
    telemetry::{DiskHeader, Header},
};

#[derive(Clone, Debug)]
pub struct IbtFile {
    _data: AVec<u8, ConstAlign<{ crate::raw::ALIGNMENT }>>,

    pub header: Header,
    pub sub_header: DiskHeader,
}

impl IbtFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let data = AVec::from_slice(raw::ALIGNMENT, &std::fs::read(&path)?);

        let raw_header = raw::Header::from_raw_bytes(&data[..raw::HEADER_SIZE]);
        let header = Header::from_raw(&raw_header);

        let raw_sub_header = raw::DiskSubHeader::from_raw_bytes(
            &data[raw::HEADER_SIZE..raw::HEADER_SIZE + raw::SUB_HEADER_SIZE],
        );
        let sub_header = DiskHeader::from_raw(&raw_sub_header);

        Ok(Self {
            _data: data,
            header,
            sub_header,
        })
    }
}
