use super::raw_fields::RawScalarKind::{Double, Signed};
use super::{ModelDecoder, ModelObjectRole, body_features, raw_fields};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.scope("model_feature", |reader| {
        body_features::model_feature(reader, map)
    })?;
    reader.scope("attributes", |reader| attributes(reader, map))
}

fn attributes(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    raw_fields::groups(
        reader,
        &[
            (Signed, "10 14 18 1c 20 24"),
            (Double, "88 90 98"),
            (Signed, "b8"),
        ],
    )?;
    attachment(reader, map, "c0")?;
    reader.i32("word_140")?;
    double_vector(reader, "point_a0")?;
    reader.i32("word_160")?;
    attachment(reader, map, "168")?;
    raw_fields::groups(reader, &[(Double, "d0 d8 e0"), (Signed, "e8 ec")])?;
    reader.object("reference_f0", map, ModelObjectRole::Any)?;
    reader.scope("faceted_reference", faceted)?;
    final_attributes(reader, map)
}

fn final_attributes(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("word_1d8")?;
    reader.f64s(&[
        "double_1e0",
        "double_1e8",
        "double_1f0",
        "aspect_width",
        "aspect_height",
    ])?;
    reader.i32s(&["aspect_mode", "word_218"])?;
    reader.f64("double_220")?;
    reader.i32("word_228")?;
    attachment(reader, map, "230")?;
    reader.f64("double_240")
}

fn attachment(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offset: &str) -> Result<()> {
    reader.scope(&format!("attachment_{offset}"), |reader| {
        reader.object("object", map, ModelObjectRole::Any)?;
        reader.f64("value")
    })
}

fn faceted(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let kind = reader.u32("packed_flags")? & 15;
    match kind {
        0 => {}
        1 | 2 => faceted_points(reader, kind)?,
        3 => {
            double_vector(reader, "direction")?;
            double_vector(reader, "point")?;
        }
        4 => {
            double_vector(reader, "point")?;
            double_vector(reader, "direction")?;
        }
        actual => {
            return reader.checked(Err(super::invalid(format!(
                "camera faceted kind {actual}; expected 0..=4"
            ))));
        }
    }
    reader.text("name")
}

fn faceted_points(reader: &mut ModelDecoder<'_>, kind: u32) -> Result<()> {
    for index in 0..reader.signed_count("points.count", 24)? {
        double_vector(reader, &format!("points[{index}]"))?;
    }
    if kind == 2 {
        reader.i32("edge_word")?;
    }
    Ok(())
}
