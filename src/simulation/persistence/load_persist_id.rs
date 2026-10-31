use super::load_reader::BinaryLoadReader;
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.words("persistid.words", 2)?;
    let declared = reader.signed("persistid.blob_count")?;
    let count = reader.available_count(declared, "persistid.blob", 1000000, 1)?;
    reader.blob("persistid.blob", count)?;
    if reader.version >= 840002 {
        reader.embedded("persistid.archive0")?;
        reader.embedded("persistid.archive1")?;
    }
    Ok(())
}
