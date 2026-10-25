use super::{
    archive_decode::DisplayDecoder,
    archive_error::invalid,
    modern_items::{point, words},
};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>, kind: i32) -> Result<()> {
    match kind {
        7 | 15 => text(archive),
        8 => sketch(archive),
        10 | 14 => font(archive),
        18 => simple(archive),
        _ => symbol(archive),
    }
}

fn text(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "unknown_00")?;
    archive.f64s(&["unknown_18", "unknown_20"])?;
    archive.i32s(&["text_position", "unknown_2c"])?;
    let text = archive.text_value("text")?;
    archive.i32("unknown_34")?;
    archive.f64s(&["unknown_38", "unknown_40"])?;
    if archive.i16("has_widths")? != 0 {
        words(archive, "unknown_48", text.text().chars().count())?;
    }
    Ok(())
}

fn sketch(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "unknown_00")?;
    archive.transform("xform")?;
    archive.f64("unknown_a8")?;
    archive.text("text")
}

fn font(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.f64("unknown_00")?;
    archive.i32("unknown_08")?;
    archive.f64s(&["unknown_10", "unknown_18"])?;
    archive.u32("unknown_20")?;
    archive.text("unknown_name")?;
    archive.f64s(&["unknown_28", "unknown_30"])
}

fn simple(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "unknown_00")?;
    if archive.i32("is_3d_text")? != 0 {
        archive.transform("plane_xform")?;
    }
    archive.f64("unknown_20")?;
    archive.i32("is_right_to_left")?;
    archive.text("text")?;
    simple_widths(archive)?;
    if archive.context.version.value() > 0x2000 {
        archive.i32("is_scale_invariant")?;
    }
    if archive.context.version.value() > 0x239a {
        archive.f64("text_scale")?;
    }
    Ok(())
}

fn simple_widths(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.i16("has_widths")? == 0 {
        return Ok(());
    }
    let count = archive.i32("width_count")?;
    if !(0..=1 << 24).contains(&count) {
        return Err(invalid(format!(
            "simple text width count {count}; expected 0..=16777216"
        )));
    }
    words(archive, "unknown_30", count as usize)
}

fn symbol(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "unknown_00")?;
    if archive.i32("is_3d_symbol")? != 0 {
        archive.transform("plane_xform")?;
    }
    archive.f64s(&["unknown_28", "unknown_30"])?;
    archive.text("unknown_name_68")?;
    archive.text("unknown_name_70")?;
    if archive.context.version.value() <= 0x371f {
        return Ok(());
    }
    let count = archive.i32("items.count")?;
    if count < 0 {
        return Err(invalid(format!(
            "symbol item count {count}; expected nonnegative count"
        )));
    }
    super::modern_items::counted(archive, count as usize)
}
