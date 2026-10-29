use super::{ModelDecoder, configuration_io, views};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

/// Preserve the complete base view before the modern direct annotation suffix.
/// Example: `three_dimensional_views::read(reader, map)?` for `mo3dView_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    if reader.version() < 0x3b07 {
        return reader.checked(Err(BessemerError::new(
            "model archive",
            "3D view",
            FailureClass::Unsupported,
            format!(
                "3D-view version {}; expected >= 0x3B07 in ordinary modern scope",
                reader.version()
            ),
        )));
    }
    reader.scope("base", |reader| views::view(reader, map))?;
    suffix(reader)
}

fn suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32s(&["raw_3f8", "raw_3fc_first", "raw_3fc_second", "raw_400"])?;
    reader.text("string_408")?;
    reader.i32s(&["raw_410", "raw_418"])?;
    reader.u32("raw_450_0")?;
    reader.u32("raw_450_1")?;
    reader.i32s(&["raw_468", "raw_46c"])?;
    vector(reader)?;
    reader.f64s(&["raw_478", "raw_480"])?;
    annotation_map(reader)?;
    reader.u32("raw_498").map(|_| ())
}

fn vector(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = configuration_io::positive_count(reader, "vector_420.declared_count", 4)?;
    for index in 0..count {
        reader.i32(&format!("vector_420.values[{index}]"))?;
    }
    Ok(())
}

fn annotation_map(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..high_low_count(reader, "map_488", 5)? {
        reader.scope(&format!("map_488.entries[{index}]"), |reader| {
            reader.text("key")?;
            reader.scope("data", annotation_data)
        })?;
    }
    Ok(())
}

fn annotation_data(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = configuration_io::positive_count(reader, "declared_count", 12)?;
    for index in 0..count {
        reader.i32(&format!("keys[{index}]"))?;
    }
    for index in 0..count {
        reader.scope(&format!("groups[{index}]"), |reader| {
            for item in 0..high_low_count(reader, "items", 16)? {
                reader.scope(&format!("items[{item}]"), display_item)?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

fn display_item(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32s(&["raw_08", "raw_0c", "raw_10", "raw_14"])
}

fn high_low_count(reader: &mut ModelDecoder<'_>, label: &str, minimum: usize) -> Result<usize> {
    let high = reader.u32(&format!("{label}.count_high"))?;
    let low = reader.u32(&format!("{label}.count_low"))?;
    let count = (u64::from(high) << 32) | u64::from(low);
    let capacity = (reader.remaining() / minimum) as u64;
    if count > capacity {
        return reader.checked(Err(super::invalid(format!(
            "{label} count {count}; expected <= {capacity} entries of at least {minimum} bytes"
        ))));
    }
    let count = count as usize;
    reader.entries(count, label)?;
    Ok(count)
}
