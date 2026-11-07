use super::{
    ModelDecoder,
    raw_fields::{RawScalarKind, fields, groups},
};
use crate::Result;
use crate::archive_codec::MaterialRead;
use RawScalarKind::{Double, Octet, Signed, Text, Unsigned, Word};

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    groups(reader, PREFIX_EARLY)?;
    middle(reader)?;
    late(reader)?;
    for index in 0..reader.signed_count("exclusion_string_count", 1)? {
        reader.text(&format!("exclusion_strings[{index}]"))?;
    }
    fields(reader, Signed, "198 1a0 1a4")?;
    grids(reader)?;
    if reader.version() > 0x3b0f {
        reader.scope("raw_string_array_b8", strings)?;
    }
    reader.scope("character_format", character_format)
}

const PREFIX_EARLY: &[(RawScalarKind, &str)] = &[
    (Signed, "10"),
    (Text, "13a0"),
    (Double, "28"),
    (Octet, "30"),
    (Unsigned, "20"),
    (Signed, "24"),
    (Double, "08"),
    (Octet, "38 14 18 1c 34"),
    (Signed, "1388"),
    (Unsigned, "ec f0 f4"),
    (Double, "f8 100 108"),
    (Unsigned, "110 114 118"),
    (Double, "120 128 130"),
    (Signed, "138"),
    (Double, "140 148"),
    (Signed, "e8 150 154 158 164 168 16c 170 174"),
];

fn middle(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(
        reader,
        &[
            (Octet, "3c"),
            (Signed, "138c 1390 1394 1398"),
            (Octet, "40 44"),
            (Word, "e0 e2"),
            (Signed, "4c 48"),
            (Double, "2bc0"),
            (Signed, "2bc8"),
            (Text, "2bd8"),
            (Signed, "2be0"),
            (Text, "2be8"),
            (Signed, "2bf0 178 17c 1a8 1ac 1b0"),
        ],
    )
}

fn late(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x2bb3 {
        fields(reader, Signed, "1b4")?;
    }
    fields(reader, Signed, "180 184 188 18c 190 194")
}

fn grids(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let modern = reader.version() > 0x374e;
    let columns = if modern { 8 } else { 5 };
    for base in [0x228, 0x288] {
        let rows = if modern || base == 0x288 { 3 } else { 2 };
        for row in 0..rows {
            grid_row(reader, base + row * 0x20, columns)?;
        }
    }
    Ok(())
}

fn grid_row(reader: &mut ModelDecoder<'_>, first: u32, columns: u32) -> Result<()> {
    for column in 0..columns {
        fields(reader, Signed, &format!("{:x}", first + column * 4))?;
    }
    Ok(())
}

pub(super) fn strings(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.count_with_minimum("count", 1)? {
        reader.text(&format!("items[{index}]"))?;
    }
    Ok(())
}

pub(super) fn character_format(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.text("font_name")?;
    groups(
        reader,
        &[
            (Double, "10"),
            (Signed, "18"),
            (Double, "20"),
            (Unsigned, "30"),
            (Double, "28"),
        ],
    )?;
    reader.f64("width_factor")?;
    fields(reader, Double, "40 48 50")?;
    if reader.version() >= 0x3eed {
        fields(reader, Unsigned, "58")?;
    }
    Ok(())
}
