use super::ModelDecoder;
use super::display_records::declared;
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Retain the stored curve points without resolving or opening the stored filename.
/// Example: `curve_files::read(reader, map)?` for `moCurveInFile_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("reference_curve", |reader| {
        super::helix_records::reference_curve(reader, map)
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.text("stored_string_780")?;
    let count = super::configuration_io::positive_count(reader, "declared_count", 24)?;
    for index in 0..count {
        double_vector(reader, &format!("points[{index}]"))?;
    }
    if reader.version() > 0x42cd {
        declared(
            reader,
            map,
            "reference_798",
            super::geometric_references::POINT_REFERENCE_CLASSES,
        )?;
    }
    Ok(())
}
