use std::path::Path;

use aligned_vec::{AVec, ConstAlign};
use saphyr::LoadableYamlNode;

use crate::{
    raw,
    telemetry::{DiskSubHeader, Header, RawConversionError, Sample, VarBufInfo, VarHeader, VarSet},
};

#[derive(Debug, thiserror::Error)]
pub enum IbtFileError {
    #[error(transparent)]
    CastError(#[from] RawConversionError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    RawTelem(#[from] raw::RawTelemError),
}

#[derive(Clone, Debug)]
pub struct IbtFile {
    data: AVec<u8, ConstAlign<{ crate::raw::ALIGNMENT }>>,

    pub header: Header,
    pub disk_sub_header: DiskSubHeader,

    pub vars: VarSet,

    pub var_buf_info: VarBufInfo,
}

impl IbtFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, IbtFileError> {
        let data = AVec::from_slice(raw::ALIGNMENT, &std::fs::read(&path)?);

        let raw_header = raw::Header::from_raw_bytes(&data[..raw::HEADER_SIZE])?;
        let header = Header::from_raw(&raw_header)?;

        let raw_sub_header = raw::DiskSubHeader::from_raw_bytes(
            &data[raw::HEADER_SIZE..raw::HEADER_SIZE + raw::SUB_HEADER_SIZE],
        );
        let sub_header = DiskSubHeader::from_raw(&raw_sub_header)?;

        let var_headers_offset = raw_header.var_header_offset as usize;
        let var_headers_len = raw::VAR_HEADER_SIZE * raw_header.num_vars as usize;
        let vh_slice = &data[var_headers_offset..var_headers_offset + var_headers_len];
        let var_headers = raw::VarHeader::slice_from_fraw_bytes(vh_slice)
            .iter()
            .map(VarHeader::from_raw)
            .collect();
        let vars = VarSet::new(var_headers);

        let var_buf_info = VarBufInfo::from_raw(&raw_header.var_bufs[0])?;

        Ok(Self {
            data,
            header,
            disk_sub_header: sub_header,
            vars,
            var_buf_info,
        })
    }

    pub fn raw_session_data(&self) -> String {
        let offset = self.header.session_info_offset;
        let len = self.header.session_info_len;
        let session_string = &self.data[offset..offset + len];
        String::from_utf8_lossy(session_string).into_owned()
    }

    pub fn session_data(&self) -> Result<saphyr::YamlOwned, saphyr::ScanError> {
        let docs = saphyr::YamlOwned::load_from_str(&self.raw_session_data())?;
        Ok(docs[0].clone())
    }

    pub fn sample(&self, idx: usize) -> Sample<'_> {
        assert!(idx < self.disk_sub_header.record_count);
        let len = self.header.buf_len;
        let offset = self.var_buf_info.buf_offset + len * idx;
        Sample::new(&self.data[offset..offset + len])
    }

    pub fn samples(&self) -> impl Iterator<Item = Sample<'_>> {
        (0..self.disk_sub_header.record_count).map(|idx| self.sample(idx))
    }
}
