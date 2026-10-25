use super::{archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn list(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let word = archive.u16("count_word")?;
    let count = if word == u16::MAX {
        archive.u32("count_dword")?
    } else {
        u32::from(word)
    };
    counted(archive, count as usize)
}

pub(super) fn counted(archive: &mut DisplayDecoder<'_>, count: usize) -> Result<()> {
    if count > archive.cursor.remaining() / 8 {
        return Err(invalid(format!(
            "display item count {count}; expected records fitting {} bytes",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count, "display items")?;
    for index in 0..count {
        archive.scope(&format!("items[{index}]"), item)?;
    }
    Ok(())
}

fn item(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.u32("size_of_element")?;
    let kind = archive.i32("type")?;
    archive.scope("data", |archive| payload(archive, kind))
}

fn payload(archive: &mut DisplayDecoder<'_>, kind: i32) -> Result<()> {
    match kind {
        1..=6 | 13 | 28 | 30 | 31 | 33 | 100..=101 => {
            super::modern_item_geometry::read(archive, kind)
        }
        7 | 8 | 10 | 14 | 15 | 18 | 19 => super::modern_item_text::read(archive, kind),
        9 | 12 => super::modern_item_style::read(archive, kind),
        11 => archive.text("hyperlink"),
        23 => super::modern_item_annotation::table(archive),
        24 | 32 | 116 => super::modern_item_annotation::property(archive),
        25 | 26 => archive.u32("unknown_00").map(|_| ()),
        29 => super::modern_item_annotation::start(archive),
        108 => archive.f32("z_pos"),
        113 => watermark(archive),
        // SerializeElement's default switch arm writes no payload, regardless of size_of_element.
        _ => Ok(()),
    }
}

fn watermark(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&["unknown_0", "unknown_4"])?;
    archive.f32("unknown_8")
}

pub(super) fn point(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..3 {
        archive.f64(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn points(archive: &mut DisplayDecoder<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        point(archive, &format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn words(archive: &mut DisplayDecoder<'_>, label: &str, count: usize) -> Result<()> {
    if count > archive.cursor.remaining() / 4 {
        return Err(invalid(format!(
            "{label} count {count}; expected u32 values fitting {} bytes",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count, label)?;
    for index in 0..count {
        archive.u32(&format!("{label}[{index}]"))?;
    }
    Ok(())
}
