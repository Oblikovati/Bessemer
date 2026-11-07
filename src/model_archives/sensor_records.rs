use super::raw_fields::{
    self,
    RawScalarKind::{Double, Signed, Single, Text},
};
use super::{ModelDecoder, ModelObjectRole, configuration_io};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| {
        super::body_features::model_feature(reader, map)
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

pub(super) fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("data", data)?;
    for (offset, classes) in [
        ("558", super::model_references::WRAPPER_CLASSES),
        ("570", super::model_references::WRAPPER_CLASSES),
        ("588", &["moDispObjRef_c"]),
    ] {
        let label = format!("references_{offset}");
        for index in 0..reader.unsigned_count(&format!("{label}.count"), 2)? {
            reader.object(
                &format!("{label}[{index}]"),
                map,
                ModelObjectRole::Declared(classes),
            )?;
        }
    }
    Ok(())
}

pub(super) fn data(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_08")?;
    let kind = reader.i32("raw_0c")?;
    raw_fields::groups(
        reader,
        &[
            (Signed, "10 14 18 1c 20 24 34 38 3c 40"),
            (Double, "48 50"),
            (Signed, "78 8c"),
            (Single, "90"),
            (Text, "98"),
            (Signed, "a0 a4 a8 ac"),
        ],
    )?;
    let precision = if reader.archive_version().value() < 0x3765 {
        Single
    } else {
        Double
    };
    raw_fields::fields(reader, precision, "b0 b8")?;
    reader.text("raw_c0")?;
    for index in 0..reader.unsigned_count("points.count", 64)? {
        reader.scope(&format!("points[{index}]"), point)?;
    }
    middle(reader)?;
    tail(reader)?;
    conditional_tail(reader, kind)
}

fn point(reader: &mut ModelDecoder<'_>) -> Result<()> {
    raw_fields::groups(
        reader,
        &[
            (Signed, "00 04 08"),
            (Single, "0c 10 14 18 1c 20"),
            (Signed, "24 28 2c 34 30 38 3c"),
        ],
    )
}

fn middle(reader: &mut ModelDecoder<'_>) -> Result<()> {
    raw_fields::groups(
        reader,
        &[
            (Signed, "f4 f8 fc 100"),
            (Text, "d0 d8 e0 e8"),
            (Signed, "f0 104 108"),
            (Double, "110"),
            (Signed, "28 2c 30"),
            (Text, "80"),
            (Signed, "88 15c 160 164 168"),
        ],
    )?;
    // The native serializer writes +0x88 twice; neither occurrence replaces the other.
    reader.i32("raw_88_repeated")?;
    Ok(())
}

fn tail(reader: &mut ModelDecoder<'_>) -> Result<()> {
    raw_fields::groups(
        reader,
        &[
            (Text, "180 c8 1b0"),
            (Double, "190"),
            (Signed, "158"),
            (Double, "198"),
            (Signed, "1a0"),
            (Double, "1a8"),
            (Text, "188"),
            (Signed, "1b8 58 1bc 1c0"),
        ],
    )
}

fn conditional_tail(reader: &mut ModelDecoder<'_>, kind: i32) -> Result<()> {
    if kind == 0x47 {
        reader.text("raw_1c8")?;
        for index in 0..configuration_io::positive_count(reader, "string_count_1d0", 1)? {
            reader.text(&format!("strings_1d8[{index}]"))?;
        }
    }
    if reader.archive_version().value() > 0x27e3 {
        reader.i32("raw_1f0")?;
    }
    if reader.archive_version().value() >= 0x3b16 {
        reader.i32("raw_7c")?;
    }
    Ok(())
}
