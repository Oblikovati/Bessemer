use super::{ModelDecoder, ModelObjectRole, display_records::declared, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, visibility};

const COMPONENT_REFERENCES: &[&str] = &[
    "moCompRef_c",
    "moCompRefPlane_c",
    "moCompRefSurface_c",
    "moCompRefAxis_c",
    "moCompRefPoint_c",
];

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.u16("word_50")?;
    reader.object("owner", map, ModelObjectRole::Any)?;
    declared(reader, map, "view", &["moDrView"])?;
    declared(reader, map, "component_reference", COMPONENT_REFERENCES)?;
    reader.i32s(&["unknown_54", "unknown_11c"])?;
    reader.scope("visual_properties", visibility::read)?;
    continuation(reader, map)
}

fn continuation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["references_68", "references_b0"] {
        reader.scope(label, |reader| {
            model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
        })?;
    }
    reader.i32("unknown_58")?;
    declared(reader, map, "linked_objects", &["moLinkedDispObjArray_c"])?;
    let present = reader.i32("has_link_data")?;
    super::part_objects::absent_payload(reader, "display object link data", present)?;
    reader.i32("stored_version")?;
    Ok(())
}

pub(super) fn reference(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    sketch_arrays::integers(reader, "values", 4)
}

pub(super) fn component_reference(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reference(reader)?;
    reader.object("component", map, ModelObjectRole::Any)
}

pub(super) fn arrow_size(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    sketch_arrays::doubles(reader, "values", 3)
}
