use super::display_records::{declared, optional_array};
use super::{ModelDecoder, ModelObjectRole, display_records, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, line_style, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["unknown_390", "flags_3e0"])?;
    double_vector(reader, "position")?;
    optional_array(reader, map, "array_298")?;
    let present = reader.i32("has_object_288")?;
    super::part_objects::absent_payload(reader, "display item object +288", present)?;
    geometry(reader, map)?;
    styles(reader, map)?;
    tail(reader, map)
}

fn geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(
        reader,
        map,
        "component_feature",
        &["moCompFeature_c", "moCompBodyFeature_c"],
    )?;
    double_vector(reader, "vector_400")?;
    reader.transform("transform_2a0")?;
    let count = reader.signed_count("indices_2b0.count", 4)?;
    sketch_arrays::integers(reader, "indices_2b0", count)?;
    if reader.u8("has_vector_2c0")? != 0 {
        double_vector(reader, "vector_2c0")?;
    }
    reader.f64("unknown_2c8")?;
    if reader.i32("has_transform_2d0")? != 0 {
        reader.transform("transform_2d0")?;
    }
    geometry_objects(reader, map)
}

fn geometry_objects(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    leaders(reader, map)?;
    reader.object("object_2d8", map, ModelObjectRole::Any)?;
    reader.f64("unknown_3b8")?;
    Ok(())
}

fn leaders(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("leaders.count", 2)? {
        declared(
            reader,
            map,
            &format!("leaders[{index}]"),
            &["moDisplayLeader_c"],
        )?;
    }
    Ok(())
}

fn styles(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    display_records::indexed_objects(reader, map, "arrow_sizes", "size", "moArrowSize_c")?;
    reader.i32("word_2f0")?;
    reader.scope("width_2f8", line_style::short_width)?;
    reader.i32s(&["word_2f4", "word_304"])?;
    reader.scope("width_30c", line_style::short_width)?;
    reader.i32s(&["word_308", "word_300", "word_314", "word_334", "word_338"])?;
    optional_array(reader, map, "array_4a0")?;
    reader.i32s(&["word_318", "word_31c"])?;
    reader.scope("width_320", line_style::short_width)?;
    reader.i32s(&["word_328", "word_32c"])
}

fn tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    display_records::indexed_objects(
        reader,
        map,
        "spline_leaders",
        "data",
        "moSplineLeaderData_c",
    )?;
    let present = reader.i32("has_watermark")?;
    super::part_objects::absent_payload(reader, "display item watermark", present)?;
    if reader.version() <= 0x3371 {
        return Ok(());
    }
    display_records::bounded_vectors(reader, "points_4f0", 4)?;
    if reader.version() < 0x3374 {
        return sketch_arrays::vectors(reader, "vectors_550", 1);
    }
    display_records::bounded_vectors(reader, "vectors_550", 2)
}
