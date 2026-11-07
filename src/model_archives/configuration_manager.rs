use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldLimits, ModelObjectRole, configuration_io as io};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::header::ArchiveVersion;
use crate::{BessemerError, FailureClass, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SavedHelperState {
    Inactive,
    Active,
}

#[derive(Debug, Clone)]
pub(super) struct ParsedConfigurationManager {
    pub fields: Vec<ModelField>,
    pub helper_state: SavedHelperState,
    pub version: ArchiveVersion,
    pub dialect: super::manager_dialect::ManagerDialect,
}

pub(super) fn parse(
    bytes: &[u8],
    version: ArchiveVersion,
    limits: ModelFieldLimits,
) -> Result<ParsedConfigurationManager> {
    let dialect = super::manager_dialect::ManagerDialect::from_version(version)?;
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        version,
        limits,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?
    .with_manager_dialect(dialect);
    require_manager_scope(&mut reader)?;
    let mut map = ObjectReader::new();
    let root = reader.configuration_scene(|reader| {
        reader.object_record(
            "manager",
            &mut map,
            ModelObjectRole::Declared(&["moConfigurationMgr_c"]),
        )
    })?;
    let fields = reader.finish()?;
    super::configuration_graph::validate(&fields, &root)?;
    let helper_state =
        match super::object_values::stored_object_integer(&fields, &root, "persisted_state_650")? {
            0 => SavedHelperState::Inactive,
            1 => SavedHelperState::Active,
            value => {
                return Err(super::invalid(format!(
                    "saved helper state {value}; expected 0 or 1"
                )));
            }
        };
    Ok(ParsedConfigurationManager {
        fields,
        helper_state,
        version,
        dialect,
    })
}

pub(super) fn require_current(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.archive_version().value() == 19000 {
        return reader.require_ordinary_part();
    }
    reader.checked(Err(BessemerError::new(
        "configuration context",
        "Contents/CMgr",
        FailureClass::Unsupported,
        format!(
            "configuration version {}; expected scoped current version 19000",
            reader.archive_version().value()
        ),
    )))
}

pub(super) fn require_manager_scope(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let Some(dialect) = reader.manager_dialect() else {
        return require_current(reader);
    };
    if reader.archive_version().value() == dialect.version() {
        return reader.require_ordinary_part();
    }
    reader.checked(Err(BessemerError::new(
        "configuration context",
        "Contents/CMgr",
        FailureClass::Unsupported,
        format!(
            "configuration version {} with dialect {dialect:?}; expected exact version {}",
            reader.archive_version().value(),
            dialect.version()
        ),
    )))
}

pub(super) fn body(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    require_manager_scope(reader)?;
    // #54: CMgr's factory leaves its concept null, so child-list bytes are absent.
    reader.scope("feature", |reader| {
        super::features::without_children(reader, map)
    })?;
    for label in ["configuration_358", "configuration_360"] {
        reader.object(
            label,
            map,
            ModelObjectRole::Declared(&["moPartConfiguration_c"]),
        )?;
    }
    initial_fields(reader, map)?;
    super::manager_tail::read(reader, map)
}

fn initial_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_404")?;
    io::list(reader, map, "configurations")?;
    io::pointers(
        reader,
        map,
        "external_models",
        ModelObjectRole::Declared(&["moExtObject_c"]),
    )?;
    reader.i32("raw_554")?;
    reader.text("string_570")?;
    io::empty_signed(reader, "map_588.count", 4)?;
    reader.object("list_5a8", map, ModelObjectRole::Declared(&["suObList"]))?;
    for index in 0..io::positive_count(reader, "map_5b0.count", 12)? {
        reader.i32(&format!("map_5b0[{index}].key"))?;
        reader.u32(&format!("map_5b0[{index}].first"))?;
        reader.u32(&format!("map_5b0[{index}].second"))?;
    }
    Ok(())
}
