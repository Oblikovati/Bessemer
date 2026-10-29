use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, mate_plane_collections, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the inherited reference plane and the complete ordered section suffix.
/// Example: `plane_sections::read(reader, map)?` for `moPlaneSection_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    inherited_geometry(reader, map)?;
    scalar_suffix(reader, map)?;
    mate_plane_collections::strings(reader, "map_8b8", true)?;
    mate_plane_collections::strings(reader, "map_8c8", false)?;
    object_map(reader, map)?;
    model_lists::pointers(reader, map, "list_8f8", ModelObjectRole::Any)?;
    mate_plane_collections::pointers(
        reader,
        map,
        "bodies_940",
        ModelObjectRole::Declared(&["moCompPlaneSectionBody_c"]),
    )?;
    visuals(reader, map)
}

fn inherited_geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("plane", |reader| {
        super::reference_planes::plane(reader, map)
    })?;
    reader.scope("section_data", |reader| {
        super::plane_section_geometry::read(reader, map)
    })
}

fn scalar_suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_710")?;
    mate_plane_collections::pointers(
        reader,
        map,
        "edges",
        ModelObjectRole::Declared(&["moPlaneSectionEdge_c"]),
    )?;
    declared(
        reader,
        map,
        "line_reference",
        super::geometric_references::LINE_REFERENCE_CLASSES,
    )?;
    reader.i32s(&["raw_714", "raw_718"])?;
    reader.f64("raw_720")?;
    reader.i32("raw_8b4")?;
    Ok(())
}

fn object_map(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("map_8e8.count", 6)? {
        reader.i32(&format!("map_8e8[{index}].key"))?;
        reader.object(
            &format!("map_8e8[{index}].value"),
            map,
            ModelObjectRole::Any,
        )?;
    }
    Ok(())
}

fn visuals(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("visuals_960.count", 4)? {
        declared(
            reader,
            map,
            &format!("visuals_960[{index}].body"),
            &["moCompPlaneSectionBody_c"],
        )?;
        declared(
            reader,
            map,
            &format!("visuals_960[{index}].visual"),
            &["moVisualProperties_c"],
        )?;
    }
    Ok(())
}

/// Preserve an edge's surface reference and strict body array without attribute lists.
/// Example: `plane_sections::edge(reader, map)?` for `moPlaneSectionEdge_c`.
pub(super) fn edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "surface", &["moGeneralSurfRef_w"])?;
    for index in 0..reader.signed_count("bodies.count", 1)? {
        reader.scope(
            &format!("bodies[{index}]"),
            super::plane_section_geometry::serialized_body,
        )?;
    }
    Ok(())
}
