use super::{ModelDecoder, ModelObjectRole, lightweight_shared};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn appearances(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("hatch", map, ModelObjectRole::Declared(&["gcXhatch_c"]))?;
    reader.object(
        "material",
        map,
        ModelObjectRole::Declared(&["moMaterial_c"]),
    )?;
    reader.object("object_1580", map, ModelObjectRole::Any)?;
    body_appearances(reader, map)?;
    lightweight_shared::properties(reader)?;
    reader.i32("has_weldment")?;
    Ok(())
}

pub(super) const BODY_CLASSES: &[&str] = &[
    "moCompAtomBody_c",
    "moCompSolidBody_c",
    "moCompSurfaceBody_c",
];

fn body_appearances(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("body_appearance_count", 4)? {
        reader.scope(&format!("body_appearances[{index}]"), |reader| {
            reader.scope("property", lightweight_shared::property)?;
            reader.object("body", map, ModelObjectRole::Declared(BODY_CLASSES))?;
            reader.i32s(&["unknown_20", "unknown_24"])
        })?;
    }
    Ok(())
}

pub(super) fn tail(reader: &mut ModelDecoder<'_>) -> Result<i32> {
    reader.require_ordinary_part()?;
    reader.i32s(&["unknown_160c", "unknown_1610"])?;
    let raw_1614 = reader.i32("unknown_1614")?;
    if reader.version() <= 0x27d0 {
        return Ok(raw_1614);
    }
    for index in 0..reader.signed_count("unknown_map_1c58.count", 8)? {
        reader.i32(&format!("unknown_map_1c58[{index}].unknown_key"))?;
        reader.i32(&format!("unknown_map_1c58[{index}].unknown_value"))?;
    }
    Ok(raw_1614)
}
