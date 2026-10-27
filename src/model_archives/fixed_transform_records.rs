use super::ModelDecoder;
use crate::archive_codec::MaterialRead;
use crate::{BessemerError, FailureClass, Result};

pub(super) fn read(reader: &mut ModelDecoder<'_>, label: &str, description: &str) -> Result<()> {
    // C-247 / C-276 bind this fixed archive operator, not variable mgXform::restore.
    for index in 0..30 {
        let word = reader.u32(&format!("{label}[{index}]"))?;
        if index == 28 {
            supported_kind(reader, word, label, description)?;
        }
    }
    Ok(())
}

fn supported_kind(
    reader: &mut ModelDecoder<'_>,
    word: u32,
    label: &str,
    description: &str,
) -> Result<()> {
    if word & 0xf0 == 0 {
        return Ok(());
    }
    reader.checked(Err(BessemerError::new(
        "model archive",
        &format!("{label}[28]"),
        FailureClass::Unsupported,
        format!("{description} fixed transform word {word:#010x}; expected word & 0xF0 == 0 for the documented fixed operator"),
    )))
}
