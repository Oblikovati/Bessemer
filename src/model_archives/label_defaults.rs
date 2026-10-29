use super::raw_fields::{
    RawScalarKind::{Double, Signed, Text, Unsigned, Word},
    fields, groups,
};
use super::{ModelDecoder, detail_prefix};
use crate::Result;
use crate::archive_codec::{MaterialRead, line_style};

#[derive(Clone, Copy)]
enum LabelDefaultKind {
    Detail,
    Section,
    Auxiliary,
    Orthographic,
    Miscellaneous,
}

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    fields(reader, Signed, "08 0c")?;
    for (index, kind) in [
        LabelDefaultKind::Detail,
        LabelDefaultKind::Section,
        LabelDefaultKind::Auxiliary,
    ]
    .iter()
    .enumerate()
    {
        definition(reader, index, *kind)?;
    }
    reader.scope("character_format_458", detail_prefix::character_format)?;
    definition(reader, 3, LabelDefaultKind::Orthographic)?;
    definition(reader, 4, LabelDefaultKind::Miscellaneous)
}

fn definition(reader: &mut ModelDecoder<'_>, index: usize, kind: LabelDefaultKind) -> Result<()> {
    reader.scope(&format!("definitions[{index}]"), |reader| {
        reader.scope("base", base)?;
        suffix(reader, kind)
    })
}

fn base(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(
        reader,
        &[
            (Signed, "08 0c 18 14 1c 20 24 28 44 2c 38"),
            (Word, "48"),
            (Text, "88 90"),
            (Unsigned, "5c"),
            (Signed, "10 58"),
        ],
    )?;
    formats(reader, 0..2)?;
    reader.scope("line_width_30", line_style::short_width)?;
    reader.scope("line_width_3c", line_style::short_width)?;
    formats(reader, 2..6)
}

fn formats(reader: &mut ModelDecoder<'_>, indices: std::ops::Range<usize>) -> Result<()> {
    for index in indices {
        reader.scope(
            &format!("formats[{index}]"),
            detail_prefix::character_format,
        )?;
    }
    Ok(())
}

fn suffix(reader: &mut ModelDecoder<'_>, kind: LabelDefaultKind) -> Result<()> {
    use LabelDefaultKind::*;
    match kind {
        Detail => detail(reader),
        Section => groups(reader, &[(Double, "c8 d0 d8"), (Signed, "e0 e4 ec f0 e8")]),
        Auxiliary => fields(reader, Signed, "c8"),
        Orthographic | Miscellaneous => fields(reader, Signed, "c8 cc"),
    }
}

fn detail(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x3b02 {
        groups(reader, &[(Unsigned, "cc"), (Signed, "c8")])?;
    }
    Ok(())
}
