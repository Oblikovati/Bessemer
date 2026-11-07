use super::{ModelDecoder, ModelObjectRole, dimensions};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve the independent attachment-data body and its actual view pointer.
/// Example: `attachment_records::data(reader, map)?` for the exact data class.
pub(super) fn data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    double_vector(reader, "point_08")?;
    double_vector(reader, "point_20")?;
    reader.object("view_38", map, ModelObjectRole::Declared(&["moDrView"]))?;
    reader.f64s(&["raw_40", "raw_48"])
}

/// Reuse the complete ordinary part attachment before its parametric suffix.
/// Example: `attachment_records::parametric(reader, map)?` for the concrete subtype.
pub(super) fn parametric(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("attachment", |reader| dimensions::attachment(reader, map))?;
    reader.f64s(&["double_108", "double_110"])?;
    reader.object("reference_118", map, ModelObjectRole::Any)?;
    reader.u16("word_124")?;
    reader.i32("word_130")?;
    Ok(())
}
