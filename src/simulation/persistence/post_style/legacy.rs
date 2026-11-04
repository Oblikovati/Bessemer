use super::super::load_reader::BinaryLoadReader;
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    raw_words(reader, "v7.prefix", &[0, 4, 8, 12, 16])?;
    super::decode::colors(reader, "v7.prefix", &[20, 36, 52, 68, 84, 100, 116])?;
    raw_words(
        reader,
        "v7.prefix",
        &[132, 136, 140, 144, 148, 152, 156, 160, 164],
    )?;
    raw_words(reader, "v7.legend", &[0, 4, 8, 12])?;
    super::decode::colors(
        reader,
        "v7.legend",
        &[16, 32, 48, 64, 80, 96, 112, 128, 144],
    )?;
    raw_words(reader, "v7.legend", &[0xa0])?;
    raw_words(reader, "v7.style", &[0x14c])?;
    super::decode::colors(reader, "v7.style", &[0x150, 0x160])
}

fn raw_words(reader: &mut BinaryLoadReader<'_>, prefix: &str, offsets: &[u32]) -> Result<()> {
    for offset in offsets {
        reader.unsigned(&format!("{prefix}.word_{offset:x}"))?;
    }
    Ok(())
}
