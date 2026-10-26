use super::{ModelDecoder, body_features};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn native_table(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| {
        body_features::model_feature(reader, map)
    })?;
    reader.scope("payload", payload)
}

pub(super) fn payload(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    if reader.archive_version().value() <= 0x374d {
        return Ok(());
    }
    reader.text("raw_368")?;
    reader.i32s(&["raw_370", "raw_374"])?;
    string_pairs(reader)?;
    reader.u32("raw_388")?;
    Ok(())
}

fn string_pairs(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = reader.signed_count("pairs.count", 2)?;
    let capacity = reader.remaining().saturating_sub(4) / 2;
    if count > capacity {
        return reader.checked(Err(super::invalid(format!(
            "native table pair count {count}; expected 0..={capacity} pairs before a four-byte time"
        ))));
    }
    for index in 0..count {
        reader.text(&format!("pairs[{index}].key"))?;
        reader.text(&format!("pairs[{index}].value"))?;
    }
    Ok(())
}
