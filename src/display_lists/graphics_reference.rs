use super::{archive_decode::DisplayDecoder, modern_scene::double_vector};
use crate::Result;

pub(super) fn manager(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("unknown_8")?;
    if archive.i32("has_graphics_ground_face_data")? != 0 {
        archive.cobject("graphics_ground_face_data")?;
    }
    let count = archive.count("all_graphics_connect_pnts.count", 2)?;
    for index in 0..count {
        archive.cobject(&format!("all_graphics_connect_pnts[{index}]"))?;
    }
    Ok(())
}

pub(super) fn magnetic(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("unknown_8")?;
    archive.cobject("unknown_10")?;
    archive.cobject("unknown_18")?;
    archive.text("unknown_20")?;
    if archive.context.version.value() > 0x2bc3 {
        archive.f64("unknown_28")?;
    }
    Ok(())
}

pub(super) fn wrapper(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("unknown_8")?;
    archive.cobject("unknown_50")?;
    if archive.i32("has_unknown_18")? != 0 {
        archive.scope("unknown_18", plane)?;
    }
    Ok(())
}

fn plane(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    // mgPlane_c stores six doubles, excluding its native vtable; section planes have a different wire body.
    double_vector(archive, "root_point")?;
    double_vector(archive, "normal")
}
