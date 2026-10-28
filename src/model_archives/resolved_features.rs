use super::configuration_manager::{ParsedConfigurationManager, SavedHelperState};
use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelObjectRole, PartConfigurationArchive,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectMapSnapshot, ObjectReader};

#[derive(Debug)]
pub(super) struct ParsedResolvedFeatures {
    pub fields: Vec<ModelField>,
    pub roots: usize,
    pub final_map: ObjectMapSnapshot,
}

pub(super) fn parse(
    bytes: &[u8],
    source: &PartConfigurationArchive,
    manager: &ParsedConfigurationManager,
    limits: ModelFieldLimits,
) -> Result<ParsedResolvedFeatures> {
    if manager.version != source.context.version {
        return Err(super::invalid(format!(
            "CMgr version {}; expected source configuration version {}",
            manager.version.value(),
            source.context.version.value()
        )));
    }
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        source.context.version,
        limits,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?
    .with_inherited_fields(source.fields())
    .with_verified_part_root(source.parsed.verified_root())
    .with_kernel(source.context.kernel.clone())
    .with_saved_helper_state(manager.helper_state);
    let mut map = ObjectReader::from_snapshot(&source.parsed.object_map);
    reader.i32("raw_prefix")?;
    let roots = reader.object_count("features.count")?;
    for index in 0..roots {
        reader.object(
            &format!("features[{index}]"),
            &mut map,
            ModelObjectRole::Any,
        )?;
    }
    if manager.helper_state == SavedHelperState::Active {
        mbsm(&mut reader, &mut map)?;
    }
    Ok(ParsedResolvedFeatures {
        fields: reader.finish()?,
        roots,
        final_map: map.into_snapshot(),
    })
}

fn mbsm(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.unsigned_count("mbsm.count", 6)? {
        reader.i32(&format!("mbsm[{index}].raw_key"))?;
        reader.object(&format!("mbsm[{index}].object"), map, ModelObjectRole::Any)?;
    }
    Ok(())
}

pub(super) fn construct(
    fields: Vec<ModelField>,
    source: &PartConfigurationArchive,
    manager: &ParsedConfigurationManager,
    limits: ModelFieldLimits,
) -> Result<ParsedResolvedFeatures> {
    let bytes = super::encode_fields(&fields, limits.maximum_bytes)?;
    let parsed = parse(&bytes, source, manager, limits)?;
    if parsed.fields != fields {
        return Err(super::field_mismatch::construction_mismatch(
            &fields,
            &parsed.fields,
        ));
    }
    Ok(parsed)
}
