use super::raw_fields::{
    RawScalarKind::{self, Double, Octet, Signed, Text},
    fields, groups,
};
use super::{ModelDecoder, detail_prefix};
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    for index in 0..8 {
        reader.f32(&format!("print_widths.line_weights[{index}]"))?;
    }
    groups(reader, EARLY_FIELDS)?;
    reader.scope("format_a0", detail_prefix::character_format)?;
    reader.scope("format_a8", detail_prefix::character_format)?;
    groups(reader, MIDDLE_FIELDS)?;
    if reader.version() > 0x27d1 {
        fields(reader, Signed, "1fc")?;
    }
    if reader.version() > 0x2bb5 {
        fields(reader, Signed, "15c 160")?;
    }
    late(reader)
}

const EARLY_FIELDS: &[(RawScalarKind, &str)] = &[
    (Double, "2bd0"),
    (Signed, "2bf4 2bf8"),
    (Octet, "2bfc 2c00"),
    (Signed, "2c0c 2c10 50 2c04 2c08 80"),
    (Double, "88"),
    (Double, "2c18 2c20 2c28 2c30 2c38"),
    (Signed, "2c40 2c44 2c48 2c4c 2c50 2c54 2c58 2c5c 98"),
];
const MIDDLE_FIELDS: &[(RawScalarKind, &str)] = &[
    (Signed, "2c64 2c68 19c 1bc"),
    (Text, "1c0 1c8 1d0 1d8"),
    (Signed, "1e0 1e4 1e8 1f8"),
    (Text, "1f0"),
];

fn late(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x335e {
        groups(reader, &[(Signed, "218"), (Text, "200 208 210")])?;
    }
    if reader.version() > 0x373f {
        fields(reader, Signed, "220 224")?;
    }
    if reader.version() >= 0x3b15 {
        fields(reader, Signed, "2c6c")?;
    }
    if reader.version() > 0x41a1 {
        fields(reader, Signed, "21c")?;
    }
    if reader.version() > 0x42e5 {
        fields(reader, Signed, "1b8")?;
    }
    Ok(())
}
