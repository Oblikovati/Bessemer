use super::context_reader::ArchiveGraphRead;
use super::{
    MaterialRead, bucket_polygons as modern_bucket_poly, bucket_polygons::CoordinateWidth,
    vectors::double_vector as point,
};
use crate::Result;

const LEGACY_LAYER_FIELDS: [&str; 5] = [
    "hatch_layer_id",
    "command_line_color",
    "hatch_layer_color",
    "layer_visible",
    "layer_printable",
];

pub(crate) fn read<R: ArchiveGraphRead>(
    archive: &mut R,
    context: &mut R::ObjectContext,
    two_dimensional: bool,
) -> Result<()> {
    archive.i32s(&["node_id", "occurence", "smooth"])?;
    let count = archive.i32("count")?;
    if count <= 0 {
        return Ok(());
    }
    coordinates(archive, count as usize, two_dimensional)?;
    if archive.version() > 0x2f8 && archive.u8("has_user_line_property")? != 0 {
        archive.scope("user_line_property", modern_bucket_poly::line_property)?;
    }
    if archive.version() > 0x393 {
        modern_bucket_poly::type_payload(archive)?;
    }
    attributes(archive)?;
    persistent_tail(archive, context)
}

fn coordinates(archive: &mut impl MaterialRead, count: usize, two_dimensional: bool) -> Result<()> {
    let width = if archive.version() < 0x2f9a {
        CoordinateWidth::Float
    } else {
        CoordinateWidth::Double
    };
    modern_bucket_poly::coordinates(archive, count, if two_dimensional { 2 } else { 3 }, width)
}

fn persistent_tail<R: ArchiveGraphRead>(
    archive: &mut R,
    context: &mut R::ObjectContext,
) -> Result<()> {
    if archive.version() < 0x2f63 {
        return Ok(());
    }
    if archive.version() <= 0x2f95 {
        return archive.graph_object(context, "discarded_edge_data", &["gcBucketEdgeData_c"]);
    }
    if archive.version() < 0x3331 {
        archive.i32("face_node")?;
    }
    if archive.i32("has_arc_data")? != 0 {
        archive.scope("arc_data", modern_bucket_poly::arc_data)?;
    }
    if archive.i32("has_silh_data")? != 0 && archive.version() < 0x333c {
        // #28: keep the temporary ID words even where a later stored ID overwrites their meaning.
        archive.i32s(&["silh_id_high", "silh_id_low"])?;
    }
    modern_bucket_poly::identifiers(archive)
}

fn attributes(archive: &mut impl MaterialRead) -> Result<()> {
    early_attributes(archive)?;
    if archive.version() > 0xabc {
        archive.i32("new_region")?;
    }
    if archive.version() > 0xad2 {
        archive.i32("thread_ptn_xform_id")?;
    }
    if archive.version() > 0x12dd {
        archive.i32("bos")?;
    }
    if archive.version() >= 0x23b4 {
        archive.i32("section_cut_new")?;
    }
    if archive.version() <= 0x2789 {
        return Ok(());
    }
    archive.i32s(&LEGACY_LAYER_FIELDS)
}

pub(crate) fn edge(archive: &mut impl MaterialRead) -> Result<()> {
    point(archive, "start_point")?;
    point(archive, "end_point")?;
    archive.i32s(&["body_id", "face_node"])?;
    archive.text("comp_string")?;
    archive.i32s(&[
        "occurence_id",
        "edge_node",
        "bucket_index",
        "bucket_type",
        "edge_type",
    ])?;
    point(archive, "center_point")?;
    modern_bucket_poly::edge_legacy_words(archive)
}

fn early_attributes(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0x81e {
        archive.i32("new_boundary")?;
    }
    if archive.version() > 0x835 {
        archive.i32("atom_id")?;
    }
    Ok(())
}
