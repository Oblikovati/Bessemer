use super::super::{ModelDecoder, configuration_io};
use crate::Result;
use crate::archive_codec::MaterialRead;

/// Preserve direct su_CTime wire words without runtime widening; e.g. `temporary_records(reader)?`.
pub(super) fn temporary_records(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = configuration_io::positive_count(reader, "temporary_count", 8)?;
    for index in 0..count {
        reader.scope(&format!("temporary_records[{index}]"), |reader| {
            reader.u32("time_0")?;
            reader.u32("time_8")?;
            Ok(())
        })?;
    }
    Ok(())
}

/// Preserve ordered duplicate pairs and raw vector zeros; e.g. `downstream_body(reader)?`.
pub(super) fn downstream_body(reader: &mut ModelDecoder<'_>) -> Result<()> {
    downstream_pairs(reader)?;
    let count = configuration_io::positive_count(reader, "value_count", 4)?;
    for index in 0..count {
        reader.i32(&format!("values[{index}]"))?;
    }
    reader.i32("unknown_38")?;
    if reader.version() >= 0x3359 {
        reader.i32s(&["values_40[0]", "values_40[1]"])?;
    }
    Ok(())
}

fn downstream_pairs(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = configuration_io::positive_count(reader, "pair_count", 8)?;
    for index in 0..count {
        reader.scope(&format!("pairs[{index}]"), |reader| {
            reader.i32("key")?;
            reader.i32("value")?;
            Ok(())
        })?;
    }
    Ok(())
}
