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
    Io(#[from] std::io::Error),

    /// An error occured converting values from raw telemetry
    #[error(transparent)]
    RawConversionError(#[from] RawConversionError),

    /// An error occured decoding telemetry data
    #[error(transparent)]
    RawTelem(#[from] raw::RawTelemError),
}

/// The contents of a `.ibt` file
///
/// These files are broken up into a header, disk sub-header,
/// variable headers, variable data, and a session string.
///
/// # Example
/// ```ignore
/// # use ibt::IbtFile;
///
/// let ibt_file = IbtFile::from_file("example-telemetry-file.ibt").unwrap();
/// let header = ibt_file.vars.var("RPM").unwrap();
/// let rpm = ibt_file.sample(0).read_var(header);
/// ```
#[derive(Clone, Debug)]
pub struct IbtFile {
    /// Currently the entirety of the file. Must be aligned to 16-bytes to safely read multi-byte
    /// data.
    data: AVec<u8, ConstAlign<{ crate::raw::ALIGNMENT }>>,

    pub header: Header,
    pub disk_sub_header: DiskSubHeader,

    /// Lists what variables are available
    pub vars: VarSet,

    /// IBT files only have on variable buffer containing all samples
    pub var_buf_info: VarBufInfo,
}

impl IbtFile {
    /// Open an IBT file at the given path
    ///
    /// # Errors
    ///
    /// Returns an error if the data is invalid or an IO error occurs.
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

    /// Decode the session string as a plain String
    pub fn raw_session_data(&self) -> String {
        let offset = self.header.session_info_offset;
        let len = self.header.session_info_len;
        let session_string = &self.data[offset..offset + len];
        String::from_utf8_lossy(session_string).into_owned()
    }

    /// Parse the session string as YAML
    pub fn session_data(&self) -> Result<saphyr::YamlOwned, saphyr::ScanError> {
        let docs = saphyr::YamlOwned::load_from_str(&self.raw_session_data())?;
        Ok(docs[0].clone())
    }

    /// Retrive the nth sample
    pub fn sample(&self, idx: usize) -> Sample<'_> {
        assert!(idx < self.disk_sub_header.record_count);
        let sample_len = self.header.buf_len;
        let offset = self.var_buf_info.buf_offset + sample_len * idx;
        Sample::new(&self.data[offset..offset + sample_len])
    }

    /// Iterate over all telemetry samples in the file
    pub fn samples(&self) -> impl Iterator<Item = Sample<'_>> {
        (0..self.disk_sub_header.record_count).map(|idx| self.sample(idx))
    }
}
