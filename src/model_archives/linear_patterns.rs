use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::geometric_references::LINE_REFERENCE_CLASSES;
use super::{ModelDecoder, ModelObjectRole, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    common(reader, map)?;
    reader.scope("linear_suffix", |reader| suffix(reader, map))
}

pub(super) fn common(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    common_prefix(reader, map)?;
    reader.scope("variation", |reader| {
        super::pattern_variation::read(reader, map)
    })?;
    reader.scope("tail", |reader| tail(reader, map))
}

/// Retain the containing node across complete common-pattern prefix members.
/// Example: `common_prefix(reader, map)?.unknown_2c`.
pub(super) fn common_prefix(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<super::node::ModelNodePrefix> {
    let node = reader.scope("body", |reader| {
        super::body_features::base_with_node(reader, map)
    })?;
    reader.scope("header", |reader| header(reader, map))?;
    reader.scope("collections", |reader| collections(reader, map))?;
    reader.scope("shape_data", |reader| {
        super::pattern_shape::read(reader, map)
    })?;
    reader.u8("byte_1048")?;
    reader.u8("byte_104c")?;
    Ok(node)
}

fn header(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["seeds", "list_9d0"] {
        declared(reader, map, label, &["suObList"])?;
    }
    reader.i32("word_9d8")?;
    declared(reader, map, "array_818", &["suObArray"])?;
    reader.u8("byte_9f4")?;
    reader.i32("word_9f8")?;
    declared(reader, map, "list_8d0", &["suObList"])?;
    header_versioned(reader, map)
}

fn header_versioned(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() >= 0x42cf {
        declared(reader, map, "list_8d8", &["suObList"])?;
        reader.i32("word_940")?;
    } else {
        reader.u8("byte_940")?;
    }
    declared(
        reader,
        map,
        "matrix_index_helper",
        &["moMatrixIndexHelper_c"],
    )
}

fn map_entries(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    integers: bool,
) -> Result<()> {
    let minimum = if integers { 6 } else { 4 };
    for index in 0..reader.count_with_minimum(&format!("{label}.count"), minimum)? {
        reader.object(&format!("{label}[{index}][0]"), map, ModelObjectRole::Any)?;
        if integers {
            reader.i32(&format!("{label}[{index}][1]"))?;
        } else {
            reader.object(&format!("{label}[{index}][1]"), map, ModelObjectRole::Any)?;
        }
    }
    Ok(())
}

pub(super) fn collections(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    map_entries(reader, map, "map_838", true)?;
    map_entries(reader, map, "map_868", false)?;
    reader.u8("byte_9e8")?;
    array(reader, map, "array_f48")
}

pub(super) fn array(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    reader.scope(label, |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })
}

pub(super) fn matrix_helper(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    array(reader, map, "array_10")?;
    array(reader, map, "array_30")?;
    reader.i32("raw_08")?;
    Ok(())
}

/// Preserve the established pattern tail; e.g. `tail(reader, map)?`.
pub(super) fn tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "list_988", &["suObList"])?;
    reader.i32("word_1120")?;
    if reader.version() > 0x2f86 {
        array(reader, map, "list_8e0")?;
    }
    if reader.version() > 0x2f93 {
        let count = reader.unsigned_count("indices_928.count", 4)?;
        sketch_arrays::integers(reader, "indices_928", count)?;
    }
    if reader.version() > 0x3745 {
        reader.u8("byte_1124")?;
    }
    if reader.version() >= 0x42dc {
        reader.u8("byte_1128")?;
    }
    Ok(())
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["1250", "1258", "1260", "1268"] {
        declared(reader, map, &format!("dimension_{offset}"), DIMENSIONS)?;
    }
    reader.u8("byte_1290")?;
    reader.u8("byte_1294")?;
    for label in ["object_1270", "object_1278"] {
        reader.object(label, map, ModelObjectRole::Any)?;
    }
    for offset in ["1280", "1288"] {
        declared(
            reader,
            map,
            &format!("line_{offset}"),
            LINE_REFERENCE_CLASSES,
        )?;
    }
    reader.u8("byte_129c")?;
    suffix_references(reader, map)
}

fn suffix_references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for (words, refs) in [
        (["12d8", "12dc"], ["12e0", "12e8"]),
        (["12f0", "12f4"], ["12f8", "1300"]),
    ] {
        for offset in words {
            reader.i32(&format!("word_{offset}"))?;
        }
        for offset in refs {
            declared(
                reader,
                map,
                &format!("reference_{offset}"),
                &["moRefWrapper_c"],
            )?;
        }
    }
    suffix_dimensions(reader, map)
}

fn suffix_dimensions(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["word_1308", "word_130c"])?;
    for offset in ["1310", "1318"] {
        declared(reader, map, &format!("dimension_{offset}"), DIMENSIONS)?;
    }
    reader.i32s(&["word_1320", "word_1324"])?;
    if reader.version() >= 0x3f00 {
        reader.u8("byte_1298")?;
    }
    Ok(())
}
