use super::raw_fields::{
    RawScalarKind::{self, Double, Signed, Text, Unsigned, Word},
    fields, groups,
};
use super::{ModelDecoder, detail_prefix};
use crate::Result;
use crate::archive_codec::{MaterialRead, line_style};

#[derive(Clone, Copy)]
enum TableDefaultKind {
    General,
    Revision,
    Bom,
    Hole,
    TitleBlock,
    Weld,
    Bend,
    Punch,
    Family,
}

const FIRST: &[TableDefaultKind] = &[
    TableDefaultKind::General,
    TableDefaultKind::Revision,
    TableDefaultKind::Bom,
    TableDefaultKind::Hole,
];
const LAST: &[TableDefaultKind] = &[
    TableDefaultKind::TitleBlock,
    TableDefaultKind::Weld,
    TableDefaultKind::Bend,
    TableDefaultKind::Punch,
];

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    fields(reader, Signed, "08")?;
    for (index, kind) in FIRST.iter().enumerate() {
        definition(reader, index, *kind)?;
    }
    reader.scope("character_format_328", detail_prefix::character_format)?;
    for (index, kind) in LAST.iter().enumerate() {
        definition(reader, index + 4, *kind)?;
    }
    if reader.version() > 0x42f1 {
        definition(reader, 8, TableDefaultKind::Family)?;
    }
    fields(reader, Double, "330 338")
}

fn definition(reader: &mut ModelDecoder<'_>, index: usize, kind: TableDefaultKind) -> Result<()> {
    reader.scope(&format!("definitions[{index}]"), |reader| {
        reader.scope("base", base)?;
        suffix(reader, kind)
    })
}

fn base(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(
        reader,
        &[
            (Signed, "2c 28"),
            (Unsigned, "08"),
            (Word, "0c"),
            (Signed, "38"),
        ],
    )?;
    reader.scope("line_width_18", line_style::short_width)?;
    reader.scope("line_width_20", line_style::short_width)?;
    reader.scope("character_format", detail_prefix::character_format)
}

fn suffix(reader: &mut ModelDecoder<'_>, kind: TableDefaultKind) -> Result<()> {
    use TableDefaultKind::*;
    match kind {
        General | TitleBlock => Ok(()),
        Revision => groups(reader, REVISION_FIELDS),
        Bom => bom(reader),
        Hole => groups(reader, HOLE_FIELDS),
        Weld => weld(reader),
        Bend => fields(reader, Signed, "40 44 4c 48"),
        Punch => groups(reader, PUNCH_FIELDS),
        Family => fields(reader, Unsigned, "40"),
    }
}

const REVISION_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "40 44 48"),
    (Unsigned, "58"),
    (Signed, "4c"),
    (Text, "50"),
];
const HOLE_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "40 44"),
    (Double, "48 50"),
    (Signed, "58 5c 60 68 6c 70 74 78 64 7c"),
];
const PUNCH_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "40 44"),
    (Double, "58 60"),
    (Signed, "48 4c 6c 50 70 68"),
];

fn bom(reader: &mut ModelDecoder<'_>) -> Result<()> {
    fields(reader, Unsigned, "40")?;
    if reader.version() > 0x2b81 {
        fields(reader, Text, "48 50 58")?;
    }
    Ok(())
}

fn weld(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x375f {
        fields(reader, Signed, "40")?;
    }
    Ok(())
}
