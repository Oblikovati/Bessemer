use super::{ModelDecoder, ModelFieldValue, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve extension order, duplicate tags and the first unrecognized terminal word.
/// Example: `punch_extensions::read(reader, map)?` before a legacy direct keeper.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.remaining() / 4 {
        let tag = reader.scalar(|cursor| cursor.i32())?;
        if !matches!(tag, 0x3d2 | 0x13e | 0x1466) {
            return reader.field("terminal_tag", ModelFieldValue::I32(tag));
        }
        reader.entries(1, "punch extensions")?;
        reader.scope(&format!("extensions[{index}]"), |reader| {
            entry(reader, map, tag)
        })?;
    }
    reader.checked(Err(super::invalid(format!(
        "punch extension sequence with {} bytes remaining; expected a terminal i32 tag",
        reader.remaining(),
    ))))
}

fn entry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, tag: i32) -> Result<()> {
    reader.field("tag", ModelFieldValue::I32(tag))?;
    match tag {
        0x3d2 => reader.i32("raw_value").map(|_| ()),
        0x13e => reader.f64("raw_value"),
        _ => reader.object("object", map, ModelObjectRole::Any),
    }
}
