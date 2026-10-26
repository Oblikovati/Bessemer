use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, material};

pub(super) fn property(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("unknown_10")?;
    reader.scope("visual_properties", material::visual)
}

pub(super) fn face_appearances(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("face_count", 4)? {
        reader.scope(&format!("faces[{index}]"), |reader| {
            reader.scope("property", property)?;
            reader.object("face", map, ModelObjectRole::Declared(&["moCompFace_c"]))?;
            reader.i32s(&["unknown_28", "unknown_1c", "unknown_18"])
        })?;
    }
    properties(reader)
}

pub(super) fn properties(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("property_count", 4)? {
        reader.scope(&format!("properties[{index}]"), property)?;
    }
    Ok(())
}

pub(super) fn spatial(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.u8("unknown_d28")?;
    reader.object(
        "curvature",
        map,
        ModelObjectRole::Declared(&["gcCurvatureObject_c"]),
    )?;
    reader.object(
        "reference_plane",
        map,
        ModelObjectRole::Declared(&["moTransRefPlaneData_c"]),
    )?;
    box_fields(reader)?;
    reader.i32("is_fastener")?;
    Ok(())
}

fn box_fields(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.u8("has_box")? != 0 {
        for index in 0..10 {
            reader.f64(&format!("box[{index}]"))?;
        }
    }
    Ok(())
}

pub(super) fn metadata(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("visual_count", 6)? {
        reader.scope(&format!("visual_properties[{index}]"), |reader| {
            reader.i32("unknown_key")?;
            reader.object(
                "visual_properties",
                map,
                ModelObjectRole::Declared(&["moVisualProperties_c"]),
            )
        })?;
    }
    reader.i32s(&["unknown_f10", "is_toolbox_part", "unknown_f24"])?;
    details(reader, map)?;
    reader.f64s(&["unknown_f78", "unknown_f80"])
}

fn details(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("detail_count", 3)? {
        reader.scope(&format!("detail_definitions[{index}]"), |reader| {
            reader.text("name")?;
            reader.object(
                "definition",
                map,
                ModelObjectRole::Declared(&["moDetailDefs_c"]),
            )
        })?;
    }
    Ok(())
}

pub(super) fn manager(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    if reader.i32("format")? != 2 {
        return Ok(());
    }
    for index in 0..reader.count_with_minimum("entry_count", 3)? {
        reader.scope(&format!("entries[{index}]"), |reader| {
            reader.text("name")?;
            reader.object("value", map, ModelObjectRole::Declared(BOUNDARY_CLASSES))
        })?;
    }
    reader.i32s(&["unknown_110", "unknown_114"])
}

const BOUNDARY_CLASSES: &[&str] = &["moBEntRef_c", "moEdgeRef_c", "moFaceRef_c", "moVertexRef_c"];
