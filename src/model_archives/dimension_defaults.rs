use super::raw_fields::{
    RawScalarKind::{self, Double, Signed, Text, Unsigned, Word},
    fields, groups,
};
use super::{
    ModelDecoder, detail_prefix,
    units::{self, ModelUnitKind},
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, line_style};

#[derive(Clone, Copy)]
enum DimensionDefaultKind {
    Linear,
    Angle,
    ArcLength,
    Radial,
    HoleCallout,
    Ordinate,
    Chamfer,
    Diameter,
    AngularOrdinate,
    Base,
}

const DEFINITIONS: &[DimensionDefaultKind] = &[
    DimensionDefaultKind::Linear,
    DimensionDefaultKind::Angle,
    DimensionDefaultKind::ArcLength,
    DimensionDefaultKind::Radial,
    DimensionDefaultKind::HoleCallout,
    DimensionDefaultKind::Ordinate,
    DimensionDefaultKind::Chamfer,
    DimensionDefaultKind::Diameter,
    DimensionDefaultKind::AngularOrdinate,
    DimensionDefaultKind::Base,
];

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    groups(reader, ROOT_FIELDS)?;
    if reader.version() > 0x375d {
        fields(reader, Signed, "d0 d4")?;
    }
    if reader.version() >= 0x3eef {
        fields(reader, Signed, "cc")?;
    }
    for (index, kind) in DEFINITIONS.iter().enumerate() {
        reader.scope(&format!("definitions[{index}]"), |reader| {
            reader.scope("base", |reader| base(reader, map))?;
            suffix(reader, *kind)
        })?;
    }
    Ok(())
}

const ROOT_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "68"),
    (Double, "08 10 18 20 28 30 38 40 48 50 58"),
    (Signed, "60 64 6c 70 74"),
    (Double, "78 80"),
    (Signed, "88 8c"),
    (Double, "90 98 a0"),
    (Unsigned, "d8"),
    (Signed, "a8"),
    (Double, "b0"),
    (Signed, "b8 bc c0 c4"),
];

fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    groups(reader, BASE_FIELDS)?;
    reader.scope("length_units", |reader| {
        units::read(reader, map, ModelUnitKind::Length)
    })?;
    reader.scope("angle_units", |reader| {
        units::read(reader, map, ModelUnitKind::Coded)
    })?;
    reader.scope("character_format", detail_prefix::character_format)?;
    reader.scope("line_format_d0", line_format)?;
    reader.scope("line_format_d8", line_format)?;
    fields(reader, Signed, "c0 78 9c")
}

const BASE_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "08 0c 10"),
    (Double, "18 20"),
    (Signed, "28"),
    (Double, "30 38"),
    (Signed, "40 44"),
    (Text, "48 50"),
    (Double, "58 60"),
    (Signed, "68 6c 70 74 80 84 88 8c"),
    (Word, "90"),
    (Signed, "94 98"),
    (Unsigned, "e0"),
];

fn line_format(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.text("name")?;
    reader.scope("weight", line_style::short_width)?;
    reader.i16("end_cap")?;
    Ok(())
}

fn suffix(reader: &mut ModelDecoder<'_>, kind: DimensionDefaultKind) -> Result<()> {
    use DimensionDefaultKind::*;
    match kind {
        Linear => linear(reader),
        Angle | HoleCallout => fields(reader, Unsigned, "100"),
        Radial => groups(reader, &[(Unsigned, "100"), (Signed, "104")]),
        Diameter => groups(reader, &[(Signed, "100"), (Unsigned, "104")]),
        Ordinate => groups(reader, ORDINATE_SUFFIX),
        AngularOrdinate => groups(reader, ANGULAR_ORDINATE_SUFFIX),
        Chamfer => groups(reader, CHAMFER_SUFFIX),
        ArcLength | Base => Ok(()),
    }
}

const ORDINATE_SUFFIX: &[(RawScalarKind, &str)] =
    &[(Signed, "100"), (Unsigned, "104"), (Double, "108")];
const ANGULAR_ORDINATE_SUFFIX: &[(RawScalarKind, &str)] =
    &[(Signed, "100"), (Unsigned, "104"), (Signed, "108 110 114")];
const CHAMFER_SUFFIX: &[(RawScalarKind, &str)] = &[
    (Signed, "100 104 108 10c"),
    (Double, "118 120"),
    (Signed, "110 128"),
];

fn linear(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(reader, &[(Unsigned, "108"), (Signed, "100")])?;
    if reader.version() > 0x3ef8 {
        fields(reader, Signed, "104")?;
    }
    Ok(())
}
