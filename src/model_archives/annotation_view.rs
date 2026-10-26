use super::{ModelDecoder, ModelObjectRole, views};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("view", |reader| views::view(reader, map))?;
    reader.i32("raw_3d0")?;
    reader.object(
        "component_reference",
        map,
        ModelObjectRole::Declared(&["moCompRef_c", "moCompRefPlane_c"]),
    )?;
    reader.object(
        "reference_3e0_initial",
        map,
        ModelObjectRole::Declared(&["moRefWrapper_c"]),
    )?;
    reader.i32("raw_3e8")?;
    reader.f64("raw_3d8")?;
    reader.object("reference_3e0_final", map, ModelObjectRole::Any)?;
    reader.i32s(&["raw_3ec", "raw_3f0"])?;
    reader.object(
        "comment",
        map,
        ModelObjectRole::Declared(&["moFeatComment_c"]),
    )?;
    versioned_words(reader)
}

fn versioned_words(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for (offset, gate) in [
        ("3f4", 0x2b7e),
        ("400", 0x2b9a),
        ("3f8", 0x2ba0),
        ("3fc", 0x2bbb),
    ] {
        if reader.version() > gate {
            reader.i32(&format!("raw_{offset}"))?;
        }
    }
    Ok(())
}
