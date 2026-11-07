use super::annotation_view_tests::LIMITS;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits};
use crate::Result;
use crate::archive_codec::ObjectReader;
use crate::header::ArchiveVersion;

pub(super) fn decode_collection(
    version: u32,
    bytes: &[u8],
    read: impl FnOnce(&mut ModelDecoder<'_>, &mut ObjectReader) -> Result<()>,
) -> Result<Vec<ModelField>> {
    decode_limited(version, bytes, LIMITS, read)
}

pub(super) fn decode_limited(
    version: u32,
    bytes: &[u8],
    limits: ModelFieldLimits,
    read: impl FnOnce(&mut ModelDecoder<'_>, &mut ObjectReader) -> Result<()>,
) -> Result<Vec<ModelField>> {
    decode_kernel(version, bytes, limits, None, read)
}

pub(super) fn decode_kernel(
    version: u32,
    bytes: &[u8],
    limits: ModelFieldLimits,
    kernel: Option<super::kernel_frames::CompressedKernelContext>,
    read: impl FnOnce(&mut ModelDecoder<'_>, &mut ObjectReader) -> Result<()>,
) -> Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        limits,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?
    .with_kernel(kernel);
    read(&mut reader, &mut ObjectReader::new())?;
    reader.finish()
}
