use super::super::load_reader::BinaryLoadReader;
use super::helpers::{color, offsets};
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    for (gate, words) in [
        (0xd9497, &[0xef8][..]),
        (0xd949d, &[0xf04][..]),
        (0xd94fa, &[0xae0, 0xadc][..]),
    ] {
        if reader.version > gate {
            offsets(reader, "dialog", words)?;
        }
    }
    if reader.version > 0xd94fc && super::decode::equation_kind(reader)? == 0x41 {
        super::equation::read(reader)?;
    }
    if reader.version < 0xd9506 {
        return Ok(());
    }
    offsets(reader, "dialog", &[0xf90])?;
    extended(reader)
}

fn extended(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version <= 0xd955d {
        return Ok(());
    }
    offsets(reader, "dialog", &[0xf94])?;
    if reader.version > 0xd9563 {
        offsets(reader, "dialog", &[0xf98, 0xf9c])?;
        reader.character_text("dialog.text_fa8", 1024)?;
        reader.character_text("dialog.text_fa0", 1024)?;
    }
    if reader.version > 0xd95c2 {
        offsets(reader, "dialog", &[0xfb0])?;
    }
    color_tail(reader)?;
    super::mesh_quality::read(reader)
}

fn color_tail(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version > 0xd95c6 {
        offsets(reader, "dialog", &[0xfd0, 0xfd4, 0xfb4])?;
        reader.blob("dialog.raw64_fc0", 8)?;
        color(reader, "dialog.color_fd8")?;
    }
    if reader.version > 0xd95ce {
        offsets(reader, "dialog", &[0xfe8])?;
    }
    if reader.version > 0xd9687 {
        offsets(reader, "dialog", &[0xfec])?;
    }
    Ok(())
}
