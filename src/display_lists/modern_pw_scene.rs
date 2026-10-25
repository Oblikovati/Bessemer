use super::{
    archive_decode::DisplayDecoder,
    modern_scene::{double_vector, wall_count},
};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
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

fn wall_info_map(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = wall_count(archive, "wall_info.count", 22)?;
    for index in 0..count {
        archive.scope(&format!("wall_info[{index}]"), |archive| {
            archive.i32("key")?;
            archive.scope("value", wall_info)
        })?;
    }
    Ok(())
}

fn dimensions(archive: &mut DisplayDecoder<'_>) -> Result<()> {
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

fn wall_info(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("unknown_8")?;
    archive.text("unknown_10")?;
    archive.i32s(&[
        "unknown_18",
        "unknown_1c",
        "unknown_20",
        "mapping_fit_to_size",
    ])
}

fn background(archive: &mut DisplayDecoder<'_>) -> Result<()> {
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

fn tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&[
        "using_ibl",
        "rotating_envmap_background",
        "background_image_type",
        "background_env_reflectivity",
        "use_background_env",
        "background_fit_to_camera_fov",
    ])?;
    archive.f64s(&[
        "pv_background_image_brightness",
        "pv_rendering_brightness",
        "pv_scene_reflectivity",
    ])?;
    archive.i32s(&["bg_fit_to_sw_graphics_area", "use_sky_shader"])
}
