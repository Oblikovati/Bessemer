use super::context_reader::ArchiveGraphRead;
use super::{
    MaterialRead, scene_data::nonnegative_count as wall_count, vectors::double_vector as point,
};
use crate::Result;

pub(crate) fn list<R: ArchiveGraphRead>(
    archive: &mut R,
    context: &mut R::ObjectContext,
    two_dimensional: bool,
) -> Result<()> {
    let count = wall_count(archive, "count", 16)?;
    for index in 0..count {
        archive.scope(&format!("items[{index}]"), |archive| {
            polygon(archive, context, two_dimensional)
        })?;
    }
    Ok(())
}

fn polygon<R: ArchiveGraphRead>(
    archive: &mut R,
    context: &mut R::ObjectContext,
    two_dimensional: bool,
) -> Result<()> {
    if archive.version() <= 0x41a0 {
        return super::bucket_legacy_polygons::read(archive, context, two_dimensional);
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
    archive: &mut impl MaterialRead,
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

fn polygon_tail(archive: &mut impl MaterialRead, flags: u32) -> Result<()> {
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
pub(crate) enum CoordinateWidth {
    Float,
    Double,
}

pub(crate) fn coordinates(
    archive: &mut impl MaterialRead,
    count: usize,
    dimensions: usize,
    width: CoordinateWidth,
) -> Result<()> {
    let bytes = match width {
        CoordinateWidth::Float => 4,
        CoordinateWidth::Double => 8,
    };
    if count > archive.remaining() / (dimensions * bytes) {
        return Err(archive.invalid(format!(
            "polygon point count {count}, dimensions {dimensions}; expected coordinates fitting {} bytes",
            archive.remaining()
        )));
    }
    archive.entries(count * dimensions, "bucket polygon coordinates")?;
    for index in 0..count * dimensions {
        coordinate_value(archive, index, width)?;
    }
    Ok(())
}

fn coordinate_value(
    archive: &mut impl MaterialRead,
    index: usize,
    width: CoordinateWidth,
) -> Result<()> {
    match width {
        CoordinateWidth::Float => archive.f32(&format!("points_f32[{index}]")),
        CoordinateWidth::Double => archive.f64(&format!("points[{index}]")),
    }
}

pub(crate) fn type_payload(archive: &mut impl MaterialRead) -> Result<()> {
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

pub(crate) fn line_property(archive: &mut impl MaterialRead) -> Result<()> {
    archive.text("style_name")?;
    archive.scope("width", super::line_style::width)?;
    archive.i32("color")?;
    if archive.version() >= 0xa80 {
        archive.i32("visibility")?;
    }
    Ok(())
}

pub(crate) fn arc_data(archive: &mut impl MaterialRead) -> Result<()> {
    for label in ["unknown_8", "unknown_20", "unknown_38"] {
        point(archive, label)?;
    }
    if archive.version() <= 0x3350 {
        point(archive, "unknown_50")?;
    }
    if archive.version() > 0x2fa9 {
        width_flag(archive, "unknown_68")?;
    }
    if archive.version() >= 0x3724 {
        archive.u8("unknown_6c")?;
    }
    arc_tail(archive)
}

fn arc_tail(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0x3336 {
        archive.i32("unknown_70")?;
    }
    if archive.version() > 0x3344 {
        width_flag(archive, "unknown_74")?;
    }
    if archive.version() >= 0x3364 {
        archive.i32("unknown_78")?;
    }
    Ok(())
}

fn width_flag(archive: &mut impl MaterialRead, label: &str) -> Result<()> {
    if archive.version() > 0x3350 {
        archive.u8(label)?;
    } else {
        archive.i32(label)?;
    }
    Ok(())
}

pub(crate) fn identifiers(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0x3330 {
        archive.u32("edge_persistent_id")?;
        archive.u32("face1_persistent_id")?;
    }
    if archive.version() > 0x3333 {
        archive.u32("face2_persistent_id")?;
    }
    if archive.version() > 0x3350 {
        parameters(archive)?;
    }
    Ok(())
}

fn parameters(archive: &mut impl MaterialRead) -> Result<()> {
    archive.i32("unique_disp_id")?;
    archive.f64s(&["mid_point_param0", "mid_point_param1"])
}

pub(crate) fn edge(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() < 0x2fac {
        return super::bucket_legacy_polygons::edge(archive);
    }
    edge_geometry(archive)?;
    archive.i32("occurence_id")?;
    if archive.version() < 0x3331 {
        archive.i32("edge_node")?;
    }
    archive.i32s(&["bucket_index", "bucket_type", "edge_type"])?;
    point(archive, "center_point")?;
    edge_tail(archive)
}

fn edge_geometry(archive: &mut impl MaterialRead) -> Result<()> {
    point(archive, "start_point")?;
    point(archive, "end_point")?;
    archive.i32("body_id")?;
    if archive.version() < 0x3331 {
        archive.i32("face_node")?;
    }
    archive.text("comp_string")?;
    if archive.version() > 0x3330 {
        archive.u32("face1_persistent_id")?;
    }
    if archive.version() > 0x3333 {
        archive.u32("face2_persistent_id")?;
    }
    Ok(())
}

fn edge_tail(archive: &mut impl MaterialRead) -> Result<()> {
    edge_legacy_words(archive)?;
    if archive.version() > 0x2fac {
        width_flag(archive, "linear_in_2d")?;
        width_flag(archive, "is_vertex")?;
    }
    edge_flags(archive)
}

pub(crate) fn edge_legacy_words(archive: &mut impl MaterialRead) -> Result<()> {
    if (0x2f7b..0x333c).contains(&archive.version()) {
        archive.i32s(&["unknown_legacy[0]", "unknown_legacy[1]"])?;
    }
    Ok(())
}

fn edge_flags(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0x3330 {
        archive.u32("edge_persistent_id")?;
    }
    if archive.version() > 0x3336 {
        archive.i32("feat_id")?;
    }
    if archive.version() > 0x3344 {
        width_flag(archive, "is_assembly_feat")?;
    }
    if archive.version() > 0x3350 {
        parameters(archive)?;
    }
    if archive.version() > 0x3363 {
        archive.i32("hole_index")?;
    }
    Ok(())
}
