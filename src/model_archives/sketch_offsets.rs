use super::{
    ModelDecoder, ModelObjectRole, model_lists, sketch_arrays, sketch_collections,
    sketch_dimensions,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn collections(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_collections::direct(reader, map, ("offset_count", "offsets"), offset)?;
    sketch_collections::direct(
        reader,
        map,
        ("sketch_offsets.count", "sketch_offsets"),
        sketch_offset,
    )
}

pub(super) fn sketch_offset(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    let count = reader.signed_count("indices.count", 4)?;
    unsigned_indices(reader, "indices", count)?;
    if count != 0 {
        reader.i32s(&["word_38", "word_34"])?;
        reader.f64("value_40")?;
        sketch_arrays::vectors(reader, "points", 2)?;
        reader.u16("word_90")?;
    }
    reader.object(
        "profile_data",
        map,
        ModelObjectRole::Declared(&["sgPGMProfileData_c"]),
    )?;
    reader.i32s(&["word_a4", "word_a8"])
}

fn unsigned_indices(reader: &mut ModelDecoder<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        reader.u32(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn offset(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    let count = sketch_collections::word_count(reader, "external_references.count")?;
    for index in 0..count {
        reader.object(
            &format!("external_references[{index}]"),
            map,
            ModelObjectRole::Declared(&["moSketchExtRef_w"]),
        )?;
    }
    reader.entries(count, "offset entity types")?;
    unsigned_indices(reader, "entity_types", count)?;
    reader.entries(count, "offset presence/handle records")?;
    offset_handles(reader, map, count)?;
    offset_suffix(reader, map, count)
}

fn offset_handles(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    count: usize,
) -> Result<()> {
    for index in 0..count {
        reader.scope(&format!("handles[{index}]"), |reader| {
            if reader.u16("presence")? == 1 {
                sketch_dimensions::handles(reader, map, &["handle"])?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

fn offset_suffix(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    count: usize,
) -> Result<()> {
    reader.f64("value_30")?;
    reader.u16("word_38")?;
    let kind = reader.u16("kind_4c")?;
    reader.u16("word_50")?;
    reader.object("dimension", map, ModelObjectRole::Any)?;
    reader.u16("word_60")?;
    reader.u16("word_68")?;
    reader.i32s(&["word_64", "word_c8", "word_48"])?;
    sketch_arrays::vectors(reader, "points", 2)?;
    offset_indices(reader, count, kind)
}

fn offset_indices(reader: &mut ModelDecoder<'_>, count: usize, kind: u16) -> Result<()> {
    if (7..=9).contains(&kind) {
        reader.entries(count, "offset indices")?;
        sketch_arrays::integers(reader, "indices", count)?;
        double_vector(reader, "vector_b0")?;
        reader.i32("word_a8")?;
        if kind != 7 && reader.version() > 0x2bbe {
            return super::part_objects::absent_payload(
                reader,
                "sketch offset kernel body",
                i32::from(kind),
            );
        }
    }
    Ok(())
}

pub(super) fn profile(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    model_lists::pointers(reader, map, "items", ModelObjectRole::Any)?;
    reader.u16("word_28")?;
    reader.f64("value_30")?;
    Ok(())
}

pub(super) fn loop_offset(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.i32s(&["word_8", "word_c"])?;
    model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
}

pub(super) fn loop_edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.u16("word_8")?;
    reader.object("array", map, ModelObjectRole::Declared(&["suObArray"]))
}

pub(super) fn sectioned_edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    loop_edge(reader, map)?;
    reader.scope("sections", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })
}

pub(super) fn section_handles(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for label in ["first", "second"] {
        reader.object(
            label,
            map,
            ModelObjectRole::Declared(&["sgSegmentHandle", "sgSplineHandle"]),
        )?;
    }
    reader.object("array", map, ModelObjectRole::Declared(&["suObArray"]))
}
