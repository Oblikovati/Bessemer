use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Retain the complete punch body and use its stored internal revision independently of file version.
/// Example: `punches::read(reader, map)?` for `moPunch_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let revision = reader.i32("revision_858")?;
    leading(reader, map, revision)?;
    values(reader, revision)?;
    late_fields(reader, map, revision)?;
    super::punch_extensions::read(reader, map)?;
    if revision < 2 {
        reader.scope("legacy_keeper_908", |reader| {
            super::draft_keepers::read(reader, map)
        })?;
    }
    Ok(())
}

fn leading(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, revision: i32) -> Result<()> {
    for label in ["dimension_800", "dimension_808"] {
        declared(reader, map, label, super::display_dimension::CLASSES)?;
    }
    if revision >= 1 {
        reader.i32("raw_864")?;
    }
    nullable_array(reader, map, "array_810")?;
    if revision < 3 {
        nullable_array(reader, map, "array_818")?;
    }
    reader.i32s(&["raw_860", "raw_840"])?;
    nullable_array(reader, map, "array_820")
}

fn nullable_array(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    reader.scope(label, |reader| body_features::nullable_array(reader, map))
}

fn values(reader: &mut ModelDecoder<'_>, revision: i32) -> Result<()> {
    if revision < 2 {
        double_vector(reader, "legacy_point_848")?;
        reader.i32("legacy_integer_830")?;
        return Ok(());
    }
    reader.scope("points_848", |reader| {
        super::punch_arrays::read(reader, true)
    })?;
    reader.scope("integers_830", |reader| {
        super::punch_arrays::read(reader, false)
    })
}

fn late_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, revision: i32) -> Result<()> {
    for (minimum, label) in [
        (4, "raw_910"),
        (5, "raw_914"),
        (6, "raw_918"),
        (6, "raw_85c"),
    ] {
        if revision >= minimum {
            reader.i32(label)?;
        }
    }
    if revision >= 7 {
        reader.object(
            "tagged_array_920",
            map,
            ModelObjectRole::Declared(&["suObArray"]),
        )?;
    }
    Ok(())
}
