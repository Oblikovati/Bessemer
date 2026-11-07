use super::{MaterialRead, material::float3};
use crate::Result;

pub(crate) fn read(archive: &mut impl MaterialRead) -> Result<()> {
    float3(archive, "albedo")?;
    archive.f32s(&[
        "unknown_28",
        "unknown_2c",
        "unknown_38",
        "unknown_3c",
        "unknown_50",
        "unknown_54",
        "unknown_48",
        "unknown_98",
        "index_of_refraction",
        "unknown_12c",
        "unknown_60",
    ])?;
    surface_colors(archive)
}

fn surface_colors(archive: &mut impl MaterialRead) -> Result<()> {
    float3(archive, "specular_tint")?;
    archive.f32("unknown_7c")?;
    float3(archive, "sheen_color")?;
    archive.f32("unknown_f8")?;
    float3(archive, "emission_color")?;
    archive.f32("unknown_c4")?;
    archive.i32("unknown_cc")?;
    archive.f32s(&["unknown_a8", "unknown_ac"])?;
    float3(archive, "flip_flop_color")?;
    archive.f32s(&["unknown_80", "unknown_90", "unknown_94"])?;
    float3(archive, "flake_color")?;
    archive.f32("unknown_e8")?;
    attenuation_and_textures(archive)
}

fn attenuation_and_textures(archive: &mut impl MaterialRead) -> Result<()> {
    float3(archive, "attenuation_color")?;
    float3(archive, "subsurface_color")?;
    archive.i32("thin_walled")?;
    archive.f32s(&["unknown_4c", "unknown_d8"])?;
    for label in [
        "unknown_20",
        "unknown_40",
        "unknown_30",
        "unknown_58",
        "unknown_8",
    ] {
        archive.text(label)?;
    }
    archive.f32("unknown_128")?;
    tail(archive)
}

fn tail(archive: &mut impl MaterialRead) -> Result<()> {
    if archive.version() > 0x46b4 {
        translucency(archive)?;
    }
    if archive.version() > 0x4971 {
        recent_tail(archive)?;
    }
    Ok(())
}

fn translucency(archive: &mut impl MaterialRead) -> Result<()> {
    archive.text("unknown_0")?;
    archive.text("unknown_b0")?;
    archive.i32("unknown_c8")?;
    archive.f32s(&["unknown_fc", "unknown_100", "unknown_104"])?;
    float3(archive, "translucency_color")?;
    for label in ["unknown_118", "unknown_120"] {
        archive.text(label)?;
    }
    Ok(())
}

fn recent_tail(archive: &mut impl MaterialRead) -> Result<()> {
    archive.f32s(&["unknown_158", "unknown_15c"])?;
    for label in ["unknown_140", "unknown_148", "unknown_150"] {
        archive.text(label)?;
    }
    Ok(())
}
