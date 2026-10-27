use super::{ModelDecoder, ModelObjectRole, body_features, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("keeper_800", |reader| keeper(reader, map))?;
    reader.i32("raw_814")?;
    reader.required_object(
        "data_808",
        map,
        ModelObjectRole::Declared(&["moPLineData_c", "moPLineProject_c"]),
    )
}

pub(super) fn keeper(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("surface_ids.count", 2)? {
        reader.scope(&format!("surface_ids[{index}]"), |reader| {
            super::boundary_references::entity_id(reader, map)
        })?;
    }
    for label in ["array_68", "array_88"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    words(reader, "raw_words_c8")?;
    model_lists::pointers(reader, map, "array_48", ModelObjectRole::Any)?;
    model_lists::unsigned_words(reader, "raw_dwords_a8")?;
    reader.i32s(&["raw_114", "raw_118"])
}

pub(super) fn data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.required_object(
        "owner",
        map,
        ModelObjectRole::Declared(&["moFeature_c", "moPLine_c"]),
    )
}

pub(super) fn project(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    project_prefix(reader, map)?;
    reader.i32s(&["raw_20", "raw_24"])
}

fn project_prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    data(reader, map)?;
    reader.scope("array_10", |reader| {
        body_features::nullable_array(reader, map)
    })
}

pub(super) fn word_array(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    words(reader, "items")
}

fn words(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    let count = reader.count_with_minimum(&format!("{label}.count"), 2)?;
    sketch_arrays::words(reader, label, count)
}
