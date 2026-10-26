use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn unsigned_words(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..reader.count_with_minimum(&format!("{label}.count"), 4)? {
        reader.u32(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn pointers(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    role: ModelObjectRole<'_>,
) -> Result<()> {
    for index in 0..reader.object_count(&format!("{label}.count"))? {
        reader.object(&format!("{label}[{index}]"), map, role)?;
    }
    Ok(())
}

pub(super) fn nodes(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    pointers(reader, map, "nodes", ModelObjectRole::Any)?;
    reader.i32("unknown_tail")?;
    Ok(())
}
