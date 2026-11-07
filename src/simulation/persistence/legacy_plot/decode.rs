use super::super::load_reader::BinaryLoadReader;
use crate::Result;

// Member gaps belong to native object storage; the stream has no corresponding padding (C-839).
const FIXED_READS: [(u16, usize); 46] = [
    (0x8, 4),
    (0xc, 4),
    (0x10, 4),
    (0x14, 4),
    (0x18, 4),
    (0x1c, 4),
    (0x20, 4),
    (0x24, 4),
    (0x28, 4),
    (0x2c, 4),
    (0x30, 256),
    (0x130, 4),
    (0x134, 4),
    (0x138, 4),
    (0x13c, 256),
    (0x23c, 256),
    (0x33c, 256),
    (0x43c, 4),
    (0x440, 4),
    (0x444, 4),
    (0x448, 4),
    (0x44c, 4),
    (0x450, 4),
    (0x454, 128),
    (0x4d4, 4),
    (0x4d8, 8),
    (0x4e0, 4),
    (0x4e4, 4),
    (0x4e8, 4),
    (0x4ec, 4),
    (0x4f0, 4),
    (0x4f8, 8),
    (0x500, 8),
    (0x508, 8),
    (0x510, 8),
    (0x518, 4),
    (0x520, 8),
    (0x528, 8),
    (0x530, 4),
    (0x534, 4),
    (0x538, 4),
    (0x53c, 4),
    (0x540, 4),
    (0x544, 4),
    (0x548, 4),
    (0x54c, 4),
];

pub(super) fn read(reader: &mut BinaryLoadReader<'_>, maximum_characters: usize) -> Result<()> {
    if reader.version < 700002 {
        return reader.blob("legacy_plot.bulk_530", 1328);
    }
    if reader.version == 700002 {
        return reader.blob("legacy_plot.bulk_550", 1360);
    }
    for (member, width) in FIXED_READS {
        let path = format!("legacy_plot.member_{member:x}");
        match width {
            4 => reader.unsigned(&path)?,
            8 => reader.double(&path)?,
            _ => reader.blob(&path, width)?,
        }
    }
    suffix(reader, maximum_characters.min(512))
}

fn suffix(reader: &mut BinaryLoadReader<'_>, maximum_characters: usize) -> Result<()> {
    reader.character_text("legacy_plot.text_550", maximum_characters)?;
    if reader.version >= 700005 {
        reader.character_text("legacy_plot.text_558", maximum_characters)?;
    }
    if reader.version >= 700006 {
        reader.character_text("legacy_plot.text_560", maximum_characters)?;
    }
    if reader.version >= 700018 {
        reader.unsigned("legacy_plot.word_568")?;
        reader.unsigned("legacy_plot.word_56c")?;
    }
    Ok(())
}
