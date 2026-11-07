use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, dimensions};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

const SKETCH_HANDLES: &[&str] = &[
    "moSkDimHandle_c",
    "moSkDimHandleRadial_c",
    "moSkDimHandleOffset_c",
    "moSkDimHandleValG2_c",
    "moSkDimHandleValG3_c",
];

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
        "moRefDim_c" => base(reader, map),
        "moRefDim1E_c" => one_entity(reader, map),
        "moRefDim2E_c" => two_entities(reader, map),
        "moDimPlaneEntTag_c" => super::dimension_plane_tags::read(reader, map),
        _ => return None,
    };
    Some(result)
}

fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("dimension", |reader| dimensions::base(reader, map))?;
    reader.transform("transform_80")?;
    for label in ["88", "90"] {
        if reader.u8(&format!("has_vector_{label}"))? != 0 {
            double_vector(reader, &format!("vector_{label}"))?;
        }
    }
    reader.i32("raw_a4")?;
    declared(reader, map, "plane_tag_a8", &["moDimPlaneEntTag_c"])?;
    reader.i32("raw_b0")?;
    declared(reader, map, "sketch_dimension_handle_b8", SKETCH_HANDLES)?;
    reader.i32s(&["raw_a0", "raw_c4", "raw_c8"])?;
    if reader.version() >= 0x46c7 {
        reader.i32("raw_d0")?;
    }
    Ok(())
}

fn one_entity(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    reader.u16("kind_d8")?;
    reader.object("entity_e0", map, ModelObjectRole::Any)?;
    reader.u16("raw_f0")?;
    if reader.version() >= 0x2f6f {
        reader.u16("raw_f4")?;
    }
    Ok(())
}

fn two_entities(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    for (kind, entity) in [("d8", "e0"), ("f0", "f8")] {
        reader.u16(&format!("kind_{kind}"))?;
        reader.object(&format!("entity_{entity}"), map, ModelObjectRole::Any)?;
    }
    for label in ["108", "110", "114", "118", "11c"] {
        reader.u16(&format!("raw_{label}"))?;
    }
    if reader.version() > 0x3747 {
        reader.u16("raw_10c")?;
    }
    Ok(())
}
