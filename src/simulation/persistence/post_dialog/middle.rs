use super::super::load_reader::BinaryLoadReader;
use super::helpers::{offsets, raw_words, raw64};
use crate::Result;

const FIELD_GATES: &[(i32, u32)] = &[
    (0xd6d82, 0xeb8),
    (0xd6d83, 0xebc),
    (0xd6d85, 0xec0),
    (0xd6d91, 0xec8),
    (0xd6d92, 0xed0),
    (0xd9494, 0xef4),
];

const WORD_GATES: &[(i32, &[u32])] = &[
    (0xcaa39, &[0xb10]),
    (0xcaa3b, &[0xc44]),
    (0xcaa40, &[0xcd0, 0xcd4, 0xcdc]),
    (0xcaa44, &[0xd04]),
    (0xcaa45, &[0xaec]),
    (0xcaa47, &[0xce0]),
    (0xcaa49, &[0x2c]),
    (0xcaa4d, &[0xd08, 0xd0c]),
    (0xcf852, &[0xcfc, 0xd00]),
];

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    prefix(reader)?;
    for (gate, words) in WORD_GATES {
        if reader.version > *gate {
            offsets(reader, "dialog", words)?;
        }
    }
    sensors_prefix(reader)?;
    super::collections::read(reader)?;
    final_fields(reader)
}

fn prefix(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version > 0xc5c1e {
        reader.character_text("dialog.text_b08", 1024)?;
    }
    if reader.version > 0xc5c23 {
        raw64(reader, "dialog.raw64_bc8_bd0_words_bd8_bdc.raw64", 2)?;
        raw_words(reader, "dialog.raw64_bc8_bd0_words_bd8_bdc.words", 2)?;
    }
    gated_references(reader)
}

fn gated_references(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version > 0xc8327 {
        offsets(reader, "dialog", &[0xbe4])?;
    }
    if reader.version > 0xc8334 {
        offsets(reader, "dialog", &[0xbec])?;
    }
    if reader.version > 0xc8339 {
        offsets(reader, "dialog", &[0xbf0])?;
        super::references::read(reader, "dialog.reference_bf8")?;
    }
    if reader.version > 0xcaa37 {
        offsets(reader, "moving_objects", &[0x48, 0x50])?;
    }
    Ok(())
}

fn sensors_prefix(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version > 0xcf859 {
        offsets(reader, "dialog", &[0xd38])?;
        reader.blob("dialog.raw64_d40", 8)?;
    }
    if reader.version > 0xcf860 {
        offsets(
            reader,
            "dialog",
            &[
                0xd48, 0xd4c, 0xd50, 0xd54, 0xd58, 0xd5c, 0xd60, 0xd64, 0xd68,
            ],
        )?;
        reader.character_text("dialog.text_d78", 1024)?;
    }
    if reader.version > 0xcf864 {
        offsets(reader, "dialog", &[0xd6c, 0xd70, 0xd74])?;
    }
    Ok(())
}

fn final_fields(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 0xd467f {
        offsets(reader, "dialog", &[0x234])?;
    }
    for &(gate, offset) in FIELD_GATES {
        if reader.version < gate {
            continue;
        }
        let label = format!("dialog.field_{offset:x}");
        if offset == 0xec8 {
            reader.blob(&label, 8)?;
        } else {
            reader.unsigned(&label)?;
        }
    }
    Ok(())
}
