use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Read a strict signed pointer count; e.g. `pointers(reader, map, "entities", role)?`.
pub(super) fn pointers(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    role: ModelObjectRole<'_>,
) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 2)? {
        reader.object(&format!("{label}[{index}]"), map, role)?;
    }
    Ok(())
}

/// Keep ordered integer/string maps; e.g. `strings(reader, "map_8b8", true)?`.
pub(super) fn strings(reader: &mut ModelDecoder<'_>, label: &str, integer_key: bool) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 5)? {
        reader.scope(&format!("{label}[{index}]"), |reader| {
            string_entry(reader, integer_key)
        })?;
    }
    Ok(())
}

fn string_entry(reader: &mut ModelDecoder<'_>, integer_key: bool) -> Result<()> {
    if integer_key {
        reader.i32("key")?;
        reader.text("value")?;
        return Ok(());
    }
    reader.text("key")?;
    reader.i32("value").map(|_| ())
}
