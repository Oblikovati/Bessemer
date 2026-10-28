use super::{ModelDecoder, helix_records};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the complete reference-curve parent and native member order.
/// Example: `cosmetic_weld_cut_lists::read(reader, map)?` for a cut-list object.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_curve", |reader| {
        helix_records::reference_curve(reader, map)
    })?;
    // #54 / C-389: the serializer's order differs from ascending member offsets.
    reader.i32s(&["raw_888", "raw_780", "raw_778", "raw_784", "raw_788"])
}
