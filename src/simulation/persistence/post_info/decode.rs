use super::super::load_reader::BinaryLoadReader;
use crate::Result;

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
    reader.character_text("legacy.text", maximum_characters)
}
