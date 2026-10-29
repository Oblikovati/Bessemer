use super::super::{ModelObjectRole, lightweight, node, part_archive};
use super::ModelDecoder;
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

/// Select the inline part factory only around the tool's stored part pointer.
/// Example: `embedded_parts::tool(reader, map)?` after a fresh tool tag.
pub(in crate::model_archives) fn tool(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    let previous = std::mem::replace(&mut reader.inline_part_scope, true);
    let previous_root = reader.verified_part_root.take();
    let result = reader.object("part_08", map, ModelObjectRole::Declared(&["moPart_c"]));
    reader.verified_part_root = previous_root;
    reader.inline_part_scope = previous;
    reader.checked(result)?;
    reader.i32s(&["raw_38", "raw_3c"])
}

/// Decode both bounded stages on the caller's map without importing external headers.
/// Example: `embedded_parts::read(reader, map)?` within the selected tool factory.
pub(in crate::model_archives) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    require_inline_scope(reader)?;
    let size = reader.section_size("lightweight_size")?;
    let (node, raw_1614) = reader.bounded(size, |reader| lightweight_body(reader, map))?;
    let size = reader.section_size("resolved_size")?;
    reader.bounded(size, |reader| {
        part_archive::resolved(reader, map, &node, raw_1614)
    })
}

fn lightweight_body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<(node::ModelNodePrefix, i32)> {
    reader.object("header", map, ModelObjectRole::Declared(&["moHeader_c"]))?;
    lightweight::after_header(reader, map, lightweight::UnitComponentContext::Inline)
}

fn require_inline_scope(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    if reader.inline_part_scope {
        return Ok(());
    }
    reader.checked(Err(BessemerError::new(
        "model archive",
        "embedded moPart_c",
        FailureClass::Unsupported,
        "inline part scope false; expected the forming-tool pointer's isolated factory",
    )))
}

#[cfg(test)]
#[path = "../../tests/resolved_features/embedded_parts.rs"]
mod tests;
