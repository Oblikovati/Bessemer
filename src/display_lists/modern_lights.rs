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
