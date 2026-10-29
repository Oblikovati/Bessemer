use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the saved helper's selected UI-bend layout before runtime filtering.
/// Example: `ui_bends::read(reader, map)?` for a complete UI-bend feature.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if !reader.sheet_metal_extended() {
        return declared(reader, map, "bend_808", &["moOneBend_c"]);
    }
    reader.i32("raw_810")?;
    let count = super::configuration_io::positive_count(reader, "declared_line_count", 12)?;
    for index in 0..count {
        reader.scope(&format!("direct_lines_828[{index}]"), ruled_line)?;
    }
    declared(reader, map, "sketch_line_848", &["moBendSketchLineData_c"])?;
    model_lists::pointers(reader, map, "array_850", ModelObjectRole::Any)
}

/// Decode one direct or tagged ruled-line body without its runtime handle.
/// Example: `ui_bends::ruled_line(reader)?` for each positive-count record.
pub(super) fn ruled_line(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.f64("raw_10")?;
    reader.i32("raw_18")?;
    Ok(())
}

/// Preserve the sketch-line word added after archive version 0x42CB.
/// Example: `ui_bends::sketch_line(reader)?` for a complete sketch-line record.
pub(super) fn sketch_line(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_0c"])?;
    if reader.version() > 0x42cb {
        reader.i32("raw_10")?;
    }
    Ok(())
}
