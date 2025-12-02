use std::path::Path;

use aligned_vec::{AVec, ConstAlign};
use saphyr::LoadableYamlNode;

use crate::{
    raw,
    telemetry::{DiskSubHeader, Header},
};

#[derive(Clone, Debug)]
pub struct IbtFile {
    data: AVec<u8, ConstAlign<{ crate::raw::ALIGNMENT }>>,

    pub header: Header,
    pub disk_sub_header: DiskSubHeader,
}

impl IbtFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let data = AVec::from_slice(raw::ALIGNMENT, &std::fs::read(&path)?);

        let raw_header = raw::Header::from_raw_bytes(&data[..raw::HEADER_SIZE]);
        let header = Header::from_raw(&raw_header);

        let raw_sub_header = raw::DiskSubHeader::from_raw_bytes(
            &data[raw::HEADER_SIZE..raw::HEADER_SIZE + raw::SUB_HEADER_SIZE],
        );
        let sub_header = DiskSubHeader::from_raw(&raw_sub_header);

        Ok(Self {
            data,
            header,
            disk_sub_header: sub_header,
        })
    }

    pub fn raw_session_data(&self) -> String {
        let offset = self.header.session_info_offset();
        let len = self.header.session_info_len();
        let session_string = &self.data[offset..offset + len];
        String::from_utf8_lossy(session_string).into_owned()
    }

    pub fn session_data(&self) -> Result<saphyr::YamlOwned, saphyr::ScanError> {
        let docs = saphyr::YamlOwned::load_from_str(&self.raw_session_data())?;
        Ok(docs[0].clone())
    }
}
