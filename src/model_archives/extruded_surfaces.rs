use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.required_object(
        "end_spec_800",
        map,
        ModelObjectRole::Declared(&["moEndSpec_c"]),
    )?;
    reader.i32("raw_810")?;
    declared(reader, map, "ordered_faces_808", &["moOrderedFaceArray_c"])?;
    reader.i32s(&["raw_814", "raw_818"])?;
    model_lists::pointers(reader, map, "array_820", ModelObjectRole::Any)
}

pub(super) fn ordered_faces(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "array_08", ModelObjectRole::Any)?;
    groups(reader, map, "raw_words_48", "arrays_28")?;
    model_lists::pointers(reader, map, "array_68", ModelObjectRole::Any)?;
    groups(reader, map, "raw_words_a8", "arrays_88")
}

fn groups(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    words: &str,
    arrays: &str,
) -> Result<()> {
    let count = reader.signed_count(&format!("{words}.count"), 6)?;
    for index in 0..count {
        reader.u32(&format!("{words}[{index}]"))?;
    }
    for index in 0..count {
        reader.scope(&format!("{arrays}[{index}]"), |reader| {
            model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
        })?;
    }
    Ok(())
}

pub(super) fn keeper(reader: &mut ModelDecoder<'_>) -> Result<()> {
    // #54 / C-181: su_CObject::Serialize returns without a body when archive mode 0x10 is clear.
    reader.require_ordinary_part()
}
