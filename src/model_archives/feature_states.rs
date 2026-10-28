use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::MaterialRead;

/// Retain both signed state words without guessing their engineering domains.
/// Example: `feature_states::read(reader)?` for the scoped native state record.
pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_10"])
}
