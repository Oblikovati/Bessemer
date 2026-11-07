use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("name_08")?;
    reader.i32("raw_10")?;
    Ok(())
}
