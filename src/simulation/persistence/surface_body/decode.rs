use super::super::load_reader::BinaryLoadReader;
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.signed("word70")?;
    reader.text("text68")?;
    early_fields(reader)?;
    late_fields(reader)
}

fn early_fields(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 830015 {
        reader.signed("word80")?;
    }
    if reader.version >= 830023 {
        reader.signed("word88")?;
    }
    if reader.version >= 830025 {
        reader.words("words84_8c", 2)?;
    }
    if reader.version >= 830029 {
        reader.words("words98_a4", 4)?;
        reader.doubles("doublesa8_c8", 5)?;
    }
    if reader.version >= 830031 {
        reader.signed("word90")?;
    }
    Ok(())
}

fn late_fields(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 850013 {
        reader.words("words100_94", 2)?;
    }
    if reader.version >= 870008 {
        let count = reader.count("list108.count", 100000, 4)?;
        for index in 0..count {
            reader.text(&format!("list108.{index}"))?;
        }
    }
    if reader.version >= 870022 {
        reader.signed("word144")?;
    }
    Ok(())
}
