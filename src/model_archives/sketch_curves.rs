use super::{ModelDecoder, ModelObjectRole, sketch_entities, sketch_geometry};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
const CROSSHATCH: ModelObjectRole<'_> = ModelObjectRole::Declared(&["gcXhatch_c"]);
const HATCH_DATA: ModelObjectRole<'_> = ModelObjectRole::Declared(&["skHatchData_c"]);
const CHOOSER: ModelObjectRole<'_> = ModelObjectRole::Declared(&["moContourChooser_c"]);
const OBJECT_LIST: ModelObjectRole<'_> = ModelObjectRole::Declared(&["su_CObList", "suObList"]);
const DWORD_ARRAY: ModelObjectRole<'_> = ModelObjectRole::Declared(&["su_CDWordArray"]);

pub(super) fn hatches(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_geometry::collection(reader, map, ("hatch_count", "hatches"), None, hatch)
}

pub(super) fn arcs(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_geometry::collection(
        reader,
        map,
        ("arc_count", "arcs"),
        Some("stored_category_mask"),
        arc,
    )
}

pub(super) fn ellipses(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_geometry::collection(
        reader,
        map,
        ("ellipse_count", "ellipses"),
        Some("stored_category_mask"),
        ellipse,
    )
}

pub(super) fn conics(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_geometry::collection(
        reader,
        map,
        ("conic_count", "conics"),
        Some("stored_category_mask"),
        conic,
    )
}

fn hatch(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let flags = reader.scope("entity", |reader| sketch_entities::entity(reader, map))?;
    if flags.flags_54 & 1 != 0 {
        return Ok(());
    }
    hatch_indices(reader)?;
    reader.object("crosshatch", map, CROSSHATCH)?;
    reader.object("hatch_data", map, HATCH_DATA)?;
    hatch_contours(reader)?;
    reader.object("contour_chooser", map, CHOOSER)?;
    reader.object("object_list", map, OBJECT_LIST)?;
    reader.object("dword_array", map, DWORD_ARRAY)
}

fn hatch_indices(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for array in 0..4 {
        let label = format!("index_arrays[{array}]");
        for index in 0..reader.signed_count(&format!("{label}.count"), 4)? {
            reader.u32(&format!("{label}[{index}]"))?;
        }
    }
    Ok(())
}

fn hatch_contours(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("contour_count", 6)? {
        reader.scope(&format!("contours[{index}]"), |reader| {
            reader.i32("entity_index")?;
            reader.u16("category")?;
            Ok(())
        })?;
    }
    Ok(())
}

fn arc(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_geometry::segment(reader, map)?;
    reader.i32("rotation")?;
    reader.escaped_index("center", 0xffff)?;
    reader.i32s(&["normal", "filet_line1", "filet_line2", "virtual_point"])
}

fn ellipse(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_geometry::segment(reader, map)?;
    reader.i32("rotation")?;
    for label in [
        "center",
        "major_point",
        "minor_point",
        "major_anti_point",
        "minor_anti_point",
    ] {
        reader.escaped_index(label, 0xffff)?;
    }
    Ok(())
}

fn conic(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_geometry::segment(reader, map)?;
    reader.i32s(&["center", "apex", "shoulder", "top_vertex"])?;
    reader.f64s(&["eccentricity", "rho"])?;
    reader.u16("geometry_type")?;
    Ok(())
}
