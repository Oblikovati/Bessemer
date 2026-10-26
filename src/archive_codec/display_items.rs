use super::item_reader::ItemRead;
use crate::Result;

pub(crate) fn list(archive: &mut impl ItemRead) -> Result<()> {
    let word = archive.u16("count_word")?;
    let count = if word == u16::MAX {
        archive.u32("count_dword")?
    } else {
        u32::from(word)
    };
    counted(archive, count as usize)
}

pub(crate) fn counted(archive: &mut impl ItemRead, count: usize) -> Result<()> {
    if count > archive.remaining() / 8 {
        return Err(archive.invalid(format!(
            "display item count {count}; expected records fitting {} bytes",
            archive.remaining()
        )));
    }
    archive.entries(count, "display items")?;
    for index in 0..count {
        archive.scope(&format!("items[{index}]"), item)?;
    }
    Ok(())
}

fn item(archive: &mut impl ItemRead) -> Result<()> {
    archive.u32("size_of_element")?;
    let kind = archive.i32("type")?;
    archive.scope("data", |archive| payload(archive, kind))
}

fn payload(archive: &mut impl ItemRead, kind: i32) -> Result<()> {
    match kind {
        1..=6 | 13 | 28 | 30 | 31 | 33 | 100..=101 => {
            super::display_item_geometry::read(archive, kind)
        }
        7 | 8 | 10 | 14 | 15 | 18 | 19 => super::display_item_text::read(archive, kind),
        9 | 12 => super::display_item_style::read(archive, kind),
        11 => archive.text("hyperlink"),
        23 => super::display_item_annotation::table(archive),
        24 | 32 | 116 => super::display_item_annotation::property(archive),
        25 | 26 => archive.u32("unknown_00").map(|_| ()),
        29 => super::display_item_annotation::start(archive),
        108 => archive.f32("z_pos"),
        113 => watermark(archive),
        // SerializeElement's default switch arm writes no payload, regardless of size_of_element.
        _ => Ok(()),
    }
}

fn watermark(archive: &mut impl ItemRead) -> Result<()> {
    archive.i32s(&["unknown_0", "unknown_4"])?;
    archive.f32("unknown_8")
}

pub(crate) fn point(archive: &mut impl ItemRead, label: &str) -> Result<()> {
    for index in 0..3 {
        archive.f64(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(crate) fn points(archive: &mut impl ItemRead, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        point(archive, &format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(crate) fn words(archive: &mut impl ItemRead, label: &str, count: usize) -> Result<()> {
    if count > archive.remaining() / 4 {
        return Err(archive.invalid(format!(
            "{label} count {count}; expected u32 values fitting {} bytes",
            archive.remaining()
        )));
    }
    archive.entries(count, label)?;
    for index in 0..count {
        archive.u32(&format!("{label}[{index}]"))?;
    }
    Ok(())
}
