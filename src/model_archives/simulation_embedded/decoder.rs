use super::{chains, components, unsupported, wrappers};
use crate::archive_codec::{
    ArchiveClass, ArchiveCursor, MaterialRead, ObjectMapSnapshot, ObjectReader,
};
use crate::header::ArchiveVersion;
use crate::model_archives::{ModelDecoder, ModelField, ModelFieldLimits, ModelObjectRole};
use crate::{Result, model_archives::reader::ModelArchiveContext};

/// Explicit archive family selected independently of the outer Simulation version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationEmbeddedProfile {
    /// Modern embedded archive version words from 4307 through 19000.
    Modern,
    /// The distinct version-4281 reader with its historical class bodies.
    Legacy4281,
}

pub(super) fn parse(
    bytes: &[u8],
    profile: SimulationEmbeddedProfile,
    limits: ModelFieldLimits,
) -> Result<(Vec<ModelField>, ObjectMapSnapshot)> {
    let (reader, map) = read_root(bytes, profile, limits)?;
    Ok((reader.finish()?, map.into_snapshot()))
}

pub(super) fn parse_prefix(
    bytes: &[u8],
    profile: SimulationEmbeddedProfile,
    limits: ModelFieldLimits,
) -> Result<(Vec<ModelField>, ObjectMapSnapshot, usize)> {
    let bounded = &bytes[..bytes.len().min(limits.maximum_bytes)];
    let (reader, map) = read_root(bounded, profile, limits)?;
    let consumed = bounded.len() - reader.remaining();
    Ok((
        reader.finish_embedded_prefix()?,
        map.into_snapshot(),
        consumed,
    ))
}

fn read_root(
    bytes: &[u8],
    profile: SimulationEmbeddedProfile,
    limits: ModelFieldLimits,
) -> Result<(ModelDecoder<'_>, ObjectReader)> {
    let version = ArchiveCursor::new(bytes).u32()?;
    require_profile(version, profile)?;
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        limits,
        ModelArchiveContext::SimulationEmbedded,
    )?;
    reader.u32("archive_version")?;
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    Ok((reader, map))
}

fn require_profile(version: u32, profile: SimulationEmbeddedProfile) -> Result<()> {
    let accepted = match profile {
        SimulationEmbeddedProfile::Modern => (4307..=19000).contains(&version),
        SimulationEmbeddedProfile::Legacy4281 => version == 4281,
    };
    if accepted {
        return Ok(());
    }
    Err(unsupported(
        "archive version",
        format!(
            "version {version} profile {profile:?}; expected Modern 4307..=19000 or Legacy4281 exactly 4281",
        ),
    ))
}

pub(in crate::model_archives) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    if let Some(result) = super::reference_wrappers::read_object(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::component_boundaries::read_object(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::boundaries::read_object(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::representations::read_object(reader, map, &class.name) {
        return result;
    }
    if let Some(result) = super::sheet_components::read_object(reader, map, &class.name) {
        return result;
    }
    read_common_object(reader, map, class)
}

fn read_common_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &ArchiveClass,
) -> Result<()> {
    match class.name.as_str() {
        "moCompXFormWrapper_c" => wrappers::component_transform(reader, map),
        "moCompHolder_c" => components::holder(reader, map),
        "moUnitComponent_c" => components::unit(reader, map),
        "moChainComponent_c" => chains::component(reader, map),
        "moReferenceChain_c" => chains::reference(reader, map),
        "moExtFeatureList_c" => chains::feature_list(reader, map),
        "moExtFeature_c" => chains::feature(reader, map),
        "moExtObject_c" => crate::model_archives::external_references::stored_fields(reader, map),
        "moCStringHandle_c" => reader.text("value"),
        name => Err(unsupported(
            name,
            format!(
                "class {name:?} schema {} version {}; expected a declared Simulation embedded reader",
                class.schema,
                reader.archive_version().value(),
            ),
        )),
    }
}
