use super::super::load_reader::BinaryLoadReader;
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.signed("member48")?;
    if reader.version >= 800068 {
        reader.signed("member4c")?;
    }
    if reader.version >= 850032 {
        reader.signed("member70")?;
    }
    if reader.version < 840013 {
        return Ok(());
    }
    reader.signed("member180")?;
    if reader.version >= 870004 {
        text_list(reader, "listb0")?;
    }
    if reader.version >= 870008 {
        text_list(reader, "liste8")?;
    }
    Ok(())
}

fn text_list(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    let count = reader.count(&format!("{label}.count"), 100000, 4)?;
    for index in 0..count {
        reader.text(&format!("{label}.{index}"))?;
    }
    Ok(())
}
