use super::display_records::declared;
use super::{ModelDecoder, model_lists, profile_geometry};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn collection(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.u32("word_2b0")?;
    for index in 0..reader.signed_count("items.count", 2)? {
        declared(
            reader,
            map,
            &format!("items[{index}]"),
            &["moSketchChain_c"],
        )?;
    }
    Ok(())
}

pub(super) fn chain(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::unsigned_words(reader, "array_08")?;
    model_lists::unsigned_words(reader, "array_60")?;
    reader.u32("word_28")?;
    reader.i32s(&["word_38", "word_50", "word_54"])?;
    reader.u32("word_58")?;
    reader.u32("word_5c")?;
    reader.text("string_80")?;
    handles(reader, map)?;
    reader.u32("word_d0")?;
    let count = reader.unsigned_count("pairs.count", 8)?;
    profile_geometry::pairs(reader, count)?;
    declared(reader, map, "trim_c0", &["contourTrimData_c"])?;
    declared(reader, map, "trim_c8", &["contourTrimData_c"])?;
    reader.i32s(&["word_90", "word_a0", "word_a4"])
}

fn handles(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let mask = reader.u16("mask")?;
    if mask > 3 {
        return reader.checked(Err(super::invalid(format!(
            "sketch-chain handle mask {mask}; expected 0..=3"
        ))));
    }
    if mask & 1 != 0 {
        declared(reader, map, "plane", &["sg3DPlaneHandle"])?;
    }
    if mask & 2 != 0 {
        for index in 0..2 {
            declared(reader, map, &format!("points[{index}]"), &["sgPointHandle"])?;
        }
    }
    Ok(())
}
