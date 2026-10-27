use super::display_records::declared;
use super::{ModelDecoder, connection_points, display_dimension, geometric_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(
        reader,
        map,
        "line_reference_710",
        geometric_references::LINE_REFERENCE_CLASSES,
    )?;
    reader.i32("raw_73c")?;
    double_vector(reader, "vector_6f8")?;
    declared(reader, map, "dimension_740", display_dimension::CLASSES)?;
    reader.i32("raw_790")?;
    for offset in ["798", "7f8", "7d8", "7b8"] {
        reader.scope(
            &format!("parameter_{offset}"),
            connection_points::configurable,
        )?;
    }
    compatibility(reader)?;
    suffix(reader, map)
}

fn compatibility(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.f64s(&["compatibility_double_0", "compatibility_double_1"])?;
    reader.i32s(&["raw_818", "compatibility_word"])?;
    reader.text("raw_string_820")?;
    reader.text("compatibility_string")?;
    reader.scope("compatibility_parameter", connection_points::configurable)
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["dimension_748", "dimension_750"] {
        declared(reader, map, label, display_dimension::CLASSES)?;
    }
    reader.i32("raw_760")?;
    double_vector(reader, "vector_768")?;
    declared(
        reader,
        map,
        "line_reference_718",
        geometric_references::LINE_REFERENCE_CLASSES,
    )?;
    reader.i32("raw_780")?;
    declared(reader, map, "dimension_788", display_dimension::CLASSES)?;
    reader.i32("raw_81c")?;
    reader.text("raw_string_828")
}
