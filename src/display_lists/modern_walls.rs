use super::{archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("key")?;
    let kind = archive.i32("type")?;
    match kind {
        1 => archive.scope("wall", rectangular),
        2 => archive.scope("wall", circular),
        3 => archive.scope("wall", spherical),
        other => Err(invalid(format!(
            "scene wall kind {other}; expected 1, 2 or 3"
        ))),
    }
}

fn base(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0x117f {
        archive.object("pw_material", &["moPWMaterialProperties_c"])?;
        return archive.i32s(&["visible", "reflective"]);
    }
    archive.i32s(&["visible", "reflective"])?;
    archive.object("viz_prop", &["moVisualProperties_c"])?;
    archive.i32("unknown_10")?;
    Ok(())
}

fn rectangular(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    base(archive)?;
    for row in 0..4 {
        for column in 0..3 {
            archive.f32(&format!("unknown_20[{row}][{column}]"))?;
        }
    }
    for index in 0..3 {
        archive.f32s(&[
            &format!("unknown_50_5c[{index}][0]"),
            &format!("unknown_50_5c[{index}][1]"),
        ])?;
    }
    Ok(())
}

fn circular(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    base(archive)?;
    if archive.context.version.value() > 0x1827 {
        for index in 0..3 {
            archive.f32(&format!("unknown_20[{index}]"))?;
        }
        archive.f64("radius")?;
    }
    if archive.context.version.value() >= 0x1842 {
        archive.f64("unknown_38")?;
    }
    Ok(())
}

fn spherical(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    base(archive)?;
    if archive.context.version.value() > 0x1827 {
        super::modern_scene::double_vector(archive, "unknown_20")?;
        archive.f64("radius")?;
    }
    Ok(())
}
