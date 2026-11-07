use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn list(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count(&format!("{label}.count"), 24)? {
        reader.scope(&format!("{label}[{index}]"), record)?;
    }
    Ok(())
}

pub(super) fn record(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["string_08", "string_10", "string_18"] {
        reader.text(label)?;
    }
    reader.i32("raw_2c")?;
    reader.u32("raw_28")?;
    reader.text("string_20")?;
    reader.i32s(&["raw_34", "raw_38", "raw_3c"])
}
