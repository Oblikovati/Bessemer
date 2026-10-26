use super::scene_reader::SceneRead;
use crate::Result;

pub(crate) fn read<R: SceneRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    archive.i32("key")?;
    let kind = archive.i32("type")?;
    match kind {
        1 => archive.scope("wall", |archive| rectangular(archive, context)),
        2 => archive.scope("wall", |archive| circular(archive, context)),
        3 => archive.scope("wall", |archive| spherical(archive, context)),
        other => Err(archive.invalid(format!("scene wall kind {other}; expected 1, 2 or 3"))),
    }
}

fn base<R: SceneRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    if archive.version() < 0x117f {
        archive.scene_object(context, "pw_material", &["moPWMaterialProperties_c"])?;
        return archive.i32s(&["visible", "reflective"]);
    }
    archive.i32s(&["visible", "reflective"])?;
    archive.scene_object(context, "viz_prop", &["moVisualProperties_c"])?;
    archive.i32("unknown_10")?;
    Ok(())
}

fn rectangular<R: SceneRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    base(archive, context)?;
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

fn circular<R: SceneRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    base(archive, context)?;
    if archive.version() > 0x1827 {
        for index in 0..3 {
            archive.f32(&format!("unknown_20[{index}]"))?;
        }
        archive.f64("radius")?;
    }
    if archive.version() >= 0x1842 {
        archive.f64("unknown_38")?;
    }
    Ok(())
}

fn spherical<R: SceneRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    base(archive, context)?;
    if archive.version() > 0x1827 {
        super::vectors::double_vector(archive, "unknown_20")?;
        archive.f64("radius")?;
    }
    Ok(())
}
