use super::header_imports::ModelHeaderLoadPlan;
use super::lightweight::ModelLightweightState;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, encode_fields};
use super::{lightweight, part_tail, reference_tail, resolved_model, shared_tail};
use crate::Result;
use crate::archive_codec::{ArchiveMapIndex, ObjectMapSnapshot, ObjectReader};
use crate::header::{ArchiveVersion, DocumentHeader};
use std::sync::Arc;

pub(super) struct ParsedPartArchive {
    fields: Vec<ModelField>,
    wire_bytes: usize,
    lightweight: ModelLightweightState,
    maximum_bytes: usize,
    header: Arc<DocumentHeader>,
    version: ArchiveVersion,
    limits: ModelFieldLimits,
    kernel: Option<super::kernel_frames::CompressedKernelContext>,
    sketch_bend_owned: Option<super::SketchBendOwnedProfile>,
    pub(super) object_map: ObjectMapSnapshot,
}

// Only a completed ParsedPartArchive can issue this operation-local root proof.
#[derive(Debug, Clone, Copy)]
pub(super) struct VerifiedPartRoot(ArchiveMapIndex);

impl VerifiedPartRoot {
    /// Use the proven map identity for owner comparisons, for example `root.index()`.
    pub(super) fn index(self) -> ArchiveMapIndex {
        self.0
    }
}

#[cfg(test)]
pub(super) fn read(
    bytes: &[u8],
    header: Arc<DocumentHeader>,
    version: ArchiveVersion,
    limits: ModelFieldLimits,
) -> Result<ParsedPartArchive> {
    read_with_profile(bytes, header, version, limits, None, None)
}

pub(super) fn read_with_profile(
    bytes: &[u8],
    header: Arc<DocumentHeader>,
    version: ArchiveVersion,
    limits: ModelFieldLimits,
    kernel: Option<super::kernel_frames::CompressedKernelContext>,
    sketch_bend_owned: Option<super::SketchBendOwnedProfile>,
) -> Result<ParsedPartArchive> {
    let plan = ModelHeaderLoadPlan::new(header.clone(), version, limits.maximum_entries)?;
    let mut reader = part_decoder(bytes, version, limits, kernel.clone(), sketch_bend_owned)?;
    let mut map = ObjectReader::new();
    let lightweight = lightweight::read(&mut reader, &mut map, plan)?;
    let size = reader.section_size("resolved_size")?;
    reader.bounded(size, |reader| {
        resolved(reader, &mut map, &lightweight.node, lightweight.raw_1614)
    })?;
    Ok(ParsedPartArchive {
        fields: reader.finish()?,
        wire_bytes: bytes.len(),
        lightweight,
        maximum_bytes: limits.maximum_bytes,
        header,
        version,
        limits,
        kernel,
        sketch_bend_owned,
        object_map: map.into_snapshot(),
    })
}

fn part_decoder(
    bytes: &[u8],
    version: ArchiveVersion,
    limits: ModelFieldLimits,
    kernel: Option<super::kernel_frames::CompressedKernelContext>,
    sketch_bend_owned: Option<super::SketchBendOwnedProfile>,
) -> Result<ModelDecoder<'_>> {
    ModelDecoder::new_in_context(
        bytes,
        version,
        limits,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .map(|reader| {
        reader
            .with_kernel(kernel)
            .with_sketch_bend_owned_profile(sketch_bend_owned)
    })
}

/// Reuse the resolved body with its actual lightweight selectors and shared map.
/// Example: `part_archive::resolved(reader, map, &node, raw_1614)?`.
pub(super) fn resolved(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    node: &super::node::ModelNodePrefix,
    raw_1614: i32,
) -> Result<()> {
    let flags = resolved_node_flags(node)?;
    resolved_model::prefix(reader, map)?;
    reader.scope("reference_tail", |reader| {
        reference_tail::read(reader, map, flags)
    })?;
    reader.scope("shared_model_tail", |reader| shared_tail::read(reader, map))?;
    reader.scope("part_resolved_tail", |reader| {
        part_tail::read(reader, map, raw_1614)
    })
}

fn resolved_node_flags(node: &super::node::ModelNodePrefix) -> Result<u32> {
    node.unknown_2c.ok_or_else(|| {
        super::invalid(format!(
            "node flags {:?}; expected stored u32 flags for the modern resolved stage",
            node.unknown_2c
        ))
    })
}

impl ParsedPartArchive {
    /// Issue proof from this completed archive, for example `parsed.verified_root()`.
    pub(super) fn verified_root(&self) -> VerifiedPartRoot {
        VerifiedPartRoot(self.lightweight.root_identity)
    }

    pub fn wire_bytes(&self) -> usize {
        self.wire_bytes
    }

    pub fn with_fields(&self, fields: Vec<ModelField>) -> Result<Self> {
        construct(
            fields,
            self.header.clone(),
            self.version,
            self.limits,
            self.kernel.clone(),
            self.sketch_bend_owned,
        )
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

pub(super) fn construct(
    mut fields: Vec<ModelField>,
    header: Arc<DocumentHeader>,
    version: ArchiveVersion,
    limits: ModelFieldLimits,
    kernel: Option<super::kernel_frames::CompressedKernelContext>,
    sketch_bend_owned: Option<super::SketchBendOwnedProfile>,
) -> Result<ParsedPartArchive> {
    super::construction::section_lengths(&mut fields, limits.maximum_bytes)?;
    let bytes = encode_fields(&fields, limits.maximum_bytes)?;
    let parsed = read_with_profile(&bytes, header, version, limits, kernel, sketch_bend_owned)?;
    if parsed.fields != fields {
        return Err(super::field_mismatch::construction_mismatch(
            &fields,
            &parsed.fields,
        ));
    }
    Ok(parsed)
}
