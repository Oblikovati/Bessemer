use super::display_items::{point, points};
use super::item_reader::ItemRead;
use crate::Result;

pub(crate) fn read(archive: &mut impl ItemRead, kind: i32) -> Result<()> {
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

fn display_point(archive: &mut impl ItemRead) -> Result<()> {
    point(archive, "point")?;
    archive.i32s(&["point_style", "unknown_1c"])
}

fn line(archive: &mut impl ItemRead) -> Result<()> {
    pair(archive)?;
    if archive.version() <= 0xbf8 {
        archive.i32("unknown_30")?;
    } else {
        archive.u8("unknown_30")?;
    }
    Ok(())
}

fn flip_end(archive: &mut impl ItemRead) -> Result<()> {
    triple(archive)?;
    archive.i32("unknown_48")?;
    Ok(())
}

fn annotation_end(archive: &mut impl ItemRead) -> Result<()> {
    point(archive, "unknown_00")?;
    archive.i32("unknown_18")?;
    Ok(())
}

fn typed_line(archive: &mut impl ItemRead) -> Result<()> {
    pair(archive)?;
    archive.i32("line_type")?;
    Ok(())
}

fn pair(archive: &mut impl ItemRead) -> Result<()> {
    point(archive, "unknown_00")?;
    point(archive, "unknown_18")
}

fn triple(archive: &mut impl ItemRead) -> Result<()> {
    pair(archive)?;
    point(archive, "unknown_30")
}

fn arc(archive: &mut impl ItemRead, typed: bool) -> Result<()> {
    triple(archive)?;
    point(archive, "unknown_48")?;
    archive.i32(if typed { "line_type" } else { "unknown_60" })?;
    Ok(())
}

fn ellipse(archive: &mut impl ItemRead) -> Result<()> {
    triple(archive)?;
    point(archive, "unknown_48")?;
    point(archive, "unknown_60")?;
    archive.f64s(&["unknown_78", "unknown_80"])
}

fn polygon(archive: &mut impl ItemRead, polygon: bool) -> Result<()> {
    if polygon {
        archive.i32("unknown_00")?;
    }
    let count = archive.item_count("points.count", 24)?;
    points(archive, "points", count)?;
    if !polygon && archive.version() > 0x12b0 {
        archive.i32("unknown_10")?;
    }
    Ok(())
}

fn parabola(archive: &mut impl ItemRead) -> Result<()> {
    triple(archive)?;
    point(archive, "unknown_48")?;
    point(archive, "unknown_68")?;
    if archive.version() > 0x13ec {
        archive.i32("unknown_60")?;
    }
    if archive.version() > 0xc58 {
        archive.f64s(&["unknown_80", "unknown_88"])?;
    }
    Ok(())
}

fn spline(archive: &mut impl ItemRead) -> Result<()> {
    let count = archive.item_count("points.count", 48)?;
    archive.entries(count, "spline vectors")?;
    archive.f64("unknown_10")?;
    if archive.version() >= 0x2016 {
        archive.i32("unknown_18")?;
    }
    points(archive, "points", count)?;
    points(archive, "vectors", count)?;
    if archive.version() >= 0x1ff8 {
        archive.text("display_name")?;
    }
    Ok(())
}

fn center_of_mass(archive: &mut impl ItemRead) -> Result<()> {
    point(archive, "unknown_00")?;
    archive.i32("style")?;
    if archive.version() > 0x146e {
        archive.f64("unknown_20")?;
        archive.i32("unknown_28")?;
    }
    if archive.version() >= 0x1473 {
        archive.f64("unknown_30")?;
    }
    Ok(())
}
