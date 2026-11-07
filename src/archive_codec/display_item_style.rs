use super::item_reader::ItemRead;
use crate::Result;

pub(crate) fn read(archive: &mut impl ItemRead, kind: i32) -> Result<()> {
    if kind == 12 {
        return user_pen(archive);
    }
    archive.i32s(&["unknown_00", "unknown_04"])?;
    archive.u32("unknown_08")?;
    archive.i32("line_style")?;
    archive.scope("line_width", width)?;
    archive.i32("line_type")?;
    if archive.version() > 0xca5 {
        archive.i32("end_cap")?;
    }
    if archive.version() >= 0x185a {
        archive.i32("unknown_20")?;
    }
    Ok(())
}

pub(crate) fn width(archive: &mut impl ItemRead) -> Result<()> {
    crate::archive_codec::line_style::width(archive)
}

fn user_pen(archive: &mut impl ItemRead) -> Result<()> {
    archive.u32("color")?;
    archive.scope("width", width)?;
    if archive.version() > 0xc57 {
        archive.i32("layer_id")?;
    }
    if archive.version() >= 0xca6 {
        archive.i32("end_cap_style")?;
    }
    archive.text("style")
}
