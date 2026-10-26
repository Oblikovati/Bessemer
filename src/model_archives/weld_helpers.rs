use super::raw_fields::{
    RawScalarKind::{self, Octet, Signed, Text},
    fields, groups,
};
use super::{ModelDecoder, ModelObjectRole, datum_helpers, detail_helpers, detail_prefix};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("helpers[0]", weld)?;
    reader.scope("helpers[1]", |reader| gtol(reader, map))
}

pub(super) fn weld(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    fields(reader, Octet, "discarded_08")?;
    for row in 0..2 {
        for base in (0x10..0x90).step_by(0x10) {
            fields(reader, Text, &format!("{:x}", base + 8 * row))?;
        }
    }
    groups(reader, WELD_FIELDS)?;
    reader.scope("format_f0", detail_prefix::character_format)?;
    fields(reader, Octet, "150")?;
    reader.scope("format_158", detail_prefix::character_format)?;
    fields(reader, Octet, "154")?;
    reader.scope("format_1b8", detail_prefix::character_format)?;
    reader.scope("surface_finish_260", detail_helpers::surface_finish)?;
    reader.scope("surface_finish_268", detail_helpers::surface_finish)
}

const WELD_FIELDS: &[(RawScalarKind, &str)] = &[
    (Text, "90"),
    (Octet, "98 9c a0 a4"),
    (Text, "a8"),
    (Octet, "b0 b4"),
    (Signed, "b8 c0"),
    (Octet, "c8 d0 e0"),
    (Signed, "bc c4"),
    (Octet, "cc d4 e4"),
    (Signed, "d8"),
    (Octet, "dc e8"),
    (Text, "218 220 228 230 238"),
    (Octet, "240 244 248 24c 250 254 258"),
];

pub(super) fn gtol(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("frame_count", 27)? {
        reader.scope(&format!("frames[{index}]"), |reader| frame(reader, map))?;
    }
    groups(reader, GTOL_FIELDS)?;
    if reader.version() >= 0x3751 {
        groups(reader, GTOL_EXTRA)?;
    }
    fields(reader, Octet, "9c ac 110 114")?;
    reader.scope("format_b0", detail_prefix::character_format)?;
    reader.scope("annotation_118", detail_helpers::annotation)?;
    fields(reader, Octet, "a0 44 48 a4 a8")?;
    if reader.version() > 0x3714 {
        fields(reader, Octet, "1e0")?;
    }
    Ok(())
}

const GTOL_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "30 34"),
    (Text, "38"),
    (Octet, "40 4c 50 54"),
    (Signed, "58"),
    (Octet, "68"),
    (Text, "70 78"),
];
const GTOL_EXTRA: &[(RawScalarKind, &str)] =
    &[(Octet, "80"), (Text, "88 90"), (Octet, "98"), (Text, "60")];

fn frame(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    groups(reader, FRAME_PREFIX)?;
    for member in ["58", "80", "a8", "d0", "f8", "120"] {
        reader.scope(&format!("strings_{member}"), detail_prefix::strings)?;
    }
    fields(reader, Signed, "20 24")?;
    if reader.version() >= 0x3715 {
        frame_datums(reader, map)?;
    }
    if reader.version() > 0x3718 {
        indicators(reader)?;
    }
    Ok(())
}

const FRAME_PREFIX: &[(RawScalarKind, &str)] = &[
    (Text, "08"),
    (Octet, "28"),
    (Text, "30 38"),
    (Octet, "40"),
    (Text, "48 50"),
];

fn frame_datums(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let presence = reader.u16("datum_presence_148")?;
    reader.entries(2, "gtol tolerance slots")?;
    for index in 0..2 {
        reader.object(
            &format!("tolerances[{index}]"),
            map,
            ModelObjectRole::Declared(&["moGTolDlgDataTol_c"]),
        )?;
    }
    if presence == 0 {
        return Ok(());
    }
    reader.entries(3, "gtol datum slots")?;
    for index in 0..3 {
        reader.scope(&format!("datums[{index}]"), datum_helpers::read)?;
    }
    Ok(())
}

fn indicators(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("indicator_count", 6)? {
        reader.scope(&format!("indicators[{index}]"), |reader| {
            groups(reader, &[(Signed, "00"), (Text, "08 10")])
        })?;
    }
    Ok(())
}
