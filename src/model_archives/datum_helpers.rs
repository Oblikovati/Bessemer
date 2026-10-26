use super::ModelDecoder;
use super::raw_fields::{
    RawScalarKind::{Double, Text, Unsigned},
    fields, groups,
};
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let kind = reader.i32("kind")?;
    if reader.version() > 0x373c {
        fields(reader, Unsigned, "40")?;
    }
    datum_payload(reader, kind)?;
    if reader.version() > 0x3b19 {
        groups(reader, DATUM_TAIL)?;
    }
    Ok(())
}

const DATUM_TAIL: &[(super::raw_fields::RawScalarKind, &str)] = &[
    (Double, "48"),
    (Text, "50"),
    (Double, "58"),
    (Text, "60"),
    (Double, "68"),
    (Text, "70"),
];

fn datum_payload(reader: &mut ModelDecoder<'_>, kind: i32) -> Result<()> {
    if kind == 0 {
        for index in 0..reader.signed_count("datum_item_count", 8)? {
            reader.scope(&format!("data[{index}]"), item)?;
        }
    } else {
        reader.scope("data", atom)?;
    }
    Ok(())
}

fn item(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.i32("kind")? == 0 {
        return reader.scope("data", vector);
    }
    reader.scope("data", atom)
}

fn vector(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x373c {
        fields(reader, Unsigned, "18")?;
    }
    for index in 0..reader.signed_count("datum_atom_count", 13)? {
        reader.scope(&format!("items[{index}]"), atom)?;
    }
    Ok(())
}

fn atom(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(reader, &[(Text, "00"), (Unsigned, "08"), (Double, "10")])?;
    if reader.version() >= 0x334b {
        fields(reader, Text, "18")?;
    }
    if reader.version() > 0x3b0e && reader.version() < 0x3b1a {
        for index in 0..3 {
            fields(reader, Double, &format!("discarded_value_{index}"))?;
            fields(reader, Text, &format!("discarded_text_{index}"))?;
        }
    }
    Ok(())
}
