use super::{ModelDecoder, ModelObjectRole, features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, material};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.u8("raw_d24_stored")?;
    reader.i32s(&["raw_ce0", "raw_ce4"])?;
    reader.object(
        "feature_colors",
        map,
        ModelObjectRole::Declared(&["moFeatColorTab_c"]),
    )?;
    reader.object("discarded_attachment", map, ModelObjectRole::Any)?;
    reader.i32("raw_d3c")?;
    if reader.i32("node_list_presence")? != 0 {
        reader.scope("node_list", |reader| model_lists::nodes(reader, map))?;
    }
    reader.i32("raw_d40")?;
    block_and_color_tables(reader, map)
}

fn block_and_color_tables(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object(
        "sketch_block_manager",
        map,
        ModelObjectRole::Declared(&["moSketchBlockMgr_c"]),
    )?;
    reader.object("list_a40", map, ModelObjectRole::Declared(&["suObList"]))?;
    reader.i32s(&["raw_ea8", "raw_eac"])?;
    keys(reader, "keys_eb8")?;
    reader.object(
        "flat_pattern_colors",
        map,
        ModelObjectRole::Declared(&["moFlatPatternColorTab_c"]),
    )?;
    reader.u32("raw_f34")?;
    if reader.version() > 0x2baf {
        keys(reader, "keys_1280")?;
    }
    Ok(())
}

fn keys(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 4)? {
        reader.u32(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn feature_colors(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("entry_count", 4)? {
        reader.scope(&format!("entries[{index}]"), |reader| {
            reader.scope("visual", material::visual)?;
            reader.i32("raw_associated_value")?;
            Ok(())
        })?;
    }
    Ok(())
}

pub(super) fn flat_colors(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    for offset in (8..0x28).step_by(4) {
        reader.u32(&format!("raw_{offset:x}"))?;
    }
    Ok(())
}

pub(super) fn sketch_blocks(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("feature", |reader| features::read(reader, map))?;
    reader.fixed_octets("block_guid", 16)?;
    reader.scope("block_map", |reader| block_map(reader, map))
}

fn block_map(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("next_id")?;
    reader.object(
        "owner",
        map,
        ModelObjectRole::Declared(&["moSketchBlockMgr_c"]),
    )?;
    for index in 0..reader.signed_count("group_count", 8)? {
        reader.scope(&format!("groups[{index}]"), group)?;
    }
    for index in 0..reader.signed_count("record_count", 37)? {
        reader.scope(&format!("records[{index}]"), block_record)?;
    }
    Ok(())
}

fn group(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("key")?;
    for index in 0..reader.signed_count("guid_count", 16)? {
        reader.fixed_octets(&format!("guids[{index}]"), 16)?;
    }
    Ok(())
}

fn block_record(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.fixed_octets("key_guid", 16)?;
    reader.fixed_octets("value_guid", 16)?;
    reader.i32("feature_id")?;
    reader.text("feature_name")
}
