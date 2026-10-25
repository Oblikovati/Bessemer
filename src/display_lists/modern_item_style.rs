use super::archive_decode::DisplayDecoder;
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>, kind: i32) -> Result<()> {
    if kind == 12 {
        return user_pen(archive);
    }
    archive.i32s(&["unknown_00", "unknown_04"])?;
    archive.u32("unknown_08")?;
    archive.i32("line_style")?;
    archive.scope("line_width", width)?;
    archive.i32("line_type")?;
    if archive.context.version.value() > 0xca5 {
        archive.i32("end_cap")?;
    }
    if archive.context.version.value() >= 0x185a {
        archive.i32("unknown_20")?;
    }
    Ok(())
}

pub(super) fn width(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0xf41 {
        return archive.i32("standard_width").map(|_| ());
    }
    short_width(archive)
}

pub(super) fn short_width(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i16("standard_width")?;
    if archive.context.version.value() >= 0xf41 {
        archive.f32("custom_width")?;
    }
    Ok(())
}

fn user_pen(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.u32("color")?;
    archive.scope("width", width)?;
    if archive.context.version.value() > 0xc57 {
        archive.i32("layer_id")?;
    }
    if archive.context.version.value() >= 0xca6 {
        archive.i32("end_cap_style")?;
    }
    archive.text("style")
}
