use super::{MaterialRead, material_properties::user_properties, vectors::double_vector};
use crate::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LightPayloadBranch {
    OldDisplay,
    // The model codec remains private until its complete root operation is verified.
    #[cfg(test)]
    ModelFeature,
}

pub(crate) fn ambient(archive: &mut impl MaterialRead, branch: LightPayloadBranch) -> Result<()> {
    if archive.version() <= 0x427 {
        legacy_light_base(archive)?;
    } else {
        current_light_base(archive)?;
    }
    if archive.version() <= 0x17c {
        return Ok(());
    }
    light_names(archive, branch)
}

fn light_names(archive: &mut impl MaterialRead, branch: LightPayloadBranch) -> Result<()> {
    archive.text("id_name")?;
    if archive.version() < 0x407
        || (archive.version() > 0x453 && branch == LightPayloadBranch::OldDisplay)
    {
        archive.text("user_name")?;
    }
    let first = if archive.version() > 0x11a3 {
        "enabled_in_pv_initial"
    } else {
        "enabled_in_pv"
    };
    if archive.version() > 0xcc0 {
        archive.i32(first)?;
    }
    if archive.version() > 0xd01 {
        archive.i32("keep_light_in_scene")?;
    }
    ambient_rendering(archive, branch)
}

fn ambient_rendering(archive: &mut impl MaterialRead, branch: LightPayloadBranch) -> Result<()> {
    if archive.version() > 0x11a3 {
        archive.i32("enabled_in_pv")?;
        archive.f64("pv_brightness")?;
        archive.i32("pv_shadow_enabled")?;
        archive.f64("pv_shadow_quality")?;
    }
    archive.scope("user_property", user_properties)?;
    if archive.version() > 0x11f5 && branch == LightPayloadBranch::OldDisplay {
        archive.i32("id")?;
    }
    Ok(())
}

pub(crate) fn directional(
    archive: &mut impl MaterialRead,
    branch: LightPayloadBranch,
) -> Result<()> {
    ambient(archive, branch)?;
    if archive.version() > 0x427 {
        double_vector(archive, "direction")?;
    }
    if archive.version() >= 0x11a4 {
        archive.f64("pv_shadow_softness")?;
    }
    Ok(())
}

pub(crate) fn spot(archive: &mut impl MaterialRead, branch: LightPayloadBranch) -> Result<()> {
    ambient(archive, branch)?;
    spot_geometry(archive)?;
    spot_rendering(archive)
}

fn spot_rendering(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() < 0x11a4 {
        return Ok(());
    }
    archive.f64s(&["pv_soft_edge", "pv_spot_light_radius"])?;
    archive.i32("pv_fog_enabled")?;
    archive.f64s(&["pv_fog_distance", "pv_fog_quality"])
}

pub(crate) fn point(archive: &mut impl MaterialRead, branch: LightPayloadBranch) -> Result<()> {
    ambient(archive, branch)?;
    if archive.version() <= 0x427 {
        return Ok(());
    }
    double_vector(archive, "position")?;
    if archive.version() <= 0x11a3 {
        return Ok(());
    }
    archive.f64("pv_point_light_radius")?;
    archive.i32("pv_fog_enabled")?;
    archive.f64s(&["pv_fog_radius", "pv_fog_quality"])
}

fn current_light_base(archive: &mut impl MaterialRead) -> Result<()> {
    archive.f64s(&["ambient", "diffuse", "specular"])?;
    archive.u32("color")?;
    archive.i32s(&["suppressed", "in_model_space", "unknown_364"])?;
    if archive.version() >= 0x452 {
        archive.i32("use_cart_display")?;
    }
    Ok(())
}

fn legacy_light_base(archive: &mut impl MaterialRead) -> Result<()> {
    archive.f64s(&[
        "ambient",
        "unknown_old_a8",
        "diffuse",
        "specular",
        "unknown_old_a0",
        "unknown_old_b0",
    ])?;
    for name in ["unknown_old_88", "unknown_old_50", "unknown_old_68"] {
        double_vector(archive, name)?;
    }
    archive.u32("color")?;
    archive.i32("suppressed").map(|_| ())
}

fn spot_geometry(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() <= 0x427 {
        return Ok(());
    }
    double_vector(archive, "position")?;
    double_vector(archive, "target")?;
    archive.f64("cone_angle")?;
    if archive.version() < 0x43f {
        return Ok(());
    }
    archive.f64s(&[
        "exponent",
        "constant_falloff",
        "linear_falloff",
        "quadratic_falloff",
        "unknown_400",
    ])
}
