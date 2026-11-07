use super::{ModelDecoder, features, model_lists};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::read(reader, map)?;
    reader.i32("unknown_354")?;
    reader.text("unknown_380")?;
    directions(reader)?;
    model_lists::pointers(reader, map, "references", super::ModelObjectRole::Any)?;
    reader.i32s(&["unknown_3a8", "unknown_3ac"])?;
    if reader.version() >= 0x3eeb {
        for offset in ["3b0", "3b1", "3b2", "3b3"] {
            reader.u8(&format!("unknown_{offset}"))?;
        }
    }
    Ok(())
}

fn directions(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = reader.u16("direction_count")? as usize;
    if count > 3 {
        return reader.checked(Err(BessemerError::new(
            "model archive",
            "direction_count",
            FailureClass::Unsupported,
            format!("position direction count {count}; expected 0..=3"),
        )));
    }
    reader.entries(count * 2, "position direction words")?;
    for label in ["unknown_388", "unknown_3a0"] {
        for index in 0..count {
            reader.u16(&format!("{label}[{index}]"))?;
        }
    }
    Ok(())
}
