use super::{ModelDecoder, ModelObjectRole, features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn per_body_sheet_metal(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    let feature_classes = super::component_references::FEATURE_CLASSES;
    for (label, classes) in [
        ("component_feature_08", feature_classes),
        ("solid_body", &["moCompSolidBody_c"][..]),
        ("component_feature_18", feature_classes),
        ("component_feature_20", feature_classes),
    ] {
        reader.object(label, map, ModelObjectRole::Declared(classes))?;
    }
    reader.i32s(&["raw_28", "raw_2c"])?;
    Ok(())
}

pub(super) fn permanent_mark(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.i32("permanent_mark_id")?;
    Ok(())
}

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    node_flags: u32,
) -> Result<()> {
    reader.require_ordinary_part()?;
    shared_prefix(reader, map)?;
    if reader.i32("raw_100c")? != 0 {
        reader.scope("sheet_metal_master", |reader| {
            sheet_metal_master(reader, map)
        })?;
    }
    id_map(reader, node_flags)?;
    increment_lists(reader, map)
}

/// Read the assembly reference tail without the part-only sheet-metal stages.
/// Example: `reference_tail::assembly(reader, map, original_flags)?`.
pub(super) fn assembly(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    node_flags: u32,
) -> Result<()> {
    reader.require_ordinary_model()?;
    shared_prefix(reader, map)?;
    id_map(reader, node_flags)
}

fn shared_prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("forwarded_references", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })?;
    reader.object(
        "permanent_marks",
        map,
        ModelObjectRole::Declared(&["moPMarkRecord_c"]),
    )?;
    reader.i32("raw_1008")?;
    atom_map(reader, map)
}

fn atom_map(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("atom_map_count", 6)? {
        reader.scope(&format!("atom_map[{index}]"), |reader| {
            reader.object("atom", map, ModelObjectRole::Declared(&["moAtom_c"]))?;
            reader.i32("raw_value")?;
            Ok(())
        })?;
    }
    Ok(())
}

fn increment_lists(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_10fc")?;
    increments(reader, map, "increments_1100")?;
    reader.i32("raw_1120")?;
    increments(reader, map, "increments_1128")
}

fn sheet_metal_master(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("feature", |reader| features::read(reader, map))?;
    for index in 0..reader.signed_count("member_count", 2)? {
        reader.object(&format!("members[{index}]"), map, ModelObjectRole::Any)?;
    }
    Ok(())
}

fn id_map(reader: &mut ModelDecoder<'_>, flags: u32) -> Result<()> {
    let width = if flags & 0x100 != 0 { 4 } else { 3 };
    for row in 0..reader.signed_count("id_map.count", width * 4)? {
        for column in 0..width {
            reader.u32(&format!("id_map.rows[{row}][{column}]"))?;
        }
    }
    Ok(())
}

fn increments(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 41)? {
        reader.scope(&format!("{label}[{index}]"), |reader| {
            increment(reader, map)
        })?;
    }
    Ok(())
}

pub(super) fn increment(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.u8("handle_absent")?;
    reader.object(
        "handle",
        map,
        ModelObjectRole::Declared(super::dimension_handles::CLASSES),
    )?;
    if reader.u8("has_name")? != 0 {
        reader.text("name")?;
    }
    reader.i32s(&["raw_18", "raw_1c", "raw_20"])?;
    reader.u8("display_absent")?;
    reader.object(
        "display_dimension",
        map,
        ModelObjectRole::Declared(super::display_dimension::CLASSES),
    )?;
    reader.u8("raw_24")?;
    reader.f64("raw_28")?;
    reader.i32("raw_50")?;
    reader.u8("raw_30")?;
    reader.f64("raw_38")
}

pub(super) fn forwarded_feature(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.i32("feature_id")?;
    Ok(())
}
