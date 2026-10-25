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
    light_names(archive)
}

fn light_names(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("id_name")?;
    archive.text("user_name")?;
    let first = if archive.context.version.value() > 0x11a3 {
        "enabled_in_pv_initial"
    } else {
        "enabled_in_pv"
    };
    archive.i32(first)?;
    archive.i32("keep_light_in_scene")?;
    ambient_rendering(archive)
}

fn ambient_rendering(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x11a3 {
        archive.i32("enabled_in_pv")?;
        archive.f64("pv_brightness")?;
        archive.i32("pv_shadow_enabled")?;
        archive.f64("pv_shadow_quality")?;
    }
    archive.scope("user_property", user_properties)?;
    if archive.context.version.value() > 0x11f5 {
        archive.i32("id")?;
    }
    Ok(())
}

pub(super) fn directional(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    ambient(archive)?;
    super::modern_scene::double_vector(archive, "direction")?;
    if archive.context.version.value() >= 0x11a4 {
        archive.f64("pv_shadow_softness")?;
    }
    Ok(())
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
    ])?;
    spot_rendering(archive)
}

fn spot_rendering(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0x11a4 {
        return Ok(());
    }
    archive.f64s(&["pv_soft_edge", "pv_spot_light_radius"])?;
    archive.i32("pv_fog_enabled")?;
    archive.f64s(&["pv_fog_distance", "pv_fog_quality"])
}

pub(super) fn point(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    ambient(archive)?;
    super::modern_scene::double_vector(archive, "position")?;
    if archive.context.version.value() <= 0x11a3 {
        return Ok(());
    }
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
