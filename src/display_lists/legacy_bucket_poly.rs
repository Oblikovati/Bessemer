use super::{
    archive_decode::DisplayDecoder,
    modern_bucket_poly::{self, CoordinateWidth},
    modern_items::point,
};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>, two_dimensional: bool) -> Result<()> {
    archive.i32s(&["node_id", "occurence", "smooth"])?;
    let count = archive.i32("count")?;
    if count <= 0 {
        return Ok(());
    }
    modern_bucket_poly::coordinates(
        archive,
        count as usize,
        if two_dimensional { 2 } else { 3 },
        CoordinateWidth::Float,
    )?;
    if archive.u8("has_user_line_property")? != 0 {
        archive.scope("user_line_property", modern_bucket_poly::line_property)?;
    }
    modern_bucket_poly::type_payload(archive)?;
    attributes(archive)
}

fn attributes(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&[
        "new_boundary",
        "atom_id",
        "new_region",
        "thread_ptn_xform_id",
        "bos",
    ])?;
    if archive.context.version.value() >= 0x23b4 {
        archive.i32("section_cut_new")?;
    }
    if archive.context.version.value() <= 0x2789 {
        return Ok(());
    }
    archive.i32s(&[
        "hatch_layer_id",
        "command_line_color",
        "hatch_layer_color",
        "layer_visible",
        "layer_printable",
    ])
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
    point(archive, "center_point")
}
