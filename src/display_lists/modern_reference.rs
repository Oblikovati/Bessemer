use super::{archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let high = archive.u32("count_high")?;
    let low = archive.u32("count_low")?;
    let count = (u64::from(high) << 32) | u64::from(low);
    if count > archive.cursor.remaining() as u64 / 4 {
        return Err(invalid(format!(
            "reference geometry count {count}; expected entries fitting remaining bytes"
        )));
    }
    archive.entries(count as usize, "reference geometry")?;
    for index in 0..count {
        archive.scope(&format!("entries[{index}]"), reference_item)?;
    }
    Ok(())
}

fn reference_item(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let kind = ReferenceKind::try_from(archive.i32("type")?)?;
    archive.text("unknown_8")?;
    archive.i32s(&["unknown_10", "unknown_14", "unknown_18"])?;
    match kind {
        ReferenceKind::Origin => Ok(()),
        ReferenceKind::Point => vector(archive, "unknown_38"),
        ReferenceKind::Axis => axis(archive),
        ReferenceKind::Plane => reference_plane(archive),
        ReferenceKind::CoordinateSystem => coordinate_system(archive),
        ReferenceKind::CPoint => cpoint(archive),
        ReferenceKind::RoutePoint => route_point(archive),
    }
}

fn reference_plane(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    vector(archive, "unknown_38")?;
    vector(archive, "unknown_50")?;
    archive.fixed_transform("unknown_68")?;
    for index in 0..4 {
        archive.f64(&format!("unknown_f0[{index}]"))?;
    }
    archive.i32s(&["unknown_110", "unknown_114"])
}

enum ReferenceKind {
    Origin,
    Point,
    Axis,
    Plane,
    CoordinateSystem,
    CPoint,
    RoutePoint,
}

impl TryFrom<i32> for ReferenceKind {
    type Error = crate::BessemerError;
    fn try_from(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::Origin),
            1 => Ok(Self::Point),
            2 => Ok(Self::Axis),
            3 => Ok(Self::Plane),
            4 => Ok(Self::CoordinateSystem),
            5 => Ok(Self::CPoint),
            6 => Ok(Self::RoutePoint),
            other => Err(invalid(format!(
                "reference geometry kind {other}; expected 0..=6"
            ))),
        }
    }
}

fn vector(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..3 {
        archive.f64(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

fn vector_triple(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..3 {
        vector(archive, &format!("{label}[{index}]"))?;
    }
    Ok(())
}

fn axis(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    vector_triple(archive, "unknown_38")?;
    vector(archive, "unknown_80")?;
    archive.f64s(&["unknown_98[0]", "unknown_98[1]"])
}

fn coordinate_system(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    vector_triple(archive, "unknown_38")?;
    for index in 0..5 {
        archive.i32(&format!("unknown_80[{index}]"))?;
    }
    vector(archive, "unknown_98")
}

fn cpoint(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("unknown_b8")?;
    archive.i32("unknown_b0")?;
    vector_triple(archive, "unknown_40")?;
    vector(archive, "unknown_88")?;
    archive.f64s(&["unknown_a0[0]", "unknown_a0[1]"])?;
    archive.scope("unknown_c0", cpoint_extra)
}

fn cpoint_extra(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.f64s(&["f0", "f8"])?;
    vector(archive, "vec")?;
    vector(archive, "pt")?;
    archive.i32s(&["i40", "i44"])?;
    archive.f64("f60")?;
    archive.i32s(&["i68", "i6c", "i78"])?;
    archive.f64("f98")?;
    archive.i32s(&["ia0", "ia4"])?;
    archive.text("sa8")?;
    archive.text("sb0")?;
    archive.i32("ib8")?;
    Ok(())
}

fn route_point(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    vector(archive, "unknown_38")?;
    vector(archive, "unknown_50")
}
