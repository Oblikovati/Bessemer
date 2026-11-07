use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // #54 / 11.227 / C-269: the sixth constructor-created array is not serialized.
    for offset in ["30", "50", "70", "90", "b0"] {
        model_lists::pointers(
            reader,
            map,
            &format!("array_{offset}"),
            ModelObjectRole::Any,
        )?;
    }
    for index in 0..6 {
        reader.i32(&format!("raw_flag_words[{index}]"))?;
    }
    // The native order differs from the runtime field ordering.
    reader.f64s(&["raw_18", "raw_28", "raw_20"])
}
