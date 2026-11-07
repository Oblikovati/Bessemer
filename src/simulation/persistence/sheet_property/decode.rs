use super::super::{load_persist_id, load_reader::BinaryLoadReader};
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    load_persist_id::read(reader)?;
    reader.text("text38")?;
    reader.text("text40")?;
    reader.words("words48_50", 3)?;
    if reader.version >= 830004 {
        reader.signed("word54")?;
    }
    Ok(())
}
