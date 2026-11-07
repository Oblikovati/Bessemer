use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| {
        reader.scope("feature", |reader| {
            super::features::without_children(reader, map)
        })?;
        reader.i32("word_354")?;
        Ok(())
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

pub(super) fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "parameters", ModelObjectRole::Any)?;
    reader.object("reference_380", map, ModelObjectRole::Any)?;
    reader.object(
        "definition_388",
        map,
        ModelObjectRole::Declared(&["moAttributeDef_c"]),
    )?;
    reader.object("reference_390", map, ModelObjectRole::Any)?;
    reader.i32("raw_3a4")?;
    Ok(())
}

pub(super) fn parameter(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("node", |reader| super::node::lightweight(reader, map))?;
    reader.object("name_40", map, ModelObjectRole::Declared(&["moNodeName_c"]))
}

pub(super) fn scalar_parameter(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    parameter(reader, map)?;
    reader.f64("raw_value_50")?;
    if reader.u8("override_presence_60")? != 0 {
        reader.f64("raw_override_58")?;
    }
    Ok(())
}

pub(super) fn string_parameter(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    parameter(reader, map)?;
    reader.text("raw_value_50")
}
