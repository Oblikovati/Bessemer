use super::display_records::{declared, optional_point};
use super::{
    ModelDecoder, display_dimension::DisplayDimensionKind, display_records, sketch_arrays,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("handles.count", 2)? {
        reader.required_object(
            &format!("handles[{index}]"),
            map,
            super::ModelObjectRole::Declared(super::dimension_handles::CLASSES),
        )?;
    }
    for label in ["word_870", "word_874", "word_878"] {
        reader.u16(label)?;
    }
    for label in ["text_7f8", "text_800", "text_818", "text_820"] {
        reader.text(label)?;
    }
    double_vector(reader, "vector_880")?;
    reader.f64("unknown_8a8")?;
    Ok(())
}

pub(super) fn placement(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.f64s(&["unknown_8b0", "unknown_950", "unknown_a78"])?;
    sketch_arrays::vectors(reader, "points_958", 5)?;
    if reader.i32("has_data_a50")? != 0 {
        double_vector(reader, "data_a50.point")?;
        double_vector(reader, "data_a50.vector")?;
        reader.f64("data_a50.value")?;
    }
    for label in [
        "word_898", "word_89c", "word_8a0", "word_8c0", "word_8e4", "word_8ec", "word_8f4",
    ] {
        reader.u16(label)?;
    }
    double_vector(reader, "vector_a38")
}

pub(super) fn references(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: DisplayDimensionKind,
) -> Result<()> {
    if kind != DisplayDimensionKind::Base {
        point_references(reader, map)?;
        sketch_arrays::doubles(reader, "reference_parameters", 2)?;
        optional_point(reader, "point_590")?;
        optional_point(reader, "point_598")?;
    }
    reader.u16("word_8e8")?;
    reader.u16("word_8f0")?;
    reader.u32("flags_8a4")?;
    display_records::optional_array(reader, map, "array_a58")?;
    display_records::optional_array(reader, map, "array_a60")
}

pub(super) fn parameters(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    let width = reader.u32("element_width")?;
    if width == 0 {
        return Ok(());
    }
    if width != 8 {
        return reader.checked(Err(crate::BessemerError::new(
            "model archive",
            "display parameters",
            crate::FailureClass::Unsupported,
            format!("parameter element width {width}; expected 0 or 8"),
        )));
    }
    reader.u32("metadata_1")?;
    reader.u32("metadata_2")?;
    let count = reader.unsigned_count("count", 8)?;
    sketch_arrays::doubles(reader, "values", count)
}

fn point_references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..2 {
        declared(
            reader,
            map,
            &format!("point_references[{index}]"),
            &["moPointRef_w"],
        )?;
    }
    Ok(())
}
