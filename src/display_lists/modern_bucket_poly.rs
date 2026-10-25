use super::{archive_decode::DisplayDecoder, modern_items::point, modern_scene::wall_count};
use crate::Result;

pub(super) fn list(archive: &mut DisplayDecoder<'_>, two_dimensional: bool) -> Result<()> {
    let count = wall_count(archive, "count", 16)?;
    for index in 0..count {
        archive.scope(&format!("items[{index}]"), |archive| {
            polygon(archive, two_dimensional)
        })?;
    }
    Ok(())
}

fn polygon(archive: &mut DisplayDecoder<'_>, two_dimensional: bool) -> Result<()> {
    archive.i32s(&["node_id", "occurence"])?;
    let flags = archive.u32("flags")?;
    let count = archive.i32("count")?;
    if count <= 0 {
        return Ok(());
    }
    coordinates(archive, count as usize, if two_dimensional { 2 } else { 3 })?;
    if archive.u8("has_user_line_property")? != 0 {
        archive.scope("user_line_property", line_property)?;
    }
    type_payload(archive)?;
    polygon_tail(archive, flags)
}

fn polygon_tail(archive: &mut DisplayDecoder<'_>, flags: u32) -> Result<()> {
    archive.i32s(&[
        "atom_id",
        "thread_ptn_xform_id",
        "hatch_layer_id",
        "command_line_color",
        "hatch_layer_color",
    ])?;
    if flags & 0x20 != 0 {
        archive.scope("arc_data", arc_data)?;
    }
    identifiers(archive)
}

fn coordinates(archive: &mut DisplayDecoder<'_>, count: usize, dimensions: usize) -> Result<()> {
    if count > archive.cursor.remaining() / (dimensions * 8) {
        return Err(super::archive_error::invalid(format!(
            "polygon point count {count}, dimensions {dimensions}; expected coordinates fitting {} bytes",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count * dimensions, "bucket polygon coordinates")?;
    for index in 0..count * dimensions {
        archive.f64(&format!("points[{index}]"))?;
    }
    Ok(())
}

fn type_payload(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let kind = archive.i32("type")?;
    if !(1..=3).contains(&kind) {
        return Ok(());
    }
    let count = wall_count(archive, "type_data.count", 8)?;
    for index in 0..count {
        archive.f64(&format!("type_data[{index}]"))?;
    }
    Ok(())
}

fn line_property(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("style_name")?;
    archive.i16("width.standard_width")?;
    archive.f32("width.custom_width")?;
    archive.i32s(&["color", "visibility"])
}

fn arc_data(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    for label in ["unknown_8", "unknown_20", "unknown_38"] {
        point(archive, label)?;
    }
    archive.u8("unknown_68")?;
    archive.u8("unknown_6c")?;
    archive.i32("unknown_70")?;
    archive.u8("unknown_74")?;
    archive.i32("unknown_78")?;
    Ok(())
}

fn identifiers(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.u32("edge_persistent_id")?;
    archive.u32("face1_persistent_id")?;
    archive.u32("face2_persistent_id")?;
    archive.i32("unique_disp_id")?;
    archive.f64s(&["mid_point_param0", "mid_point_param1"])
}

pub(super) fn edge(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "start_point")?;
    point(archive, "end_point")?;
    archive.i32("body_id")?;
    archive.text("comp_string")?;
    archive.u32("face1_persistent_id")?;
    archive.u32("face2_persistent_id")?;
    archive.i32s(&["occurence_id", "bucket_index", "bucket_type", "edge_type"])?;
    point(archive, "center_point")?;
    edge_tail(archive)
}

fn edge_tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.u8("linear_in_2d")?;
    archive.u8("is_vertex")?;
    archive.u32("edge_persistent_id")?;
    archive.i32("feat_id")?;
    archive.u8("is_assembly_feat")?;
    archive.i32("unique_disp_id")?;
    archive.f64s(&["mid_point_param0", "mid_point_param1"])?;
    archive.i32("hole_index")?;
    Ok(())
}
