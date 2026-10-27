use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("data_10", contents)
}

pub(super) fn contents(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["50", "68"] {
        for index in 0..3 {
            reader.f64(&format!("vector_{label}[{index}]"))?;
        }
    }
    for label in ["28", "30", "38", "40", "80", "88"] {
        reader.f64(&format!("raw_{label}"))?;
    }
    reader.i32("raw_90")?;
    reader.f64("raw_48")?;
    Ok(())
}
