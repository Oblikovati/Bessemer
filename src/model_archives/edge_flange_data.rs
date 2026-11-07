use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists, model_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the complete direct or tagged feature-data body with a nonnull owner.
/// Example: `edge_flange_data::read(reader, map)?` after the bend-group prefix.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.required_object(
        "owner_08",
        map,
        ModelObjectRole::Declared(super::body_features::CLASSES),
    )?;
    if !reader.sheet_metal_extended() {
        declared(
            reader,
            map,
            "sheet_metal_18",
            &["moSheetMetal_c", "moTemplateSheetMetal_c"],
        )?;
    }
    prefix(reader, map)?;
    middle(reader, map)?;
    suffix(reader, map)
}

fn dimension(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offset: &str) -> Result<()> {
    declared(
        reader,
        map,
        &format!("dimension_{offset}"),
        &["moDisplayDistanceDim_c"],
    )
}

fn reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offset: &str) -> Result<()> {
    declared(
        reader,
        map,
        &format!("reference_{offset}"),
        model_references::WRAPPER_CLASSES,
    )
}

fn point(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offset: &str) -> Result<()> {
    declared(
        reader,
        map,
        &format!("point_{offset}"),
        super::geometric_references::POINT_REFERENCE_CLASSES,
    )
}

fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    dimension(reader, map, "b8")?;
    reader.i32("raw_c0")?;
    reader.f64s(&["raw_28", "raw_38", "raw_40", "raw_48"])?;
    reader.i32s(&["raw_64", "raw_6c", "raw_70", "raw_74", "raw_7c"])?;
    reference(reader, map, "88")?;
    point(reader, map, "98")?;
    reader.i32("raw_80")?;
    reference(reader, map, "90")?;
    point(reader, map, "a0")?;
    reader.i32s(&["raw_ac", "raw_a8", "raw_c4", "raw_c8", "raw_68"])
}

fn middle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    dimension(reader, map, "138")?;
    reference(reader, map, "140")?;
    reader.i32s(&["raw_148", "raw_60", "raw_50"])?;
    if reader.version() > 0x2f5f {
        reader.i32("raw_78")?;
    }
    model_lists::pointers(reader, map, "array_170", ModelObjectRole::Any)?;
    dimension(reader, map, "198")?;
    reader.i32s(&["raw_168", "raw_58", "raw_54", "raw_204"])?;
    if reader.version() > 0x3356 {
        reader.i32("raw_240")?;
    }
    Ok(())
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference(reader, map, "d0")?;
    reader.i32s(&["raw_d8", "raw_e8"])?;
    reader.f64("raw_e0")?;
    reader.i32s(&["raw_190", "raw_268"])?;
    if reader.version() > 0x42cb {
        reader.i32("raw_26c")?;
    }
    Ok(())
}
