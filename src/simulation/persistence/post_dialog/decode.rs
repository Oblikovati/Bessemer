use super::super::load_reader::BinaryLoadReader;
use super::helpers::{raw_words, raw64};
use crate::{Result, archive_codec::ArchiveCursor, simulation::invalid};

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version < 700335 {
        return Err(invalid(
            "dialog version",
            format!("{}; expected >= 700335", reader.version),
        ));
    }
    initial(reader)?;
    super::super::post_info::read_postinfo(reader, 0x100000)?;
    super::super::post_style::read_poststyle(reader, 512)?;
    if reader.version >= 800004 {
        super::planes::read(reader)?;
        super::middle::read(reader)?;
    }
    super::late::read(reader)
}

fn initial(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    initial_words(reader)?;
    reader.signed("temporary_state_word")?;
    raw_words(reader, "eight_words528_544", 8)?;
    raw64(reader, "four_raw64_548_560", 4)?;
    raw_words(reader, "three_words568_570", 3)?;
    for label in ["text578", "text580"] {
        reader.character_text(label, 1024)?;
    }
    reader.unsigned("word588")?;
    raw64(reader, "sixteen_raw64_590_608", 16)?;
    for label in ["text610", "text618", "text620", "text628", "text630"] {
        reader.character_text(label, 1024)?;
    }
    Ok(())
}

fn initial_words(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    for index in 0..14 {
        let label = format!("fourteen_words0c_28_30_3c_518_51c[{index}]");
        if matches!(index, 1 | 13) {
            reader.signed(&label)?;
        } else {
            reader.unsigned(&label)?;
        }
    }
    Ok(())
}

pub(super) fn equation_kind(reader: &BinaryLoadReader<'_>) -> Result<i32> {
    let remaining = reader.bytes.get(4..).ok_or_else(|| {
        invalid(
            "dialog equation kind",
            format!(
                "{} bytes; expected an initial kind word at byte4",
                reader.bytes.len()
            ),
        )
    })?;
    ArchiveCursor::new(remaining).i32()
}
