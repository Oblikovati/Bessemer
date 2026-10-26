use super::header_imports::ModelHeaderLoadPlan;
use super::lightweight::ModelLightweightState;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, encode_fields};
use super::{
    detail_definitions, environment, layers, lightweight, model_arrays, part_tail, post_views,
    reference_tail, resolved_start, shared_tail, views,
};
use crate::Result;
use crate::archive_codec::ObjectReader;
use crate::header::{ArchiveVersion, DocumentHeader};
use std::sync::Arc;

pub(super) struct ParsedPartArchive {
    fields: Vec<ModelField>,
    lightweight: ModelLightweightState,
    maximum_bytes: usize,
    header: Arc<DocumentHeader>,
    version: ArchiveVersion,
    limits: ModelFieldLimits,
    kernel: Option<super::kernel_frames::CompressedKernelContext>,
}

pub(super) fn read(
    bytes: &[u8],
    header: Arc<DocumentHeader>,
    version: ArchiveVersion,
    limits: ModelFieldLimits,
) -> Result<ParsedPartArchive> {
    read_with_kernel(bytes, header, version, limits, None)
}

pub(super) fn read_with_kernel(
    bytes: &[u8],
    header: Arc<DocumentHeader>,
    version: ArchiveVersion,
    limits: ModelFieldLimits,
    kernel: Option<super::kernel_frames::CompressedKernelContext>,
) -> Result<ParsedPartArchive> {
    let plan = ModelHeaderLoadPlan::new(header.clone(), version, limits.maximum_entries)?;
    let mut reader = part_decoder(bytes, version, limits, kernel.clone())?;
    let mut map = ObjectReader::new();
    let lightweight = lightweight::read(&mut reader, &mut map, plan)?;
    let size = reader.section_size("resolved_size")?;
    reader.bounded(size, |reader| resolved(reader, &mut map, &lightweight))?;
    Ok(ParsedPartArchive {
        fields: reader.finish()?,
        lightweight,
        maximum_bytes: limits.maximum_bytes,
        header,
        version,
        limits,
        kernel,
    })
}

fn part_decoder(
    bytes: &[u8],
    version: ArchiveVersion,
    limits: ModelFieldLimits,
    kernel: Option<super::kernel_frames::CompressedKernelContext>,
) -> Result<ModelDecoder<'_>> {
    ModelDecoder::new_in_context(
        bytes,
        version,
        limits,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .map(|reader| reader.with_kernel(kernel))
}

fn resolved(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    state: &ModelLightweightState,
) -> Result<()> {
    let flags = resolved_node_flags(state)?;
    resolved_start::read(reader, map)?;
    reader.scope("detail_definitions", |reader| {
        detail_definitions::read(reader, map)
    })?;
    reader.scope("after_details", |reader| environment::read(reader, map))?;
    reader.scope("views", |reader| views::read(reader, map))?;
    reader.scope("postviews", |reader| post_views::read(reader, map))?;
    reader.scope("arrays", |reader| model_arrays::read(reader, map))?;
    reader.scope("layers", |reader| layers::read(reader, map))?;
    reader.scope("reference_tail", |reader| {
        reference_tail::read(reader, map, flags)
    })?;
    reader.scope("shared_model_tail", |reader| shared_tail::read(reader, map))?;
    reader.scope("part_resolved_tail", |reader| {
        part_tail::read(reader, map, state.raw_1614)
    })
}

fn resolved_node_flags(state: &ModelLightweightState) -> Result<u32> {
    state.node.unknown_2c.ok_or_else(|| {
        super::invalid(format!(
            "node flags {:?}; expected stored u32 flags for the modern resolved stage",
            state.node.unknown_2c
        ))
    })
}

impl ParsedPartArchive {
    pub fn with_fields(&self, mut fields: Vec<ModelField>) -> Result<Self> {
        super::construction::section_lengths(&mut fields, self.maximum_bytes)?;
        let bytes = encode_fields(&fields, self.maximum_bytes)?;
        let parsed = read_with_kernel(
            &bytes,
            self.header.clone(),
            self.version,
            self.limits,
            self.kernel.clone(),
        )?;
        if parsed.fields != fields {
            return Err(super::invalid(
                "constructed fields did not reproduce their paths, types or resolved object identities; expected exact schema agreement",
            ));
        }
        Ok(parsed)
    }

    pub fn fields(&self) -> &[ModelField] {
        &self.fields
    }

    pub fn lightweight(&self) -> &ModelLightweightState {
        &self.lightweight
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        encode_fields(&self.fields, self.maximum_bytes)
    }
}
