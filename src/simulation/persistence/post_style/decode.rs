use super::super::load_reader::BinaryLoadReader;
use crate::Result;

pub(in crate::simulation::persistence) fn read(
    reader: &mut BinaryLoadReader<'_>,
    maximum_characters: usize,
) -> Result<()> {
    if reader.version < 700006 {
        return super::legacy::read(reader);
    }
    words(reader, "style", &[0x238, 0x234, 0x21c, 0x240, 0x23c])?;
    colors(reader, "style", &[0x1f4, 0x1bc, 0x220, 0x1dc])?;
    words(reader, "style", &[0x1b8, 0x1f0, 0x230])?;
    legend(reader)?;
    reader.character_text("style.text", maximum_characters)?;
    suffix(reader)
}

pub(super) fn words(
    reader: &mut BinaryLoadReader<'_>,
    prefix: &str,
    offsets: &[u32],
) -> Result<()> {
    for offset in offsets {
        reader.signed(&format!("{prefix}.word_{offset:x}"))?;
    }
    Ok(())
}

pub(super) fn colors(
    reader: &mut BinaryLoadReader<'_>,
    prefix: &str,
    offsets: &[u32],
) -> Result<()> {
    for offset in offsets {
        for component in 0..4 {
            reader.unsigned(&format!("{prefix}.color_{offset:x}.component_{component}"))?;
        }
    }
    Ok(())
}

fn legend(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    words(reader, "legend", &[0xa8, 0xc])?;
    colors(
        reader,
        "legend",
        &[0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78, 0x88, 0x98],
    )?;
    reader.unsigned("legend.word_ac")?;
    reader.unsigned("legend.word_b0")?;
    words(reader, "legend", &[0x10, 0x14, 0xb8, 0xbc, 0xb4, 8])?;
    if reader.version > 810008 {
        words(reader, "legend", &[0xd4])?;
    }
    if reader.version > 810018 {
        colors(reader, "legend", &[0xd8])?;
    }
    legend_suffix(reader)
}

fn legend_suffix(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 840008 {
        words(reader, "legend", &[0xc0])?;
    }
    if reader.version <= 840009 {
        return Ok(());
    }
    words(reader, "legend", &[0xc4, 0xc8])?;
    if reader.version >= 890103 {
        words(reader, "legend", &[0xe8, 0xfc])?;
        colors(reader, "legend", &[0xec, 0x100])?;
    }
    Ok(())
}

fn suffix(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version > 700329 {
        words(reader, "style", &[0x28c, 0x290, 0x294, 0x298])?;
        colors(reader, "style", &[0x29c])?;
    }
    if reader.version > 820005 {
        colors(reader, "style", &[0x2ac, 0x2bc])?;
    }
    if reader.version >= 840008 {
        words(reader, "style", &[0x1ec, 0x204, 0x208])?;
        colors(reader, "style", &[0x20c])?;
    }
    if reader.version > 890701 {
        colors(reader, "style", &[0x1cc])?;
    }
    Ok(())
}
