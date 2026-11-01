use super::super::load_reader::BinaryLoadReader;
use crate::{Result, archive_codec::ArchiveCursor, simulation::invalid};

pub(super) fn read(reader: &mut BinaryLoadReader<'_>, maximum_characters: usize) -> Result<()> {
    for index in 0..7 {
        let path = format!("post_info.header[{index}]");
        if matches!(index, 2 | 5 | 6) {
            reader.signed(&path)?;
        } else {
            reader.unsigned(&path)?;
        }
    }
    for group in ["section", "iso"] {
        for index in 0..6 {
            record(reader, &format!("post_info.{group}[{index}]"))?;
        }
    }
    legacy_suffix(reader, maximum_characters)
}

fn record(reader: &mut BinaryLoadReader<'_>, prefix: &str) -> Result<()> {
    reader.signed(&format!("{prefix}.primitive_type"))?;
    reader.signed(&format!("{prefix}.cut_direction"))?;
    for vector in ["normal", "center"] {
        for component in 0..3 {
            reader.unsigned(&format!("{prefix}.{vector}[{component}]"))?;
        }
    }
    reader.unsigned(&format!("{prefix}.shared_scalar"))
}

fn legacy_suffix(reader: &mut BinaryLoadReader<'_>, maximum_characters: usize) -> Result<()> {
    if reader.version != 700002 {
        return Ok(());
    }
    reader.unsigned("legacy.word")?;
    let count = ArchiveCursor::new(&reader.bytes[reader.position()..]).i32()?;
    let count = reader.bounded(count, "legacy.text", maximum_characters, 2)?;
    // bounded() has already restricted the count to the available two-byte units.
    let width = count * 2;
    require_text_budget(width, reader.limits.maximum_text_bytes)?;
    reader.blob("legacy.text", width + 4)
}

fn require_text_budget(width: usize, maximum: usize) -> Result<()> {
    if width <= maximum {
        return Ok(());
    }
    Err(invalid(
        "legacy.text",
        format!("{width} payload bytes; expected <= {maximum}"),
    ))
}
