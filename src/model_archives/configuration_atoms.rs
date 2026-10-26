use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn face_color(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("face_10", map, ModelObjectRole::Declared(&["moCompFace_c"]))?;
    reader.object(
        "visual_18",
        map,
        ModelObjectRole::Declared(&["moVisualProperties_c"]),
    )?;
    reader.i32("raw_08")?;
    Ok(())
}

pub(super) fn linked_map(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    for index in 0..reader.unsigned_count(&format!("{label}.count"), 6)? {
        reader.i32(&format!("{label}[{index}].key"))?;
        reader.object(
            &format!("{label}[{index}].object"),
            map,
            ModelObjectRole::Declared(&["moLinkedAtomIdNode_c"]),
        )?;
    }
    Ok(())
}

pub(super) fn linked_node(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_08")?;
    for label in ["dwords_90", "dwords_d0"] {
        model_lists::unsigned_words(reader, label)?;
    }
    for label in ["map_f0", "map_100", "map_110", "map_120"] {
        linked_map(reader, map, label)?;
    }
    Ok(())
}
