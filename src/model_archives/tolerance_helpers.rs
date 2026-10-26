use super::ModelDecoder;
use super::raw_fields::{
    RawScalarKind::{Double, Signed, Text},
    fields, groups,
};
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    groups(reader, &[(Text, "08"), (Signed, "10 14 18")])?;
    if reader.i32("presence_1c")? != 0 {
        reader.scope("tolerance_20", values_and_modifiers)?;
    }
    Ok(())
}

fn values_and_modifiers(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("value_count", 12)? {
        reader.scope(&format!("values[{index}]"), value)?;
    }
    fields(
        reader,
        Signed,
        "18_first 18_second 1c 20 24 28 2c 30 34 38 3c",
    )?;
    modifiers(reader)?;
    fields(reader, Signed, "58 5c 60 64 68")?;
    if reader.version() < 0x3724 {
        fields(reader, Signed, "discarded_word")?;
    }
    fields(reader, Signed, "6c 70")?;
    if reader.version() > 0x3b10 {
        fields(reader, Signed, "74 78 7c")?;
    }
    Ok(())
}

fn value(reader: &mut ModelDecoder<'_>) -> Result<()> {
    fields(reader, Signed, "key")?;
    if reader.version() >= 0x373f {
        fields(reader, Signed, "subkey")?;
    }
    fields(reader, Double, "value")?;
    if reader.version() >= 0x334b {
        fields(reader, Text, "text")?;
    }
    Ok(())
}

fn modifiers(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = if reader.version() >= 0x373f {
        reader.signed_count("modifier_count", 16)?
    } else {
        reader.entries(1, "implicit tolerance modifier")?;
        1
    };
    for index in 0..count {
        reader.scope(&format!("modifiers[{index}]"), |reader| {
            fields(reader, Signed, "00 04 08 0c")
        })?;
    }
    Ok(())
}
