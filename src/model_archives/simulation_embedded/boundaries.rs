use super::bounds;
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::model_archives::{
    ModelDecoder, ModelObjectRole, boundary_references, part_objects::absent_payload,
};

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    Some(match name {
        "moFaceRef_c" => face(reader, map),
        "moEdgeRef_c" => base(reader, map),
        "moVertexRef_c" => vertex(reader, map),
        _ => return None,
    })
}

fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let count = boundary_references::header_words(reader)?;
    if count > 10000 {
        return Err(crate::model_archives::invalid(format!(
            "boundary entity count {count}; expected 0..=10000"
        )));
    }
    boundary_references::header_values(reader)?;
    reader.object("vertex_reference", map, ModelObjectRole::Any)?;
    let count = reader.counted_entries("boundary entity IDs", i64::from(count), 2)?;
    for index in 0..count {
        reader.object(
            &format!("entities[{index}].representation"),
            map,
            ModelObjectRole::Any,
        )?;
    }
    reader.u32("unknown_48")?;
    suffix(reader)
}

fn suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.archive_version().value() > 0x23bb {
        let count = reader.i32("temporary_count")?;
        absent_payload(reader, "temporary boundary records", count)?;
    }
    if reader.archive_version().value() > 0x332c {
        reader.i32("unknown_ec")?;
        let present = reader.i32("has_direct_model")?;
        absent_payload(reader, "direct boundary model", present)?;
    }
    Ok(())
}

fn face(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    let count = bounds::signed_count(reader, "face_records.count", 10000, 16)?;
    for index in 0..count {
        for word in 0..4 {
            reader.u32(&format!("face_records[{index}].words[{word}]"))?;
        }
    }
    Ok(())
}

fn vertex(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    let present = reader.i32("has_point")?;
    absent_payload(reader, "embedded vertex point", present)
}
