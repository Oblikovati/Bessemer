use super::{archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn ambient(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.f64s(&["ambient", "diffuse", "specular"])?;
    archive.u32("color")?;
    archive.i32s(&[
        "suppressed",
        "in_model_space",
        "unknown_364",
        "use_cart_display",
    ])?;
    archive.text("id_name")?;
    archive.text("user_name")?;
    archive.i32s(&[
        "enabled_in_pv_initial",
        "keep_light_in_scene",
        "enabled_in_pv",
    ])?;
    ambient_rendering(archive)
}

fn ambient_rendering(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.f64("pv_brightness")?;
    archive.i32("pv_shadow_enabled")?;
    archive.f64("pv_shadow_quality")?;
    archive.scope("user_property", user_properties)?;
    archive.i32("id")?;
    Ok(())
}

pub(super) fn directional(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    ambient(archive)?;
    archive.f64s(&[
        "direction[0]",
        "direction[1]",
        "direction[2]",
        "pv_shadow_softness",
    ])
}

pub(super) fn spot(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    ambient(archive)?;
    super::modern_scene::double_vector(archive, "position")?;
    super::modern_scene::double_vector(archive, "target")?;
    archive.f64s(&[
        "cone_angle",
        "exponent",
        "constant_falloff",
        "linear_falloff",
        "quadratic_falloff",
        "unknown_400",
        "pv_soft_edge",
        "pv_spot_light_radius",
    ])?;
    archive.i32("pv_fog_enabled")?;
    archive.f64s(&["pv_fog_distance", "pv_fog_quality"])
}

pub(super) fn point(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    ambient(archive)?;
    super::modern_scene::double_vector(archive, "position")?;
    archive.f64("pv_point_light_radius")?;
    archive.i32("pv_fog_enabled")?;
    archive.f64s(&["pv_fog_radius", "pv_fog_quality"])
}

pub(super) fn user_properties(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let raw = archive.u32("count")?;
    let count = if raw >= 0x8000_0000 { 0 } else { raw as usize };
    archive.entries(count, "user properties")?;
    if count > archive.cursor.remaining() / 4 {
        return Err(invalid(format!(
            "user property count {count}; expected entries fitting remaining bytes"
        )));
    }
    for index in 0..count {
        archive.scope(&format!("entries[{index}]"), property)?;
    }
    Ok(())
}

fn property(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    match archive.u32("type")? {
        0 => archive.f32("value"),
        1 => {
            archive.i32("value")?;
            Ok(())
        }
        2 => archive.text("value"),
        _ => Ok(()),
    }
}
