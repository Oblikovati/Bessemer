use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::MaterialRead;

/// Preserve material settings and direct ranges; e.g. `read(reader)?`.
pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_0c"])?;
    reader.text("table_name_10")?;
    let count = reader.i32("ranges.count")?;
    // #214 / C-3559: native LOAD zero-extends the signed wire count's bits.
    let count = reader.counted_entries("material thickness ranges", i64::from(count as u32), 37)?;
    for index in 0..count {
        reader.scope(&format!("ranges[{index}]"), canonical_range)?;
    }
    Ok(())
}

fn canonical_range(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("raw_08")?;
    reader.f64("raw_10")?;
    reader.i32("raw_18")?;
    reader.f64("raw_20")?;
    reader.i32("raw_28")?;
    reader.u8("raw_2c")?;
    reader.f64("raw_30")?;
    Ok(())
}
