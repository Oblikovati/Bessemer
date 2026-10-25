use super::{archive_decode::DisplayDecoder, archive_error::invalid, modern_material::float3};
use crate::Result;

pub(super) fn list(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    if archive.context.version.value() <= 0xfd7 {
        return Ok(());
    }
    let count = unsigned_count(archive, &format!("{label}.count"), 1)?;
    for index in 0..count {
        archive.scope(&format!("{label}.items[{index}]"), decal)?;
    }
    Ok(())
}

pub(super) fn part(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0x239d {
        return list(archive, "decals");
    }
    let count = unsigned_count(archive, "decals_map.count", 8)?;
    for index in 0..count {
        archive.scope(&format!("decals_map[{index}]"), |archive| {
            archive.i32("key")?;
            list(archive, "value")
        })?;
    }
    Ok(())
}

fn unsigned_count(archive: &mut DisplayDecoder<'_>, label: &str, minimum: usize) -> Result<usize> {
    let count = archive.u32(label)?;
    if u64::from(count) > archive.cursor.remaining() as u64 / minimum as u64 {
        return Err(invalid(format!(
            "{label} count {count}; expected entries fitting {} bytes at {minimum} bytes per entry",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count as usize, label)?;
    Ok(count as usize)
}

fn decal(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.u8("unknown_0")?;
    for label in ["u_dir", "v_dir", "center"] {
        float3(archive, label)?;
    }
    archive.f32("face_width")?;
    archive.text("mask_filename")?;
    archive.u8("mask_type")?;
    archive.i32("mask_invert")?;
    let count = unsigned_count(archive, "color_mask.count", 4)?;
    for index in 0..count {
        archive.u32(&format!("color_mask[{index}]"))?;
    }
    properties(archive)
}

fn properties(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.scope("texture_properties", super::modern_material::texture)?;
    archive.scope("material_properties", super::modern_material::material)?;
    let count = archive.count("decal_faces.count", 16)?;
    for index in 0..count {
        archive.scope(&format!("decal_faces[{index}]"), face)?;
    }
    tail(archive)
}

fn face(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.u32("face_id")?;
    float3(archive, "unknown_10")?;
    if archive.context.version.value() > 0x12db {
        for index in 0..3 {
            archive.f32(&format!("unknown_1c[{index}]"))?;
        }
        archive.i32("unknown_1c[3]")?;
    }
    Ok(())
}

fn tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x1197 {
        archive.i32("use_underlying_appearance")?;
    }
    if archive.context.version.value() > 0x11bb {
        archive.i32("orig_decal_id")?;
    }
    if archive.context.version.value() > 0x1bbe {
        archive.f32s(&["mirror_offset_x", "mirror_offset_y"])?;
    }
    Ok(())
}
