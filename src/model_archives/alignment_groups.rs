use super::display_records::declared;
use super::{ModelDecoder, dimension_handles, features, model_references, sketch_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve complete ordinary alignment groups and their direct collection boundaries.
/// Example: `alignment_groups::read(reader, map)?` for `moAlignGroup_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("feature", |reader| features::read(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    handles(reader, map, "handles_370", usize::MAX)?;
    reader.i32("raw_3d8")?;
    for index in 0..reader.count_with_minimum("bytes_3b8.count", 1)? {
        reader.u8(&format!("bytes_3b8[{index}]"))?;
    }
    let flag = scalar_prefix(reader)?;
    suffix(reader, map, flag)
}

fn scalar_prefix(reader: &mut ModelDecoder<'_>) -> Result<i32> {
    reader.u32("raw_40c")?;
    reader.f64("raw_400")?;
    reader.i32("raw_3f0")?;
    if reader.version() <= 0x2f4d {
        return Ok(0);
    }
    reader.i32("raw_3f4")
}

fn handles(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    maximum: usize,
) -> Result<()> {
    let count = handle_count(reader, label, maximum)?;
    for index in 0..count {
        declared(
            reader,
            map,
            &format!("{label}.items[{index}]"),
            dimension_handles::CLASSES,
        )?;
    }
    Ok(())
}

fn handle_count(reader: &mut ModelDecoder<'_>, label: &str, maximum: usize) -> Result<usize> {
    let count = reader.object_count(&format!("{label}.count"))?;
    if count <= maximum {
        return Ok(count);
    }
    reader.checked(Err(super::invalid(format!(
        "alignment handle count {count}; expected <= {maximum} for {label}"
    ))))
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, flag: i32) -> Result<()> {
    if reader.version() < 0x2f50 {
        return Ok(());
    }
    if reader.version() < 0x2fa0 {
        for index in 0..2 {
            declared(
                reader,
                map,
                &format!("legacy_handles_450[{index}]"),
                dimension_handles::CLASSES,
            )?;
        }
        return Ok(());
    }
    modern(reader, map, flag)
}

fn modern(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, flag: i32) -> Result<()> {
    handles(reader, map, "handles_450", 2)?;
    declared(reader, map, "component_4c0", model_references::COMPONENTS)?;
    declared(reader, map, "entity_4a8", sketch_references::HANDLES)?;
    double_vector(reader, "vector_478")?;
    double_vector(reader, "vector_490")?;
    if flag != 0 {
        double_vector(reader, "point_4d8")?;
    }
    Ok(())
}
