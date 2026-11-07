use super::super::load_reader::BinaryLoadReader;
use super::helpers::{count, counted_bytes, offsets, raw_words};
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version > 0xcf865 {
        sensors(reader)?;
    }
    if reader.version >= 0xcf86d {
        offsets(reader, "dialog", &[0x230])?;
    }
    if reader.version >= 0xd1f67 {
        let count = count(reader, "dialog.da4")?;
        reader.words("dialog.da8_integer_array", count)?;
    }
    if reader.version >= 0xd4677 {
        offsets(reader, "dialog", &[0xbc0])?;
        super::references::read(reader, "dialog.reference_b78")?;
    }
    string_list(reader)
}

fn sensors(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    let sensors = count(reader, "sensors")?;
    for index in 0..sensors {
        let label = format!("sensor[{index}]");
        reader.character_text(&format!("{label}.name"), 1024)?;
        let points = count(reader, &format!("{label}.points"))?;
        for point in 0..points {
            raw_words(reader, &format!("{label}.point[{point}].eleven_words"), 11)?;
        }
    }
    Ok(())
}

fn string_list(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version < 0xd4679 {
        return Ok(());
    }
    let count = count(reader, "dialog.string_list_dd0")?;
    for index in 0..count {
        let label = format!("dialog.string_list[{index}]");
        if reader.version < 0xd96f3 {
            counted_bytes(reader, &label, 0x100000)?;
        } else {
            reader.character_text(&label, 1024)?;
        }
    }
    Ok(())
}
