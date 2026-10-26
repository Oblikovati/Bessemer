use super::{ModelDecoder, ModelObjectRole, part_objects::absent_payload};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.u32("unknown_60")?;
    reader.u32("unknown_64")?;
    let count = reader.u32("entity_count")? as usize;
    reader.u32("unknown_e8")?;
    reader.u32("unknown_70")?;
    reader.f64s(&["unknown_28", "unknown_30"])?;
    reader.object(
        "vertex_reference",
        map,
        ModelObjectRole::Declared(&["moVertexRef_c"]),
    )?;
    entities(reader, map, count)?;
    reader.u32("unknown_48")?;
    let temporary = reader.i32("temporary_count")?;
    absent_payload(reader, "temporary contour-chooser records", temporary)?;
    modern_suffix(reader)
}

fn entities(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, count: usize) -> Result<()> {
    let maximum = reader.remaining() / 2;
    let shape = if count <= maximum {
        Ok(())
    } else {
        Err(super::invalid(format!(
            "boundary entity count {count}; expected <= {maximum} direct entity IDs"
        )))
    };
    reader.checked(shape)?;
    reader.entries(count, "boundary entity IDs")?;
    for index in 0..count {
        reader.scope(&format!("entities[{index}]"), |reader| {
            entity_id(reader, map)
        })?;
    }
    Ok(())
}

fn modern_suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() <= 0x332c {
        return Ok(());
    }
    reader.i32("unknown_ec")?;
    let present = reader.i32("has_direct_model_data")?;
    absent_payload(reader, "direct boundary model data", present)
}

pub(super) fn entity_id(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object(
        "representation",
        map,
        ModelObjectRole::Declared(&["moEntityIdRep_c"]),
    )?;
    Ok(())
}

pub(super) fn entity_representation(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("feature", map, ModelObjectRole::Any)?;
    reader.object(
        "external_feature_reference",
        map,
        ModelObjectRole::Declared(&["moFR_c"]),
    )?;
    Ok(())
}

pub(super) fn face(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    for index in 0..reader.signed_count("face_record_count", 16)? {
        reader.scope(&format!("face_records[{index}]"), |reader| {
            reader.i32("unknown_08")?;
            reader.u32("unknown_0c")?;
            reader.u32("unknown_10")?;
            reader.i32("unknown_14")?;
            Ok(())
        })?;
    }
    Ok(())
}

pub(super) fn vertex(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    if reader.i32("has_point")? != 0 {
        for index in 0..3 {
            reader.f64(&format!("point[{index}]"))?;
        }
    }
    Ok(())
}

pub(super) fn silhouette_prefix(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    base(reader, map)?;
    for index in 0..3 {
        reader.object(
            &format!("faces[{index}]"),
            map,
            ModelObjectRole::Declared(&["moFaceRef_c"]),
        )?;
    }
    reader.u32("flags_1a0")?;
    Ok(())
}

pub(super) fn silhouette_edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    silhouette_prefix(reader, map)?;
    reader.f64s(&["value_1b0", "value_1b8"])
}
