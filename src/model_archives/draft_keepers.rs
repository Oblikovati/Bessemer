use super::{ModelDecoder, ModelObjectRole, boundary_references, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    entity_ids(reader, map, "surface_ids_18")?;
    model_lists::pointers(reader, map, "references_50", ModelObjectRole::Any)?;
    if reader.version() > 0x3746 {
        entity_ids(reader, map, "surface_ids_28")?;
    }
    Ok(())
}

fn entity_ids(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 2)? {
        reader.scope(&format!("{label}[{index}]"), |reader| {
            boundary_references::entity_id(reader, map)
        })?;
    }
    Ok(())
}
