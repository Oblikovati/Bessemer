use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists, model_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the direct child without inventing a tag; e.g. `read_child(reader, map)?`.
pub(super) fn read_child(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    // #217: the native base is ordinary su_CObject, whose binary body is empty.
    for offset in ["10", "30", "50"] {
        model_lists::pointers(reader, map, &format!("list_{offset}"), ModelObjectRole::Any)?;
    }
    declared(reader, map, "wrapper_70", model_references::WRAPPER_CLASSES)?;
    declared(reader, map, "synch_78", &["moLoftSynchRef_c"])?;
    declared(
        reader,
        map,
        "connected_80",
        &["moMoldPartingConnectedComponent_c"],
    )?;
    declared(reader, map, "direction_88", &["moDirectionSpec_c"])
}

/// Preserve the connected component's direct arrays and raw word; e.g. `read_connected(reader, map)?`.
pub(super) fn read_connected(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for offset in ["08", "90"] {
        model_lists::pointers(reader, map, &format!("list_{offset}"), ModelObjectRole::Any)?;
    }
    model_lists::unsigned_words(reader, "dwords_b0")?;
    reader.i32("raw_80")?;
    declared(reader, map, "synch_28", &["moLoftSynchRef_c"])
}
