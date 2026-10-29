use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, linear_patterns};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("increments.count", 41)? {
        reader.scope(&format!("increments[{index}]"), |reader| {
            increment(reader, map)
        })?;
    }
    reader.u8("byte_9fc")?;
    reader.u8("byte_a00")?;
    originals(reader, map)
}

/// Keep the shared original-dimension collections in their stored order.
/// Example: `originals(reader, map)?` after variation-specific indices.
pub(super) fn originals(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["original_a08", "original_a28"] {
        for index in 0..reader.signed_count(&format!("{label}.count"), 81)? {
            reader.scope(&format!("{label}[{index}]"), |reader| original(reader, map))?;
        }
    }
    for label in ["original_a48", "original_cb0"] {
        reader.scope(label, |reader| original(reader, map))?;
    }
    Ok(())
}

fn named_handle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.u8("handle_missing")?;
    reader.object("handle", map, ModelObjectRole::Any)?;
    if reader.u8("has_name")? != 0 {
        reader.text("name")?;
    }
    Ok(())
}

pub(super) fn increment(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    named_handle(reader, map)?;
    reader.i32s(&["word_18", "word_1c", "word_20"])?;
    reader.u8("dimension_missing")?;
    reader.object("dimension", map, ModelObjectRole::Any)?;
    reader.u8("byte_24")?;
    reader.f64("value_28")?;
    reader.i32("word_50")?;
    reader.u8("byte_30")?;
    reader.f64("value_38")?;
    Ok(())
}

pub(super) fn original(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    named_handle(reader, map)?;
    reader.u8("byte_18")?;
    reader.f64("value_20")?;
    reader.u8("byte_b4")?;
    reader.f64s(&["value_b8", "value_c0"])?;
    reader.i32("word_260")?;
    for offset in ["c8", "cc", "d0", "d4", "1d8", "1d0", "1d4", "218"] {
        reader.i32(&format!("word_{offset}"))?;
    }
    if reader.u8("has_increment")? != 0 {
        reader.scope("increment", |reader| increment(reader, map))?;
    }
    original_tail(reader, map)
}

fn original_tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("component_features.count", 2)? {
        declared(
            reader,
            map,
            &format!("component_features[{index}]"),
            super::component_references::FEATURE_CLASSES,
        )?;
    }
    reader.i32("word_1c0")?;
    linear_patterns::array(reader, map, "array_1b0")?;
    reader.i32("word_25c")?;
    Ok(())
}
