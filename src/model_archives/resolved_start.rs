use super::{ModelDecoder, ModelObjectRole, model_lists, resolved_managers};
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("feature_list", |reader| model_lists::nodes(reader, map))?;
    reader.scope("atom_manager", |reader| {
        resolved_managers::atom_manager(reader, map)
    })?;
    reader.scope("ghost_manager", |reader| {
        resolved_managers::ghost_manager(reader, map)
    })?;
    reader.object(
        "relation_manager",
        map,
        ModelObjectRole::Declared(&["moRelMgr_c"]),
    )?;
    reader.scope("array_b00", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })
}
