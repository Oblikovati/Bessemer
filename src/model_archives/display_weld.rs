use super::{ModelDecoder, ModelObjectRole, display_item, display_object};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve the complete ordinary display-item base and its weld suffix.
/// Example: `display_weld::read(reader, map)?` at the exact class dispatch.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    display_object::read(reader, map)?;
    reader.scope("item", |reader| display_item::read(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

// #54 / C-387: modern gates hold; runtime repairs and callbacks add no input.
fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    double_vector(reader, "vector_5a0")?;
    reader.f64("raw_5a8")?;
    reader.i32("raw_5d8")?;
    reader.text("string_5b8")?;
    reader.object("object_5c0", map, ModelObjectRole::Any)
}
