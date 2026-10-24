use super::archive_decode::DisplayDecoder;
use crate::Result;

pub(super) fn environment(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("user_name")?;
    archive.scope("splitter_10", splitter)?;
    archive.i32s(&["unknown_1b0", "browser_view_width"])?;
    archive.scope("splitter_e0", splitter)?;
    archive.i32s(&["unknown_1b8", "unknown_1bc", "unknown_1c0", "unknown_1c4"])
}

fn splitter(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let rows = usize::from(archive.u16("rows")?);
    let cols = usize::from(archive.u16("cols")?);
    splitter_header(archive)?;
    let count = rows * cols;
    if count > 4 {
        return Err(super::archive_error::invalid(format!(
            "splitter {rows}x{cols}; expected at most four panes"
        )));
    }
    archive.entries(count, "splitter panes")?;
    for index in 0..count {
        archive.scope(&format!("panes[{index}]"), pane)?;
    }
    Ok(())
}

fn pane(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.transform("unknown_28")?;
    archive.transform("pan_zoom_xform")?;
    archive.u16("display_mode")?;
    archive.u16("hlr_display_quality")?;
    archive.f64("perspective")?;
    archive.text("unknown_a8")
}

pub(super) fn fonts(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.u16("line_fonts.count")? as usize;
    archive.entries(count, "line fonts")?;
    for index in 0..count {
        archive.scope(&format!("line_fonts[{index}]"), |archive| {
            archive.text("key")?;
            archive.object("value", &["uiLFConfig_c"])
        })?;
    }
    Ok(())
}

pub(super) fn font(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("name")?;
    archive.i16("standard_width")?;
    archive.f32("custom_width")?;
    archive.i16("end_cap")?;
    Ok(())
}

fn splitter_header(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    for label in [
        "unknown_10",
        "unknown_14",
        "unknown_18",
        "unknown_1c",
        "unknown_20",
        "unknown_24",
    ] {
        archive.u16(label)?;
    }
    Ok(())
}
