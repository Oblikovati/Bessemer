use super::display_records::declared;
use super::geometric_references::SURFACE_CLASSES;
use super::{ModelDecoder, linear_patterns, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn plane(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.i32("presence")? != 0 {
        double_vector(reader, "point")?;
        double_vector(reader, "normal")?;
    }
    Ok(())
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "face_800", &["moCompFace_c"])?;
    reader.i32("raw_808")?;
    reader.scope("plane_810", plane)?;
    linear_patterns::array(reader, map, "array_820")?;
    linear_patterns::array(reader, map, "array_840")?;
    reader.i32("raw_9b8")?;
    declared(reader, map, "surface_9a0", SURFACE_CLASSES)?;
    declared(reader, map, "matrix_helper_9c8", &["moMatrixIndexHelper_c"])?;
    reader.u8("raw_9b4")?;
    reader.i32("raw_9bc")?;
    for label in ["raw_9f0", "raw_9c0", "raw_9c4"] {
        reader.u8(label)?;
    }
    continuation(reader, map)
}

fn continuation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x2f86 {
        linear_patterns::array(reader, map, "list_880")?;
    }
    if reader.version() > 0x2f93 {
        let count = reader.unsigned_count("words_8c8.count", 4)?;
        sketch_arrays::integers(reader, "words_8c8", count)?;
    }
    if reader.version() > 0x3745 {
        reader.u8("raw_9b0")?;
        declared(reader, map, "surface_9a8", SURFACE_CLASSES)?;
    }
    if reader.version() > 0x3760 {
        reader.scope("plane_818", plane)?;
    }
    Ok(())
}

pub(super) fn unsigned_array(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    let count = reader.signed_count("count", 4)?;
    reader.i32("growth")?;
    for index in 0..count {
        reader.u32(&format!("items[{index}]"))?;
    }
    Ok(())
}
