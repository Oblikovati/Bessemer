use super::super::load_reader::BinaryLoadReader;
use super::helpers::{offsets, raw64};
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version > 0xd968a {
        offsets(
            reader,
            "mesh_quality",
            &[
                0xffc, 0x1000, 0x1004, 0x1008, 0x1018, 0x1118, 0x111c, 0x1120, 0x1124, 0x1128,
            ],
        )?;
        for index in 0..12 {
            raw64(reader, &format!("mesh_quality.raw64_pair[{index}]"), 2)?;
        }
        trailing_words(reader)?;
    }
    if reader.version >= 0xd9691 {
        offsets(reader, "dialog", &[0x1168])?;
    }
    Ok(())
}

fn trailing_words(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    for index in 0..12 {
        reader.unsigned(&format!("mesh_quality.word_array[{index}]"))?;
    }
    if reader.version > 0xd968b {
        reader.blob("mesh_quality.raw64_1110", 8)?;
    }
    if reader.version > 0xd968d {
        offsets(reader, "mesh_quality", &[0x112c])?;
    }
    if reader.version > 0xd968e {
        offsets(reader, "mesh_quality", &[0x1160])?;
    }
    Ok(())
}
