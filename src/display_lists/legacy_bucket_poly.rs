use super::{
    archive_decode::DisplayDecoder,
    modern_bucket_poly::{self, CoordinateWidth},
    modern_items::point,
};
use crate::Result;

const LEGACY_LAYER_FIELDS: [&str; 5] = [
    "hatch_layer_id",
    "command_line_color",
    "hatch_layer_color",
    "layer_visible",
    "layer_printable",
];

pub(super) fn read(archive: &mut DisplayDecoder<'_>, two_dimensional: bool) -> Result<()> {
    archive.i32s(&["node_id", "occurence", "smooth"])?;
    let count = archive.i32("count")?;
    if count <= 0 {
        return Ok(());
    }
    coordinates(archive, count as usize, two_dimensional)?;
    if archive.u8("has_user_line_property")? != 0 {
        archive.scope("user_line_property", modern_bucket_poly::line_property)?;
    }
    modern_bucket_poly::type_payload(archive)?;
    attributes(archive)?;
    persistent_tail(archive)
}

fn coordinates(
    archive: &mut DisplayDecoder<'_>,
    count: usize,
    two_dimensional: bool,
) -> Result<()> {
    let width = if archive.context.version.value() < 0x2f9a {
        CoordinateWidth::Float
    } else {
        CoordinateWidth::Double
    };
    modern_bucket_poly::coordinates(archive, count, if two_dimensional { 2 } else { 3 }, width)
}

fn persistent_tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0x2f63 {
        return Ok(());
    }
    if archive.context.version.value() <= 0x2f95 {
        return archive.object("discarded_edge_data", &["gcBucketEdgeData_c"]);
    }
    if archive.context.version.value() < 0x3331 {
        archive.i32("face_node")?;
    }
    if archive.i32("has_arc_data")? != 0 {
        archive.scope("arc_data", modern_bucket_poly::arc_data)?;
    }
    if archive.i32("has_silh_data")? != 0 && archive.context.version.value() < 0x333c {
        // #28: keep the temporary ID words even where a later stored ID overwrites their meaning.
        archive.i32s(&["silh_id_high", "silh_id_low"])?;
    }
    modern_bucket_poly::identifiers(archive)
}

fn attributes(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&["new_boundary", "atom_id", "new_region"])?;
    if archive.context.version.value() > 0xad2 {
        archive.i32("thread_ptn_xform_id")?;
    }
    if archive.context.version.value() > 0x12dd {
        archive.i32("bos")?;
    }
    if archive.context.version.value() >= 0x23b4 {
        archive.i32("section_cut_new")?;
    }
    if archive.context.version.value() <= 0x2789 {
        return Ok(());
    }
    archive.i32s(&LEGACY_LAYER_FIELDS)
}

pub(super) fn edge(archive: &mut DisplayDecoder<'_>) -> Result<()> {
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
