use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::MaterialRead;

/// Preserve the three stored feature-ID words without assigning application roles.
/// Example: `manager_scalar_records::feature_ids(reader)?` for `moSMAndFPFeatIdPair`.
pub(super) fn feature_ids(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_0c", "raw_10"])
}

/// Retain material names and scalar bits without interpreting units or runtime values.
/// Example: `manager_scalar_records::material(reader)?` for the custom material record.
pub(super) fn material(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("name_08")?;
    reader.f64s(&["raw_10", "raw_18"])?;
    reader.i32("raw_20")?;
    reader.f64("raw_28")
}
