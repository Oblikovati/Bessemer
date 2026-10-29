use super::{ModelDecoder, ModelObjectRole, model_lists, raw_fields};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
use raw_fields::RawScalarKind::{Double, Single, Text, Word};

const BORDER_TAIL: &[(raw_fields::RawScalarKind, &str)] = &[
    (Single, "58 5c"),
    (Double, "68"),
    (Single, "70_x 70_y 70_z"),
];
const SYMBOL_BODY: &[(raw_fields::RawScalarKind, &str)] = &[
    (Text, "40 48"),
    (Double, "50 60 70"),
    (Single, "78_x 78_y 78_z"),
];

/// Retain the complete inherited text base before the recursive border suffix.
/// Example: `annotation_extras::border(reader, map)?`.
pub(super) fn border(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("base", super::note_text::text_data_base)?;
    raw_fields::groups(
        reader,
        &[(Single, "40 44 48 4c 50 54"), (Word, "60 62"), (Text, "80")],
    )?;
    reader.object(
        "data_list_88",
        map,
        ModelObjectRole::Declared(&["moGenTextDataList_c"]),
    )?;
    raw_fields::groups(reader, BORDER_TAIL)
}

/// Preserve both versioned symbol suffixes and exact floating-point bits.
/// Example: `annotation_extras::symbol(reader)?`.
pub(super) fn symbol(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.scope("base", super::note_text::text_data_base)?;
    raw_fields::groups(reader, SYMBOL_BODY)?;
    if reader.version() > 0x2bca {
        reader.i32("raw_84")?;
    }
    if reader.version() >= 0x2be2 {
        raw_fields::fields(reader, Double, "58 68")?;
    }
    Ok(())
}

/// Keep the direct array's ReadCount without inserting a pointer or presence tag.
/// Example: `annotation_extras::thread_instances(reader, map)?`.
pub(super) fn thread_instances(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("array_08", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })
}

/// Retain all three raw thread-item words without assigning field semantics.
/// Example: `annotation_extras::thread_item(reader)?`.
pub(super) fn thread_item(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_0c", "raw_10"])
}

/// Use the complete valued base before the stored pattern-position word.
/// Example: `annotation_extras::pattern_count(reader, map)?`.
pub(super) fn pattern_count(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::sketch_dimensions::valued(reader, map)?;
    reader.i32("pattern_position_130").map(|_| ())
}
