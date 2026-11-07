use super::super::load_reader::BinaryLoadReader;
use super::helpers::offsets;
use crate::Result;

const PLANE_GATES: &[(i32, u32)] = &[
    (800006, 0xad4),
    (800007, 0xad8),
    (800024, 0xae4),
    (800025, 0xae8),
    (800045, 0xaf0),
    (800075, 0xb00),
];

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    offsets(reader, "dialog", &[0xad0])?;
    for index in 0..6 {
        offsets(reader, &format!("iso[{index}]"), &[0x10, 8, 0xc, 0x14])?;
    }
    for index in 0..6 {
        section(reader, index)?;
    }
    for &(gate, offset) in PLANE_GATES {
        if reader.version > gate {
            offsets(reader, "dialog", &[offset])?;
        }
    }
    if reader.version > 810008 {
        super::references::read(reader, "dialog.reference_b30")?;
    }
    Ok(())
}

fn section(reader: &mut BinaryLoadReader<'_>, index: usize) -> Result<()> {
    let label = format!("section[{index}]");
    offsets(
        reader,
        &label,
        &[
            8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x34, 0x30, 0x38, 0x44, 0x3c,
            0x40, 0x48,
        ],
    )?;
    reader.character_text(&format!("{label}.name"), 1024)?;
    if reader.version > 810012 {
        super::references::read(reader, &format!("{label}.reference"))?;
    }
    Ok(())
}
