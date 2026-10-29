use super::display_records::declared;
use super::{
    ModelDecoder, ModelObjectRole, configuration_io, linear_patterns, pattern_variation,
    sketch_arrays,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Select the native advanced variation block before dimension-pattern records.
/// Example: `dimension_patterns::read(reader, map)?` for `moDimPattern_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("pattern", |reader| {
        linear_patterns::common_with_variation(reader, map, variation)
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn variation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    pattern_variation::prefix(reader, map)?;
    let count = configuration_io::positive_count(reader, "declared_index_count_f88", 4)?;
    sketch_arrays::integers(reader, "indices_f88", count)?;
    pattern_variation::originals(reader, map)
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["1460", "1468", "1478", "1480"] {
        display(reader, map, offset)?;
    }
    reader.u8("raw_14fc")?;
    for offset in ["1470", "1488"] {
        display(reader, map, offset)?;
    }
    descriptors(reader, map)
}

fn descriptors(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["1490", "1498"] {
        reader.object(&format!("object_{offset}"), map, ModelObjectRole::Any)?;
    }
    reader.i32("raw_1650")?;
    declared(reader, map, "features_14f0", &["suObList"])?;
    reader.u8("raw_15f8")?;
    reader.i32("raw_15fc")?;
    regeneration(reader, map)
}

fn regeneration(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    statuses(reader, map, "1600")?;
    statuses(reader, map, "1620")?;
    for index in 0..configuration_io::positive_count(reader, "declared_count_1648", 2)? {
        reader.object(
            &format!("references_1648[{index}]"),
            map,
            ModelObjectRole::Any,
        )?;
    }
    Ok(())
}

fn display(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offset: &str) -> Result<()> {
    declared(
        reader,
        map,
        &format!("dimension_{offset}"),
        super::display_dimension::CLASSES,
    )
}

fn statuses(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offset: &str) -> Result<()> {
    for index in
        0..configuration_io::positive_count(reader, &format!("declared_count_{offset}"), 16)?
    {
        reader.scope(&format!("statuses_{offset}[{index}]"), |reader| {
            status(reader, map)
        })?;
    }
    Ok(())
}

fn status(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["raw_00", "raw_04"])?;
    if reader.i32("raw_08")? == 0 {
        declared(reader, map, "features_10", &["suObList"])?;
    }
    for index in 0..configuration_io::positive_count(reader, "declared_feature_count_18", 12)? {
        reader.scope(&format!("features_18[{index}]"), |reader| {
            feature_status(reader, map)
        })?;
    }
    Ok(())
}

fn feature_status(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_00")?;
    if reader.i32("raw_string_flag")? == 0 {
        reader.text("string_08")?;
    }
    if reader.i32("raw_feature_flag")? == 0 {
        declared(reader, map, "features_18", &["suObList"])?;
    }
    Ok(())
}
