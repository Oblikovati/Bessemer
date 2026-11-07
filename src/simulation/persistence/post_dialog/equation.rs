use super::super::load_reader::BinaryLoadReader;
use super::helpers::{count, offsets, raw_words};
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.character_text("equation.text8", 1024)?;
    offsets(reader, "equation", &[0x10])?;
    reader.character_text("equation.text18", 1024)?;
    reader.character_text("equation.text20", 1024)?;
    let count = count(reader, "equation.variables")?;
    for index in 0..count {
        let label = format!("equation.variable[{index}]");
        reader.character_text(&format!("{label}.text8"), 1024)?;
        raw_words(reader, &format!("{label}.two_words_raw64.words"), 2)?;
        reader.blob(&format!("{label}.two_words_raw64.raw64"), 8)?;
    }
    Ok(())
}
