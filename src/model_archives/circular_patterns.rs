use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, linear_patterns, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("pattern", |reader| linear_patterns::common(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

pub(super) fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let primary = initial_dimensions(reader, map)?;
    reader.object("object_1150", map, ModelObjectRole::Any)?;
    declared(
        reader,
        map,
        "line_1158",
        super::geometric_references::LINE_REFERENCE_CLASSES,
    )?;
    reader.u8("raw_1160")?;
    let alternate = angular_dimension(reader, map, "dimension_1198")?;
    reader.u8("raw_11a8")?;
    angular_dimension(reader, map, "dimension_11a0")?;
    for offset in ["11ac", "11b0", "11b4", "11c0", "11c4"] {
        reader.u8(&format!("raw_{offset}"))?;
    }
    if !(primary || alternate) {
        return reader.checked(Err(super::invalid(
            "circular dimensions 1138=null and 1198=null; expected at least one nonnull dimension",
        )));
    }
    collections(reader)
}

fn initial_dimensions(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<bool> {
    let mut primary = false;
    for offset in ["1130", "1138", "1140", "1148"] {
        let record = reader.object_record(
            &format!("dimension_{offset}"),
            map,
            ModelObjectRole::Declared(super::display_dimension::CLASSES),
        )?;
        if offset == "1138" {
            primary = !matches!(record, ObjectRecord::Null);
        }
    }
    Ok(primary)
}

fn angular_dimension(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<bool> {
    let record = reader.object_record(
        label,
        map,
        ModelObjectRole::Declared(&["moDisplayAngularDim_c"]),
    )?;
    Ok(!matches!(record, ObjectRecord::Null))
}

fn collections(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("map_11d0.count", 8)? {
        reader.scope(&format!("map_11d0[{index}]"), |reader| {
            reader.i32("raw_key")?;
            words(reader, "raw_values")
        })?;
    }
    words(reader, "raw_words_1200")?;
    if reader.version() > 0x3efa {
        double_vector(reader, "point_1168")?;
        double_vector(reader, "vector_1180")?;
    }
    Ok(())
}

fn words(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    let count = reader.signed_count(&format!("{label}.count"), 4)?;
    sketch_arrays::integers(reader, label, count)
}
