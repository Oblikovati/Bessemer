use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("array_848", |reader| {
        body_features::nullable_array(reader, map)
    })?;
    reader.i32("raw_840")?;
    model_lists::pointers(reader, map, "array_800", ModelObjectRole::Any)?;
    model_lists::unsigned_words(reader, "dwords_820")?;
    reader.i32s(&["raw_850", "raw_854", "raw_858", "raw_85c"])?;
    // #54 / C-272: constructor state at +860 consumes no modern archive bytes.
    model_lists::pointers(reader, map, "array_868", ModelObjectRole::Any)
}

pub(super) fn data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for offset in ["08", "10", "18"] {
        reader.scope(&format!("array_{offset}"), |reader| {
            body_features::nullable_array(reader, map)
        })?;
    }
    reader.i32("raw_20")?;
    Ok(())
}
