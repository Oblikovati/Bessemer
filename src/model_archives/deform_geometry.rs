use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::{MaterialRead, vectors::double_vector};

/// Retain the geometry body's independent stored version and fixed-width suffixes.
/// Example: `deform_geometry::read(reader)?` inside a deform feature.
pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    prefix(reader)?;
    for index in 0..15 {
        reader.i32(&format!("words_f8_134[{index}]"))?;
    }
    let version = reader.i32("words_f8_134[15]")?;
    for index in 0..10 {
        reader.f64(&format!("values_138_180[{index}]"))?;
    }
    reader.i32("raw_188")?;
    versioned(reader, version)
}

fn prefix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("raw_00")?;
    for index in 0..7 {
        double_vector(reader, &format!("triples[{index}]"))?;
    }
    reader.i32s(&["raw_c0", "raw_c4"])?;
    reader.f64s(&["raw_c8", "raw_d0"])?;
    reader.i32("raw_d8")?;
    reader.f64s(&["raw_e0", "raw_e8", "raw_f0"])?;
    Ok(())
}

fn versioned(reader: &mut ModelDecoder<'_>, version: i32) -> Result<()> {
    if version > 0 {
        reader.i32s(&["words_18c_190[0]", "words_18c_190[1]"])?;
    }
    if version > 1 {
        reader.f64s(&["values_5c0_5e8[0]", "values_5c0_5e8[1]"])?;
    }
    if version > 2 {
        super::fixed_transform_records::read(reader, "transform_5f0", "deform geometry")?;
    }
    later_versions(reader, version)
}

fn later_versions(reader: &mut ModelDecoder<'_>, version: i32) -> Result<()> {
    if version > 3 {
        reader.i32("raw_678")?;
        reader.scope("primitive_680", primitive)?;
    }
    if version >= 5 {
        reader.f64("raw_7e0")?;
    }
    if version >= 6 {
        reader.i32("raw_7e8")?;
    }
    if version >= 7 {
        reader.i32("raw_7ec")?;
    }
    Ok(())
}

fn primitive(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("raw_00")?;
    for index in 0..6 {
        reader.f64(&format!("values[{index}]"))?;
    }
    reader.i32("raw_38").map(|_| ())
}
