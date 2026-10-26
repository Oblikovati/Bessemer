use super::display_items::{point, words};
use super::item_reader::ItemRead;
use crate::Result;

pub(crate) fn read(archive: &mut impl ItemRead, kind: i32) -> Result<()> {
    match kind {
        7 | 15 => text(archive),
        8 => sketch(archive),
        10 | 14 => font(archive),
        18 => simple(archive),
        _ => symbol(archive),
    }
}

fn text(archive: &mut impl ItemRead) -> Result<()> {
    point(archive, "unknown_00")?;
    archive.f64s(&["unknown_18", "unknown_20"])?;
    archive.i32s(&["text_position", "unknown_2c"])?;
    let text = archive.item_text_value("text")?;
    if archive.version() > 0x72a {
        archive.i32("unknown_34")?;
        archive.f64s(&["unknown_38", "unknown_40"])?;
    }
    if archive.version() > 0xa03 && archive.i16("has_widths")? != 0 {
        words(archive, "unknown_48", text.text().chars().count())?;
    }
    Ok(())
}

fn sketch(archive: &mut impl ItemRead) -> Result<()> {
    point(archive, "unknown_00")?;
    archive.graph_transform("xform")?;
    if archive.version() > 0x52d {
        archive.f64("unknown_a8")?;
    }
    archive.text("text")
}

fn font(archive: &mut impl ItemRead) -> Result<()> {
    archive.f64("unknown_00")?;
    archive.i32("unknown_08")?;
    archive.f64s(&["unknown_10", "unknown_18"])?;
    archive.u32("unknown_20")?;
    archive.text("unknown_name")?;
    if archive.version() > 0x52e {
        archive.f64("unknown_28")?;
    }
    if archive.version() > 0x7df {
        archive.f64("unknown_30")?;
    }
    Ok(())
}

fn simple(archive: &mut impl ItemRead) -> Result<()> {
    point(archive, "unknown_00")?;
    if archive.version() > 0xb74 && archive.i32("is_3d_text")? != 0 {
        archive.graph_transform("plane_xform")?;
    }
    archive.f64("unknown_20")?;
    archive.i32("is_right_to_left")?;
    let text = archive.item_text_value("text")?;
    simple_widths(archive, text.text())?;
    simple_version_tail(archive)
}

fn simple_version_tail(archive: &mut impl ItemRead) -> Result<()> {
    if (0x1448..0x144a).contains(&archive.version()) {
        archive.i32("unknown_v1448")?;
    }
    // #37: the native reader discards this float, but exact writes must retain its four bytes.
    if (0x1c05..0x1fb5).contains(&archive.version()) {
        archive.f32("unknown_v1c05")?;
    }
    if archive.version() > 0x2000 {
        archive.i32("is_scale_invariant")?;
    }
    if archive.version() > 0x239a {
        archive.f64("text_scale")?;
    }
    Ok(())
}

fn simple_widths(archive: &mut impl ItemRead, text: &str) -> Result<()> {
    if archive.i16("has_widths")? == 0 {
        return Ok(());
    }
    let count = if archive.version() > 0xaeb {
        i128::from(archive.i32("width_count")?)
    } else {
        text.chars().count() as i128
    };
    if !(0..=1 << 24).contains(&count) {
        return Err(archive.invalid(format!(
            "simple text width count {count}; expected 0..=16777216"
        )));
    }
    words(archive, "unknown_30", count as usize)
}

fn symbol(archive: &mut impl ItemRead) -> Result<()> {
    point(archive, "unknown_00")?;
    if archive.version() > 0xbbd && archive.i32("is_3d_symbol")? != 0 {
        archive.graph_transform("plane_xform")?;
    }
    archive.f64s(&["unknown_28", "unknown_30"])?;
    archive.text("unknown_name_68")?;
    archive.text("unknown_name_70")?;
    if archive.version() <= 0x371f {
        return Ok(());
    }
    let count = archive.i32("items.count")?;
    if count < 0 {
        return Err(archive.invalid(format!(
            "symbol item count {count}; expected nonnegative count"
        )));
    }
    super::display_items::counted(archive, count as usize)
}
