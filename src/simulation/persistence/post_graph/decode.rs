use super::super::load_reader::BinaryLoadReader;
use crate::{Result, archive_codec::ArchiveCursor, simulation::invalid};

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version < 800018 {
        return Err(invalid(
            "graph version",
            format!("{}; expected >= 800018", reader.version),
        ));
    }
    reader.words("twelve_words08_34", 12)?;
    if reader.version >= 870002 {
        reader.signed("word38")?;
    }
    reader.words("three_words3c_44", 3)?;
    if reader.version >= 870002 {
        reader.signed("word48")?;
    }
    for label in ["textb0", "textb8", "textc0"] {
        character_text(reader, label)?;
    }
    reader.words("six_words4c_60", 6)?;
    reader.words("eighteen_words64_a8", 18)?;
    continuations(reader)
}

fn continuations(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 800026 {
        reader.signed("wordc8")?;
        character_text(reader, "textd0")?;
        raw64_array(reader, "sixteen_raw64_d8_150", 16)?;
    }
    if reader.version >= 800062 {
        reader.words("words158_160", 3)?;
    }
    if reader.version >= 820022 {
        mixed_820022(reader)?;
    }
    late_fields(reader)
}

fn late_fields(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 840028 {
        reader.signed("word190")?;
    }
    if reader.version >= 890005 {
        reader.words("six_words198_1ac_raw64_1b0.words", 6)?;
        reader.blob("six_words198_1ac_raw64_1b0.raw64_1b0", 8)?;
    }
    if reader.version >= 890110 {
        reader.signed("word1f8")?;
    }
    Ok(())
}

fn mixed_820022(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.signed("word16c_two_raw64_words180_184.word16c")?;
    raw64_array(reader, "word16c_two_raw64_words180_184.raw64", 2)?;
    reader.words("word16c_two_raw64_words180_184.words180_184", 2)
}

fn raw64_array(reader: &mut BinaryLoadReader<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        reader.blob(&format!("{label}[{index}]"), 8)?;
    }
    Ok(())
}

fn character_text(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    let count = ArchiveCursor::new(&reader.bytes[reader.position()..]).i32()?;
    let count = reader.bounded(count, label, 1024, 2)?;
    let width = count * 2;
    if width > reader.limits.maximum_text_bytes {
        return Err(invalid(
            label,
            format!(
                "{width} payload bytes; expected <= {}",
                reader.limits.maximum_text_bytes
            ),
        ));
    }
    reader.blob(label, width + 4)
}
