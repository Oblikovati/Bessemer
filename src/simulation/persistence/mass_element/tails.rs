use super::super::load_reader::BinaryLoadReader;
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    let prefix = "record0.mass";
    reader.words(&format!("{prefix}.three_words"), 3)?;
    reader.texts(&format!("{prefix}.header_text"), 2)?;
    reader.words(&format!("{prefix}.three_flags"), 3)?;
    reader.doubles(&format!("{prefix}.six_doubles"), 6)?;
    reader.words(&format!("{prefix}.six_words"), 6)?;
    reader.doubles(&format!("{prefix}.sixteen_doubles"), 16)?;
    if reader.version >= 890204 {
        reader.selected_word_pairs(prefix)?;
    }
    reader.texts(&format!("{prefix}.text"), 6)?;
    reader.signed(&format!("{prefix}.word410"))?;
    reader.words(&format!("{prefix}.six_tail_words"), 6)
}
