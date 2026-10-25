use super::{archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.scope("concept", super::modern_concept::read)?;
    archive.object("pw_scene_data", &["pwSceneData_c"])?;
    let count = wall_count(archive, "walls.count", 8)?;
    for index in 0..count {
        archive.scope(&format!("walls[{index}]"), super::modern_walls::read)?;
    }
    archive.bounds("bbox")?;
    archive.i32("floor_orient")?;
    archive.transform("required_xform")?;
    archive.cobject("floor_selection")?;
    floor(archive)
}

fn floor(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("floor_dir")?;
    archive.f64("floor_offset")?;
    archive.i32("floor_offset_dir")?;
    archive.f64s(&["floor_rotation", "env_rotation"])?;
    archive.i32("transparent_shadow_floor")?;
    archive.f32("floor_reflectivity")?;
    archive.i32s(&[
        "resize_automatic",
        "ambient_occlusion",
        "background_fit_to_camera_fov",
        "flags",
        "flatten_floor",
    ])?;
    archive.f64s(&["environment_size", "flattened_floor_height"])?;
    archive.i32("accurate_environment_lighting")?;
    archive.transform("flatten_floor_xform")?;
    field_of_view(archive)
}

fn field_of_view(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x42ec {
        archive.f64("hdr_exposure")?;
        archive.i32("dspbr_tone_map")?;
    }
    if archive.context.version.value() > 0x46ba {
        archive.f64("field_of_view")?;
    }
    Ok(())
}

pub(super) fn wall_count(
    archive: &mut DisplayDecoder<'_>,
    label: &str,
    minimum: usize,
) -> Result<usize> {
    let stored = archive.i32(label)?;
    let count = stored.max(0) as usize;
    if count > archive.cursor.remaining() / minimum {
        return Err(invalid(format!(
            "{label} count {stored}; expected nonpositive empty or entries fitting {} bytes",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count, label)?;
    Ok(count)
}

pub(super) fn double_vector(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..3 {
        archive.f64(&format!("{label}[{index}]"))?;
    }
    Ok(())
}
