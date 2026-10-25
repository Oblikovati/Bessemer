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
    archive.i32s(&["line_type", "end_cap", "unknown_20"])
}

fn width(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i16("standard_width")?;
    archive.f32("custom_width")
}

fn user_pen(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.u32("color")?;
    archive.scope("width", width)?;
    archive.i32s(&["layer_id", "end_cap_style"])?;
    archive.text("style")
}
