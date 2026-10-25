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
    if archive.context.version.value() <= 0x41a0 {
        return super::legacy_bucket_poly::read(archive, two_dimensional);
    }
    archive.i32s(&["node_id", "occurence"])?;
    let flags = archive.u32("flags")?;
    let count = archive.i32("count")?;
    if count <= 0 {
        return Ok(());
    }
    polygon_contents(archive, count as usize, two_dimensional, flags)
}

fn polygon_contents(
    archive: &mut DisplayDecoder<'_>,
    count: usize,
    two_dimensional: bool,
    flags: u32,
) -> Result<()> {
    coordinates(
        archive,
        count,
        if two_dimensional { 2 } else { 3 },
        CoordinateWidth::Double,
    )?;
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

#[derive(Clone, Copy)]
pub(super) enum CoordinateWidth {
    Float,
    Double,
}

pub(super) fn coordinates(
    archive: &mut DisplayDecoder<'_>,
    count: usize,
    dimensions: usize,
    width: CoordinateWidth,
) -> Result<()> {
    let bytes = match width {
        CoordinateWidth::Float => 4,
        CoordinateWidth::Double => 8,
    };
    if count > archive.cursor.remaining() / (dimensions * bytes) {
        return Err(super::archive_error::invalid(format!(
            "polygon point count {count}, dimensions {dimensions}; expected coordinates fitting {} bytes",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count * dimensions, "bucket polygon coordinates")?;
    for index in 0..count * dimensions {
        coordinate_value(archive, index, width)?;
    }
    Ok(())
}

fn coordinate_value(
    archive: &mut DisplayDecoder<'_>,
    index: usize,
    width: CoordinateWidth,
) -> Result<()> {
    match width {
        CoordinateWidth::Float => archive.f32(&format!("points_f32[{index}]")),
        CoordinateWidth::Double => archive.f64(&format!("points[{index}]")),
    }
}

pub(super) fn type_payload(archive: &mut DisplayDecoder<'_>) -> Result<()> {
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

pub(super) fn line_property(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("style_name")?;
    archive.i16("width.standard_width")?;
    archive.f32("width.custom_width")?;
    archive.i32s(&["color", "visibility"])
}

pub(super) fn arc_data(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    for label in ["unknown_8", "unknown_20", "unknown_38"] {
        point(archive, label)?;
    }
    if archive.context.version.value() <= 0x3350 {
        point(archive, "unknown_50")?;
    }
    if archive.context.version.value() > 0x2fa9 {
        width_flag(archive, "unknown_68")?;
    }
    if archive.context.version.value() >= 0x3724 {
        archive.u8("unknown_6c")?;
    }
    arc_tail(archive)
}

fn arc_tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x3336 {
        archive.i32("unknown_70")?;
    }
    if archive.context.version.value() > 0x3344 {
        width_flag(archive, "unknown_74")?;
    }
    if archive.context.version.value() >= 0x3364 {
        archive.i32("unknown_78")?;
    }
    Ok(())
}

fn width_flag(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    if archive.context.version.value() > 0x3350 {
        archive.u8(label)?;
    } else {
        archive.i32(label)?;
    }
    Ok(())
}

pub(super) fn identifiers(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x3330 {
        archive.u32("edge_persistent_id")?;
        archive.u32("face1_persistent_id")?;
    }
    if archive.context.version.value() > 0x3333 {
        archive.u32("face2_persistent_id")?;
    }
    if archive.context.version.value() > 0x3350 {
        parameters(archive)?;
    }
    Ok(())
}

fn parameters(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("unique_disp_id")?;
    archive.f64s(&["mid_point_param0", "mid_point_param1"])
}

pub(super) fn edge(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0x2fac {
        return super::legacy_bucket_poly::edge(archive);
    }
    edge_geometry(archive)?;
    archive.i32("occurence_id")?;
    if archive.context.version.value() < 0x3331 {
        archive.i32("edge_node")?;
    }
    archive.i32s(&["bucket_index", "bucket_type", "edge_type"])?;
    point(archive, "center_point")?;
    edge_tail(archive)
}

fn edge_geometry(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "start_point")?;
    point(archive, "end_point")?;
    archive.i32("body_id")?;
    if archive.context.version.value() < 0x3331 {
        archive.i32("face_node")?;
    }
    archive.text("comp_string")?;
    if archive.context.version.value() > 0x3330 {
        archive.u32("face1_persistent_id")?;
    }
    if archive.context.version.value() > 0x3333 {
        archive.u32("face2_persistent_id")?;
    }
    Ok(())
}

fn edge_tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    edge_legacy_words(archive)?;
    if archive.context.version.value() > 0x2fac {
        width_flag(archive, "linear_in_2d")?;
        width_flag(archive, "is_vertex")?;
    }
    edge_flags(archive)
}

pub(super) fn edge_legacy_words(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if (0x2f7b..0x333c).contains(&archive.context.version.value()) {
        archive.i32s(&["unknown_legacy[0]", "unknown_legacy[1]"])?;
    }
    Ok(())
}

fn edge_flags(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x3330 {
        archive.u32("edge_persistent_id")?;
    }
    if archive.context.version.value() > 0x3336 {
        archive.i32("feat_id")?;
    }
    if archive.context.version.value() > 0x3344 {
        width_flag(archive, "is_assembly_feat")?;
    }
    if archive.context.version.value() > 0x3350 {
        parameters(archive)?;
    }
    if archive.context.version.value() > 0x3363 {
        archive.i32("hole_index")?;
    }
    Ok(())
}
