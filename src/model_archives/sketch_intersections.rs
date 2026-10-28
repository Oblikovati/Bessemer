use super::geometric_references::LINE_REFERENCE_CLASSES;
use super::{
    ModelDecoder, ModelObjectRole, model_lists, sketch_arrays, sketch_collections,
    sketch_dimensions, sketch_logical,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn collections(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_collections::direct(
        reader,
        map,
        ("styled_curves.count", "styled_curves"),
        styled,
    )?;
    sketch_collections::direct(
        reader,
        map,
        ("intersections.count", "intersections"),
        intersection,
    )
}

pub(super) fn styled(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("handles.count", 2)? {
        sketch_dimensions::handles(reader, map, &[&format!("handles[{index}]")])?;
    }
    reader.scope("states", sketch_logical::segment_states)?;
    reader.i32s(&["word_30", "word_40", "word_34", "word_38"])?;
    reader.u16("word_3c")?;
    reader.f64s(&["value_10", "value_8"])
}

pub(super) fn intersection(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    if reader.u16("marker")? != 0 {
        return Ok(());
    }
    reader.u32("raw_flags_6c")?;
    let count = reader.i32("handle_count")?;
    reader.i32("raw_last_edge_count_58")?;
    references(reader, map)?;
    let count = reader.counted_entries("handle_count", i64::from(count.max(0)), 2)?;
    for index in 0..count {
        sketch_dimensions::handles(reader, map, &[&format!("handles_40[{index}]")])?;
    }
    reader.scope("dynamic_words_50", dynamic_words)?;
    intersection_edges(reader, map, count)?;
    intersection_suffix(reader, map)
}

fn references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // #46 / C-279: base-typed pointers can hold reviewed concrete wrapper subclasses.
    for offset in ["10", "18", "20", "28"] {
        reader.object(
            &format!("reference_{offset}"),
            map,
            ModelObjectRole::Declared(super::model_references::WRAPPER_CLASSES),
        )?;
    }
    for offset in ["30", "38"] {
        reader.object(
            &format!("bent_references_{offset}"),
            map,
            ModelObjectRole::Declared(&["bentRefArr"]),
        )?;
    }
    Ok(())
}

fn intersection_edges(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    count: usize,
) -> Result<()> {
    let count = reader.counted_entries("intersection edges", count as i64, 100)?;
    for index in 0..count {
        reader.scope(&format!("edges_48[{index}]"), |reader| edge(reader, map))?;
    }
    Ok(())
}

pub(super) fn dynamic_words(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let width = reader.u32("element_width")?;
    if width == 0 {
        return Ok(());
    }
    if width % 4 != 0 {
        return reader.checked(Err(super::invalid(format!(
            "dynamic element width {width}; expected a multiple of 4"
        ))));
    }
    reader.u32("metadata_1")?;
    reader.u32("metadata_2")?;
    let count = reader.unsigned_count("count", width as usize)?;
    dynamic_budget(reader, count, (width / 4) as usize)?;
    dynamic_values(reader, count, (width / 4) as usize)
}

fn dynamic_budget(reader: &mut ModelDecoder<'_>, count: usize, width: usize) -> Result<()> {
    let words = count.checked_mul(width).ok_or_else(|| {
        super::invalid(format!(
            "dynamic array {count} * {width}; expected representable word count"
        ))
    });
    let words = reader.checked(words)?;
    reader.entries(words, "dynamic array words")
}

fn dynamic_values(reader: &mut ModelDecoder<'_>, count: usize, width: usize) -> Result<()> {
    for index in 0..count {
        for column in 0..width {
            reader.u32(&format!("values[{index}][{column}]"))?;
        }
    }
    Ok(())
}

pub(super) fn edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_158", "raw_15c", "raw_160", "raw_164", "raw_168"])?;
    for offset in ["38", "80", "c8", "110"] {
        reader.scope(&format!("group_{offset}"), |reader| {
            edge_reference(reader, map)
        })?;
    }
    Ok(())
}

fn edge_reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.u16("raw_00")?;
    reader.i32s(&["raw_20", "raw_24", "raw_28"])?;
    reader.object(
        "reference_08",
        map,
        ModelObjectRole::Declared(&["moLoopRef_c"]),
    )?;
    for offset in ["10", "18"] {
        reader.object(
            &format!("reference_{offset}"),
            map,
            ModelObjectRole::Declared(&["moEdgeRef_c"]),
        )?;
    }
    Ok(())
}

fn intersection_suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.u8("transform_presence")? != 0 {
        reader.transform("transform_60")?;
    }
    if reader.version() > 0x2b71 {
        reader.u16("raw_70")?;
        reader.u16("raw_74")?;
        reader.object(
            "handle_list_78",
            map,
            ModelObjectRole::Declared(&["suObList"]),
        )?;
        reader.object("arc_80", map, ModelObjectRole::Declared(&["sgArcHandle"]))?;
    }
    late_suffix(reader, map)
}

fn late_suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x2b98 {
        late_points(reader, map)?;
    }
    if reader.version() >= 0x2be0 {
        reader.f64("raw_b0")?;
    }
    Ok(())
}

fn late_points(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["88", "8c", "90", "a0"] {
        reader.u16(&format!("raw_{offset}"))?;
    }
    reader.object(
        "line_98",
        map,
        ModelObjectRole::Declared(LINE_REFERENCE_CLASSES),
    )?;
    let count = reader.i32("point_count")?;
    let count = reader.counted_entries("point_count", i64::from(count.max(0)), 24)?;
    sketch_arrays::vectors(reader, "raw_points_a8", count)
}

pub(super) fn loop_reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::boundary_references::base(reader, map)?;
    reader.u32("raw_f0")?;
    Ok(())
}

pub(super) fn bent_references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
}
