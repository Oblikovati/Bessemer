use super::display_records::declared;
use super::{ModelDecoder, body_features};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Retain bitmap feature data and its full-width compact-transform presence word.
/// Example: `sketch_bitmaps::read(reader, map)?` for `moSketchBitmap_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("model_feature", |reader| {
        body_features::model_feature(reader, map)
    })?;
    declared(reader, map, "data_360", &["moSketchBitmapData_c"])?;
    if reader.i32("transform_presence")? != 0 {
        reader.transform("transform_378")?;
    }
    Ok(())
}

/// Preserve complete bitmap metadata and the explicitly sized raw byte plane.
/// Example: `sketch_bitmaps::payload(reader, map)?` for `moSketchBitmapData_c`.
pub(super) fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    double_vector(reader, "point_08")?;
    reader.f64s(&["raw_28", "raw_20", "raw_30"])?;
    let width = reader.i32("raw_48")?;
    let height = reader.i32("raw_4c")?;
    reader.i32("raw_50")?;
    reader.text("string_58")?;
    references(reader, map)?;
    metadata(reader)?;
    byte_plane(reader, width, height)?;
    if reader.version() >= 0x2bd3 {
        reader.i32("raw_80")?;
    }
    Ok(())
}

fn references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.u32("reference_presence")? == 0 {
        return Ok(());
    }
    declared(
        reader,
        map,
        "line_b8",
        super::geometric_references::LINE_REFERENCE_CLASSES,
    )?;
    declared(
        reader,
        map,
        "dimension_reference_c0",
        &["moDimRefWrapper_c"],
    )
}

fn metadata(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32s(&["raw_70", "raw_60", "raw_64"])?;
    reader.u32("raw_68")?;
    reader.i32("raw_6c")?;
    Ok(())
}

fn byte_plane(reader: &mut ModelDecoder<'_>, width: i32, height: i32) -> Result<()> {
    if reader.u8("plane_presence")? == 0 {
        return Ok(());
    }
    let count = plane_extent(reader, width, height)?;
    reader.entries(count, "sketch bitmap byte plane")?;
    for index in 0..count {
        reader.u8(&format!("byte_plane_40[{index}]"))?;
    }
    Ok(())
}

fn plane_extent(reader: &mut ModelDecoder<'_>, width: i32, height: i32) -> Result<usize> {
    let count = usize::try_from(width).ok().and_then(|width| {
        usize::try_from(height)
            .ok()
            .and_then(|height| width.checked_mul(height))
    });
    let Some(count) = count.filter(|count| *count <= reader.remaining()) else {
        return reader.checked(Err(super::invalid(format!(
            "sketch bitmap byte-plane extent ({width}, {height}); expected nonnegative dimensions with product <= {} remaining bytes",
            reader.remaining(),
        ))));
    };
    Ok(count)
}
