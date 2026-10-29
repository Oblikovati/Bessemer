use super::raw_fields::{
    RawScalarKind::{self, Double, Signed, Text, Unsigned, Word},
    fields, groups,
};
use super::{ModelDecoder, detail_prefix};
use crate::Result;
use crate::archive_codec::{MaterialRead, line_style};

#[derive(Clone, Copy)]
enum AnnotationDefaultKind {
    Balloon,
    Note,
    Datum,
    GeometricTolerance,
    SurfaceFinish,
    Weld,
    BendNote,
    Base,
    RevisionCloud,
    ViewLocation,
}

const DEFINITIONS: &[AnnotationDefaultKind] = &[
    AnnotationDefaultKind::Balloon,
    AnnotationDefaultKind::Note,
    AnnotationDefaultKind::Datum,
    AnnotationDefaultKind::GeometricTolerance,
    AnnotationDefaultKind::SurfaceFinish,
    AnnotationDefaultKind::Weld,
    AnnotationDefaultKind::BendNote,
    AnnotationDefaultKind::Base,
    AnnotationDefaultKind::RevisionCloud,
    AnnotationDefaultKind::ViewLocation,
];

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    groups(reader, ROOT_FIELDS)?;
    if reader.version() >= 0x2f70 {
        fields(reader, Signed, "18")?;
    }
    if reader.version() >= 0x376a {
        fields(reader, Signed, "34")?;
    }
    for (index, kind) in DEFINITIONS.iter().enumerate() {
        reader.scope(&format!("definitions[{index}]"), |reader| {
            reader.scope("base", base)?;
            suffix(reader, *kind)
        })?;
    }
    Ok(())
}

const ROOT_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "10"),
    (Double, "08"),
    (Signed, "14 1c 20 24 28"),
    (Unsigned, "38"),
    (Signed, "2c 30"),
];

fn base(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(
        reader,
        &[
            (Signed, "08"),
            (Double, "10"),
            (Signed, "18 1c 20 24 34 40"),
            (Word, "44"),
        ],
    )?;
    reader.scope("line_width_2c", line_style::short_width)?;
    reader.scope("line_width_38", line_style::short_width)?;
    reader.scope("character_format", detail_prefix::character_format)?;
    fields(reader, Signed, "28")
}

fn suffix(reader: &mut ModelDecoder<'_>, kind: AnnotationDefaultKind) -> Result<()> {
    use AnnotationDefaultKind::*;
    match kind {
        Balloon => groups(reader, BALLOON_FIELDS),
        Note => groups(
            reader,
            &[(Signed, "58 5c 60"), (Double, "68 70 78"), (Signed, "80")],
        ),
        Datum => fields(reader, Signed, "58 68 6c 60 64 5c"),
        GeometricTolerance => geometric_tolerance(reader),
        SurfaceFinish => groups(reader, &[(Signed, "58 5c"), (Double, "60 68")]),
        Weld => weld(reader),
        BendNote => groups(reader, &[(Signed, "58 5c 60"), (Double, "68")]),
        Base => Ok(()),
        RevisionCloud => fields(reader, Double, "58"),
        ViewLocation => groups(reader, VIEW_LOCATION_FIELDS),
    }
}

const BALLOON_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "58 5c 60 64 68 6c"),
    (Text, "70 78"),
    (Unsigned, "80"),
    (Double, "90 88"),
    (Signed, "b4 b8"),
    (Text, "c0"),
    (Double, "98 a0 a8"),
    (Signed, "b0"),
];
const VIEW_LOCATION_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "58 5c 60 64"),
    (Double, "68"),
    (Signed, "70"),
    (Double, "78"),
    (Signed, "74"),
];

fn geometric_tolerance(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() >= 0x3738 {
        fields(reader, Signed, "58")?;
    }
    if reader.version() >= 0x3b05 {
        fields(reader, Signed, "5c")?;
    }
    if reader.version() > 0x3ef3 {
        fields(reader, Word, "60")?;
    }
    Ok(())
}

fn weld(reader: &mut ModelDecoder<'_>) -> Result<()> {
    fields(reader, Signed, "58")?;
    reader.scope("character_format_60", detail_prefix::character_format)
}
