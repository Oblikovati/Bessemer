use super::{MaterialRead, vectors::double_vector};
use crate::Result;

pub(crate) fn read(archive: &mut impl MaterialRead) -> Result<()> {
    for label in ["scene_file_name", "display_name", "light_file"] {
        archive.text(label)?;
    }
    archive.i32s(&[
        "shadow_type",
        "shadow_softness",
        "shadow_quality",
        "environment_type",
    ])?;
    wall_info_map(archive)?;
    dimensions(archive)
}

fn wall_info_map(archive: &mut impl MaterialRead) -> Result<()> {
    // #41: generic thread-map keys reach PW bodies before root scenes introduce them.
    let version = archive.version();
    let minimum = 14 + 4 * usize::from(version >= 0xdf8) + 4 * usize::from(version >= 0xe11);
    let count = nonnegative_count(archive, "wall_info.count", minimum)?;
    for index in 0..count {
        archive.scope(&format!("wall_info[{index}]"), |archive| {
            if archive.version() >= 0xdf8 {
                archive.i32("key")?;
            }
            archive.scope("value", wall_info)
        })?;
    }
    Ok(())
}

fn dimensions(archive: &mut impl MaterialRead) -> Result<()> {
    archive.f64s(&[
        "floor_offset",
        "scene_length",
        "scene_width",
        "scene_height",
    ])?;
    archive.i32s(&[
        "fixed_aspect_ratio",
        "auto_resize",
        "alignment",
        "background_type",
    ])?;
    background(archive)
}

fn wall_info(archive: &mut impl MaterialRead) -> Result<()> {
    archive.text("unknown_8")?;
    archive.text("unknown_10")?;
    archive.i32s(&["unknown_18", "unknown_1c", "unknown_20"])?;
    if archive.version() >= 0xe11 {
        archive.i32("mapping_fit_to_size")?;
    }
    Ok(())
}

fn background(archive: &mut impl MaterialRead) -> Result<()> {
    double_vector(archive, "background_color1")?;
    double_vector(archive, "background_color2")?;
    archive.text("background_image")?;
    archive.f64("background_multiplier")?;
    archive.i32s(&["background_mapping", "background_env_type"])?;
    double_vector(archive, "background_env_color1")?;
    double_vector(archive, "background_env_color2")?;
    archive.text("background_env_image")?;
    archive.f64s(&[
        "background_env_diff_multiplier",
        "background_env_refl_multiplier",
    ])?;
    archive.i32s(&["background_env_mapping", "foreground_type"])?;
    double_vector(archive, "sky_color")?;
    archive.f64s(&["start_distance", "opaque_distance"])?;
    tail(archive)
}

fn tail(archive: &mut impl MaterialRead) -> Result<()> {
    archive.i32s(&["using_ibl", "rotating_envmap_background"])?;
    if archive.version() > 0xe0b {
        archive.i32s(&[
            "background_image_type",
            "background_env_reflectivity",
            "use_background_env",
        ])?;
    }
    if archive.version() > 0xf39 {
        archive.i32("background_fit_to_camera_fov")?;
    }
    historical_rendering(archive)
}

fn historical_rendering(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0x1194 {
        archive.f64s(&[
            "pv_background_image_brightness",
            "pv_rendering_brightness",
            "pv_scene_reflectivity",
        ])?;
    }
    if archive.version() > 0x1310 {
        archive.i32("bg_fit_to_sw_graphics_area")?;
    }
    if archive.version() >= 0x17dd {
        archive.i32("use_sky_shader")?;
    }
    Ok(())
}

pub(crate) fn nonnegative_count(
    archive: &mut impl MaterialRead,
    label: &str,
    minimum: usize,
) -> Result<usize> {
    let stored = archive.i32(label)?;
    let count = stored.max(0) as usize;
    let capacity = archive.remaining().checked_div(minimum).ok_or_else(|| {
        archive.invalid(format!(
            "{label} minimum {minimum}; expected nonzero entry bytes"
        ))
    })?;
    if count > capacity {
        return Err(archive.invalid(format!(
            "{label} count {stored}; expected nonpositive empty or entries fitting {} bytes",
            archive.remaining()
        )));
    }
    archive.entries(count, label)?;
    Ok(count)
}
