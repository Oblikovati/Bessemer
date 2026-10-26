use super::{ModelDecoder, ModelObjectRole, model_lists, node};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, visibility};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("node", |reader| node::lightweight(reader, map))?;
    if reader.i32("has_merged_feature_list")? != 0 {
        reader.scope("merged_feature_list", |reader| {
            model_lists::nodes(reader, map)
        })?;
    }
    reader.object(
        "visual_properties",
        map,
        ModelObjectRole::Declared(&["moVisualProperties_c"]),
    )?;
    reader.scope("feature_list", |reader| model_lists::nodes(reader, map))?;
    initial_fields(reader)?;
    property_fields(reader, map)?;
    final_fields(reader)
}

fn initial_fields(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32s(&["unknown_290", "unknown_284"])?;
    reader.u32("unknown_294")?;
    if reader.u8("has_unknown_1e0")? != 0 {
        reader.text("unknown_1e0")?;
    }
    reader.i32s(&["unknown_2a0", "unknown_2b0", "unknown_2b4", "unknown_2a8"])?;
    reader.f64("unknown_1d8")?;
    reader.i32("unknown_2ac")?;
    Ok(())
}

fn property_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object("unknown_250", map, ModelObjectRole::Any)?;
    reader.text("unknown_1f0")?;
    reader.u8("unknown_1ec")?;
    reader.scope("entity_visual_properties", visibility::read)?;
    reader.i32s(&["unknown_2e4", "unknown_a0"])?;
    reader.object("unknown_2e8", map, ModelObjectRole::Any)
}

fn final_fields(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.u32("unknown_200")?;
    reader.u8("unknown_2f8")?;
    reader.u8("unknown_2f9")?;
    reader.i32s(&["unknown_310", "unknown_hidden", "unknown_304"])?;
    reader.u32("unknown_29c")?;
    reader.u32("unknown_298")?;
    reader.u16("unknown_90")?;
    reader.i32s(&["unknown_2a4", "unknown_33c"])
}

pub(super) fn folder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    read(reader, map)?;
    reader.i32s(&["unknown_358", "unknown_368"])
}

pub(super) fn feature_group(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    notes_area: bool,
) -> Result<()> {
    folder(reader, map)?;
    reader.object("unknown_380", map, ModelObjectRole::Any)?;
    reader.i32s(&["unknown_374", "unknown_38c"])?;
    // The notes-area final vtable enables the extra word; ordinary groups omit it.
    if notes_area {
        reader.i32("unknown_370")?;
    }
    reader.i32("unknown_394")?;
    Ok(())
}

pub(super) fn model_feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("feature", |reader| read(reader, map))?;
    reader.i32("word_354")?;
    Ok(())
}
