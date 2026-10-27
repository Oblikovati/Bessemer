use super::ModelDecoder;
use super::display_records::declared;
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Result<()> {
    match name {
        "moFixedFaceInFlat_c" => fixed_face(reader, map),
        "moCompSheetmetalFeat_c" => component(reader, map),
        _ => feature(reader, map, name == "moTemplateSheetMetal_c"),
    }
}

fn feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, template: bool) -> Result<()> {
    reader.require_ordinary_part()?;
    if template {
        reader.require_saved_helper_state()?;
    }
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    reader.scope("payload", |reader| super::sheet_payload::read(reader, map))
}

fn fixed_face(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    if reader.sheet_metal_extended() {
        reader.i32("raw_bend_id_10")?;
    } else {
        declared(reader, map, "bend_08", &["moOneBend_c"])?;
    }
    reader.i32("raw_14")?;
    declared(reader, map, "face_18", &["moCompFace_c"])?;
    reader.f64("raw_20")?;
    reader.f64("raw_28")?;
    declared(
        reader,
        map,
        "edge_30",
        &["moCompEdge_c", "moCompSilhEdge_c"],
    )
}

fn component(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("component_reference", |reader| {
        super::component_references::base(reader, map)
    })?;
    reader.i32("raw_e8")?;
    Ok(())
}
