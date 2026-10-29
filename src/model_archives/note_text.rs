use super::{ModelDecoder, ModelObjectRole, raw_fields, sketch_text};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
use raw_fields::RawScalarKind::{Double, Signed, Single, Text, Unsigned, Word};

const DISPLAY_ITEMS: &[&str] = &[
    "moDisplayItem_c",
    "moDisplayNote_c",
    "moDisplayWeld_c",
    "moDisplayDim_c",
    "moDisplayDistanceDim_c",
    "moDisplayRadialDim_c",
    "moDisplayAngularDim_c",
    "moDisplayRevolveDim_c",
];
const PARAGRAPH_FIELDS: &[(raw_fields::RawScalarKind, &str)] = &[
    (Word, "00"),
    (Double, "08 10 18"),
    (Word, "20 24"),
    (Text, "28"),
    (Signed, "30"),
    (Unsigned, "34"),
    (Signed, "38"),
    (Double, "40 48"),
    (Signed, "50"),
    (Double, "58 60"),
    (Signed, "68 6c"),
];

/// Preserve generic text and its direct paragraph without runtime flag repair.
/// Example: `note_text::read(reader, map)?`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("dimension_text", |reader| sketch_text::read(reader, map))?;
    data_pointer(reader, map, "data_list_158")?;
    reader.i32("raw_160")?;
    reader.object(
        "display_item_170",
        map,
        ModelObjectRole::Declared(DISPLAY_ITEMS),
    )?;
    reader.scope("paragraph_188", paragraph)
}

fn paragraph(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    raw_fields::groups(reader, PARAGRAPH_FIELDS)
}

/// Retain a tagged text-array pointer before the direct scalar suffix.
/// Example: `note_text::data_list(reader, map)?`.
pub(super) fn data_list(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("array_10", map, ModelObjectRole::Declared(&["suObArray"]))?;
    reader.i32("raw_128")?;
    raw_fields::fields(reader, Double, "18 20 28 30")
}

/// Preserve each line head's independent f32 triples and direct paragraph.
/// Example: `note_text::line_head(reader, map)?`.
pub(super) fn line_head(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_0c"])?;
    raw_fields::fields(reader, Single, "14_x 14_y 14_z")?;
    reader.u16("raw_10")?;
    reader.scope("paragraph_30", paragraph)?;
    raw_fields::fields(reader, Single, "38_x 38_y 38_z")?;
    data_pointer(reader, map, "data_list_48")?;
    raw_fields::groups(reader, &[(Signed, "50"), (Double, "58"), (Signed, "64")])
}

/// Retain the stored character as a u16 unit and preserve both short flags.
/// Example: `note_text::character(reader)?`.
pub(super) fn character(reader: &mut ModelDecoder<'_>) -> Result<()> {
    text_data_base(reader)?;
    reader.u16("raw_40").map(|_| ())
}

/// Keep the shared 36-byte prefix separate from character-only storage.
/// Example: `text_data_base(reader)?` before a border or character suffix.
pub(super) fn text_data_base(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    raw_fields::groups(
        reader,
        &[
            (Signed, "08 0c"),
            (Single, "20_x 20_y 20_z 2c"),
            (Word, "30"),
            (Signed, "34"),
            (Single, "38"),
            (Word, "10"),
        ],
    )
}

fn data_pointer(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    reader.object(
        label,
        map,
        ModelObjectRole::Declared(&["moGenTextDataList_c"]),
    )
}
