use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, base_flanges, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Compose the inherited bend group and all direct mitered-flange collections.
/// Example: `mitered_flanges::read(reader, map)?` for `moSMMiteredFlange_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("bend_group", |reader| base_flanges::bend_group(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "profile_918", &["moProfileRef_w"])?;
    scalar_prefix(reader)?;
    for offset in ["c38", "c40"] {
        declared(
            reader,
            map,
            &format!("dimension_{offset}"),
            super::display_dimension::CLASSES,
        )?;
    }
    reader.i32("raw_974")?;
    direct_collections(reader, map)?;
    for offset in ["9f0", "a10", "a30", "a50"] {
        reader.scope(&format!("handles_{offset}"), |reader| {
            base_flanges::handles(reader, map)
        })?;
    }
    model_lists::pointers(reader, map, "array_c58", ModelObjectRole::Any)?;
    reader.i32("raw_978")?;
    Ok(())
}

fn scalar_prefix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.f64s(&["raw_948", "raw_c20", "raw_c28", "raw_c30"])?;
    reader.i32s(&[
        "raw_968", "raw_96c", "raw_970", "raw_97c", "raw_980", "raw_b9c", "raw_984", "raw_b98",
    ])?;
    reader.f64s(&["raw_c08", "raw_c10"])?;
    reader.i32s(&["raw_c18", "raw_c1c"])
}

fn direct_collections(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_lists::pointers(reader, map, "array_988", ModelObjectRole::Any)?;
    model_lists::unsigned_words(reader, "dwords_ba8")?;
    model_lists::unsigned_words(reader, "dwords_bc8")?;
    model_lists::pointers(reader, map, "array_be8", ModelObjectRole::Any)
}
