use super::{
    archive_decode::DisplayDecoder,
    modern_items::{point, points},
};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>, kind: i32) -> Result<()> {
    match kind {
        1 => display_point(archive),
        2 | 101 => arc(archive, kind == 101),
        3 => ellipse(archive),
        4 => line(archive),
        5 | 6 => polygon(archive, kind == 6),
        13 => parabola(archive),
        28 => flip_end(archive),
        30 => annotation_end(archive),
        31 => spline(archive),
        33 => center_of_mass(archive),
        _ => typed_line(archive),
    }
}

fn display_point(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "point")?;
    archive.i32s(&["point_style", "unknown_1c"])
}

fn line(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    pair(archive)?;
    archive.u8("unknown_30")?;
    Ok(())
}

fn flip_end(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    triple(archive)?;
    archive.i32("unknown_48")?;
    Ok(())
}

fn annotation_end(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "unknown_00")?;
    archive.i32("unknown_18")?;
    Ok(())
}

fn typed_line(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    pair(archive)?;
    archive.i32("line_type")?;
    Ok(())
}

fn pair(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "unknown_00")?;
    point(archive, "unknown_18")
}

fn triple(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    pair(archive)?;
    point(archive, "unknown_30")
}

fn arc(archive: &mut DisplayDecoder<'_>, typed: bool) -> Result<()> {
    triple(archive)?;
    point(archive, "unknown_48")?;
    archive.i32(if typed { "line_type" } else { "unknown_60" })?;
    Ok(())
}

fn ellipse(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    triple(archive)?;
    point(archive, "unknown_48")?;
    point(archive, "unknown_60")?;
    archive.f64s(&["unknown_78", "unknown_80"])
}

fn polygon(archive: &mut DisplayDecoder<'_>, polygon: bool) -> Result<()> {
    if polygon {
        archive.i32("unknown_00")?;
    }
    let count = archive.count("points.count", 24)?;
    points(archive, "points", count)?;
    if !polygon && archive.context.version.value() > 0x12b0 {
        archive.i32("unknown_10")?;
    }
    Ok(())
}

fn parabola(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    triple(archive)?;
    point(archive, "unknown_48")?;
    point(archive, "unknown_68")?;
    if archive.context.version.value() > 0x13ec {
        archive.i32("unknown_60")?;
    }
    if archive.context.version.value() > 0xc58 {
        archive.f64s(&["unknown_80", "unknown_88"])?;
    }
    Ok(())
}

fn spline(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.count("points.count", 48)?;
    archive.entries(count, "spline vectors")?;
    archive.f64("unknown_10")?;
    if archive.context.version.value() >= 0x2016 {
        archive.i32("unknown_18")?;
    }
    points(archive, "points", count)?;
    points(archive, "vectors", count)?;
    if archive.context.version.value() >= 0x1ff8 {
        archive.text("display_name")?;
    }
    Ok(())
}

fn center_of_mass(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "unknown_00")?;
    archive.i32("style")?;
    if archive.context.version.value() > 0x146e {
        archive.f64("unknown_20")?;
        archive.i32("unknown_28")?;
    }
    if archive.context.version.value() >= 0x1473 {
        archive.f64("unknown_30")?;
    }
    Ok(())
}
