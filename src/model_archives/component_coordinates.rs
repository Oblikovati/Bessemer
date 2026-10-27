use super::ModelDecoder;
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::component_references::reference_geometry(reader, map)?;
    if reader.version() > 0x3b27 {
        fixed_transform(reader)?;
    }
    Ok(())
}

fn fixed_transform(reader: &mut ModelDecoder<'_>) -> Result<()> {
    // C-247: this is the fixed archive operator, not the variable restore encoding.
    for index in 0..30 {
        let word = reader.u32(&format!("transform_100[{index}]"))?;
        if index == 28 {
            supported_transform_kind(reader, word)?;
        }
    }
    Ok(())
}

fn supported_transform_kind(reader: &mut ModelDecoder<'_>, word: u32) -> Result<()> {
    if word & 0xf0 == 0 {
        return Ok(());
    }
    reader.checked(Err(BessemerError::new(
        "model archive",
        "transform_100[28]",
        FailureClass::Unsupported,
        format!("component coordinate-system fixed transform word {word:#010x}; expected word & 0xF0 == 0 for the documented fixed operator"),
    )))
}
