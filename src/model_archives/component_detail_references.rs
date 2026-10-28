use super::{
    ModelDecoder, ModelObjectRole, component_references, model_lists, node, sketch_arrays,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Retain the exact internal component-detail branch after proving its component.
/// Example: `component_detail_references::read(reader, map)?` at its class dispatch.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let component = reader.scope("component_reference", |reader| {
        component_references::base_record(reader, map)
    })?;
    reader.require_current_part_unit(&component)?;
    for label in ["detail_e8", "detail_f8"] {
        reader.object(label, map, ModelObjectRole::Declared(&["moDetailItem_c"]))?;
    }
    reader.i32("raw_100")?;
    Ok(())
}

/// Decode the complete detail-item base without adding a tagged attachment list.
/// Example: `component_detail_references::detail_item(reader, map)?` for the base class.
pub(super) fn detail_item(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("node", |reader| node::lightweight(reader, map))?;
    reader.u16("raw_48")?;
    reader.object("owner_40", map, ModelObjectRole::Any)?;
    model_lists::pointers(
        reader,
        map,
        "attachments_50",
        ModelObjectRole::Declared(&["moAttachPoint_c"]),
    )?;
    let count = reader.signed_count("raw_values_60.count", 4)?;
    sketch_arrays::integers(reader, "raw_values_60", count)?;
    reader.i32("raw_78")?;
    reader.object(
        "attachment_80",
        map,
        ModelObjectRole::Declared(&["moAttachPoint_c"]),
    )
}
