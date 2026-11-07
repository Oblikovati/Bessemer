use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::MaterialRead;

/// Retain native bulk-array bytes as raw words without applying transform semantics.
/// Example: `pattern_transform_words::read(reader)?` for the sketch-pattern array.
pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let width = reader.u32("element_width")? as usize;
    if width == 0 {
        return Ok(());
    }
    if !width.is_multiple_of(120) {
        return reader.checked(Err(super::invalid(format!(
            "pattern transform element width {width}; expected zero or a multiple of 120"
        ))));
    }
    read_words(reader, width)
}

fn read_words(reader: &mut ModelDecoder<'_>, width: usize) -> Result<()> {
    reader.u32("capacity")?;
    reader.u32("growth")?;
    let count = reader.unsigned_count("count", width)?;
    let words = reader.checked(count.checked_mul(width / 4).ok_or_else(|| {
        super::invalid(format!(
            "pattern transform count {count} width {width}; expected fitting u32-word extent"
        ))
    }))?;
    reader.entries(words, "pattern bulk-transform words")?;
    for index in 0..words {
        reader.u32(&format!("words[{index}]"))?;
    }
    Ok(())
}
