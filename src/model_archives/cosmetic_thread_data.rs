use super::display_records::declared;
use super::{ModelDecoder, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["word_08", "word_0c", "word_10"])?;
    reader.u8("byte_14")?;
    reader.f64s(&["value_18", "value_20", "value_28"])?;
    reader.i32("word_30")?;
    reader.f64("value_38")?;
    sketch_arrays::vectors(reader, "points", 3)?;
    sketch_arrays::doubles(reader, "cone_values", 8)?;
    reader.i32s(&["word_100", "word_180"])?;
    sketch_arrays::doubles(reader, "values_48_58_50_60", 4)?;
    reader.i32("word_108")?;
    instances(reader, map)?;
    caches(reader, map)?;
    values(reader)?;
    versioned(reader)
}

fn instances(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let count = reader.signed_count("instance_count", 8)?;
    reader.entries(count, "thread instance IDs")?;
    reader.entries(count, "thread instance words")?;
    for index in 0..count {
        reader.transform(&format!("transforms[{index}]"))?;
    }
    for index in 0..count {
        declared(
            reader,
            map,
            &format!("instance_ids[{index}]"),
            &["moCThreadInstanceId_c"],
        )?;
    }
    sketch_arrays::integers(reader, "instance_words", count)?;
    if count != 0 {
        double_vector(reader, "vector_178")?;
    }
    Ok(())
}

fn caches(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("caches.count", 2)? {
        declared(
            reader,
            map,
            &format!("caches[{index}]"),
            &["moCThreadCacheData_c"],
        )?;
    }
    Ok(())
}

fn values(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let word = reader.i32("word_10c")?;
    reader.f64("value_1c8")?;
    if word != 0 {
        sketch_arrays::doubles(reader, "values_1d0_1d8", 2)?;
    }
    if reader.u8("has_point_120")? == 1 {
        double_vector(reader, "point_120")?;
    }
    reader.f64("value_128")?;
    for (string, word) in [("190", "1a0"), ("1a8", "1b0"), ("1b8", "1c0")] {
        reader.text(&format!("text_{string}"))?;
        reader.i32(&format!("word_{word}"))?;
    }
    Ok(())
}

fn versioned(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x2b6a {
        reader.text("text_198")?;
    }
    if reader.version() > 0x3366 {
        reader.f64("value_2d0")?;
    }
    if reader.version() > 0x3379 {
        double_vector(reader, "point_80")?;
    }
    if reader.version() < 0x3749 {
        return Ok(());
    }
    if reader.i32("word_118")? != 0 {
        sketch_arrays::doubles(reader, "values_1d0_1d8_final", 2)?;
    }
    Ok(())
}
