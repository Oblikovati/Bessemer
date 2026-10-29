use super::{ModelDecoder, ModelObjectRole, body_features, configuration_io, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve sculpt's directly owned trim data without adding object tags.
/// Example: `sculpt_features::read(reader, map)?` for `moSculpt_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["trim_array_20", "entity_keeper_78"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    for label in ["chooser_820", "chooser_8a8"] {
        reader.scope(label, |reader| body_features::direct_chooser(reader, map))?;
    }
    groups(reader)?;
    reader.i32s(&["raw_c0", "raw_c4", "raw_d0", "raw_c8", "raw_cc"])
}

fn groups(reader: &mut ModelDecoder<'_>) -> Result<()> {
    // #54 / C-398 proves that native nonpositive counts preserve an empty loop.
    let count = configuration_io::positive_count(reader, "dword_array_count", 2)?;
    for index in 0..count {
        model_lists::unsigned_words(reader, &format!("dword_arrays_960[{index}].items"))?;
    }
    Ok(())
}
