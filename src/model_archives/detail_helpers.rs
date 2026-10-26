use super::raw_fields::{
    RawScalarKind::{self, Double, Octet, Signed, Text},
    fields, groups,
};
use super::{ModelDecoder, ModelObjectRole, detail_prefix, invalid};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("helpers[0]", surface_finish)?;
    reader.scope("helpers[1]", |reader| note(reader, map))
}

pub(super) fn annotation(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(reader, &[(Signed, "68 6c 70 74"), (Octet, "78")])?;
    reader.scope("character_format_08", detail_prefix::character_format)?;
    groups(reader, ANNOTATION_TAIL)
}

const ANNOTATION_TAIL: &[(RawScalarKind, &str)] = &[
    (Octet, "7c"),
    (Signed, "80"),
    (Double, "88"),
    (Signed, "90"),
    (Octet, "94"),
    (Signed, "98"),
    (Double, "a0"),
    (Signed, "a8"),
];

pub(super) fn surface_finish(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    groups(reader, SURFACE_PREFIX)?;
    reader.scope("character_format_c0", detail_prefix::character_format)?;
    fields(reader, Octet, "120 124")?;
    reader.scope("annotation_128", annotation)?;
    if reader.version() > 0x42e3 {
        groups(reader, &[(Signed, "7c"), (Double, "80")])?;
    }
    if reader.version() > 0x46bf {
        groups(reader, &[(Text, "88 90"), (Signed, "98 9c")])?;
    }
    Ok(())
}

const SURFACE_PREFIX: &[(RawScalarKind, &str)] = &[
    (Signed, "08"),
    (Text, "10 20 18 28 30 38 40 48 50 58"),
    (Octet, "68 6c 70 74"),
    (Signed, "78"),
    (Octet, "a0 a4 a8 ac"),
    (Double, "b0"),
    (Octet, "b8"),
];

pub(super) fn note(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    groups(reader, NOTE_PREFIX)?;
    reader.scope("character_format_c8", detail_prefix::character_format)?;
    fields(reader, Signed, "240 244")?;
    if reader.version() > 0x2f4a {
        note_map(reader, map)?;
    }
    fields(reader, Octet, "138 13c 140")?;
    reader.scope("annotation_150", annotation)?;
    groups(reader, NOTE_TAIL)?;
    if reader.version() > 0x3373 {
        fields(reader, Octet, "148")?;
    }
    Ok(())
}

const NOTE_PREFIX: &[(RawScalarKind, &str)] = &[
    (Text, "08 18 20 28"),
    (Octet, "10 30 34 38 3c 40"),
    (Signed, "44"),
    (Double, "50"),
    (Octet, "58"),
    (Signed, "5c 60 78 7c 80 84 88 8c"),
    (Double, "90 98"),
    (Octet, "a0 a4 a8 ac b0 b4 b8 bc c0 12c 130 134"),
];
const NOTE_TAIL: &[(RawScalarKind, &str)] = &[
    (Octet, "48 128"),
    (Signed, "14c"),
    (Double, "68"),
    (Signed, "218 21c"),
    (Text, "220"),
    (Signed, "230 234 238 23c"),
    (Double, "70 228"),
    (Signed, "4c"),
    (Octet, "144"),
];

fn note_map(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let count = note_count(reader)?;
    for index in 0..count {
        reader.scope(&format!("helper_map[{index}]"), |reader| {
            note_entry(reader, map)
        })?;
    }
    Ok(())
}

fn note_count(reader: &mut ModelDecoder<'_>) -> Result<usize> {
    let count = reader.u64("helper_map_count")?;
    let available = (reader.remaining() / 8) as u64;
    let maximum = available.min(i32::MAX as u64);
    let shape = if count <= maximum {
        Ok(count as usize)
    } else {
        Err(invalid(format!(
            "note-helper map count {count}; expected 0..={maximum} entries of at least 8 bytes"
        )))
    };
    let count = reader.checked(shape)?;
    reader.entries(count, "note-helper map entries")?;
    Ok(count)
}

fn note_entry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("key")?;
    let kind = reader.i32("kind")?;
    let class = match kind {
        1 => "moGTolDataHelper_c",
        2 => "moSFDataHelper_c",
        4 => "moNoteDataHelper_c",
        _ => return Ok(()),
    };
    reader.object("helper", map, ModelObjectRole::Declared(&[class]))
}
