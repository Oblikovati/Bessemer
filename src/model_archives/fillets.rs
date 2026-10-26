use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::raw_fields::{
    RawScalarKind::{Octet, Signed},
    fields,
};
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    for (label, stage) in STAGES {
        reader.scope(label, |reader| stage(reader, map))?;
    }
    Ok(())
}

const STAGES: &[(&str, super::sketch_geometry::DirectReader)] = &[
    ("references", references),
    ("settings", settings),
    ("cache", super::fillet_cache::read),
    ("tail", tail),
];

pub(super) fn keeper(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("surface_ids.count", 2)? {
        reader.scope(&format!("surface_ids[{index}]"), |reader| {
            super::boundary_references::entity_id(reader, map)
        })?;
    }
    reader.scope("references", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })
}

fn references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("edge_keeper", |reader| keeper(reader, map))?;
    reader.scope("face_keeper", |reader| keeper(reader, map))?;
    for offset in ["840", "848", "850", "858", "870"] {
        declared(reader, map, &format!("list_{offset}"), &["suObList"])?;
    }
    reader.scope("array_8d8", |reader| nullable_array(reader, map))?;
    reader.scope("array_8f0", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })?;
    declared(reader, map, "list_878", &["suObList"])
}

fn nullable_array(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.i32("presence")? == 0 {
        return Ok(());
    }
    reader.scope("array", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })
}

fn settings(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "dimension_828", DIMENSIONS)?;
    declared(reader, map, "list_860", &["suObList"])?;
    reader.u8("byte_800")?;
    declared(reader, map, "list_868", &["suObList"])?;
    reader.u8("byte_814")?;
    reader.i32("word_8e0")?;
    reader.u8("byte_928")?;
    declared(reader, map, "point_880", &["moPointRef_w"])?;
    setting_values(reader)?;
    if reader.i32("reference_presence")? != 0 {
        declared(reader, map, "reference_808", &["moFeatureRefWrapper_c"])?;
    }
    Ok(())
}

fn setting_values(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.u8("byte_8cc")?;
    reader.u8("byte_8d0")?;
    reader.u16("word_8d4")?;
    reader.i32("word_938")?;
    reader.u8("byte_92c")?;
    reader.i32("word_910")?;
    reader.f64s(&["value_918", "value_920"])?;
    for offset in ["934", "93c", "940"] {
        reader.u8(&format!("byte_{offset}"))?;
    }
    Ok(())
}

fn tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("word_b90")?;
    edge_data(reader, map)?;
    model_lists::unsigned_words(reader, "array_c18")?;
    declared(reader, map, "dimension_830", DIMENSIONS)?;
    declared(reader, map, "dimension_838", DIMENSIONS)?;
    reader.u8("byte_944")?;
    reader.i32s(&["word_8e4", "word_8e8"])?;
    if reader.version() > 0x2b8e {
        reader.u8("byte_c40")?;
        declared(reader, map, "list_c38", &["suObList"])?;
    }
    Ok(())
}

fn edge_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("edge_data.count", 18)? {
        reader.i32(&format!("edge_data[{index}][0]"))?;
        reader.i32(&format!("edge_data[{index}][1]"))?;
        reader.f64(&format!("edge_data[{index}][2]"))?;
        declared(
            reader,
            map,
            &format!("edge_data[{index}][3]"),
            &["moCompEdge_c"],
        )?;
    }
    Ok(())
}

pub(super) fn variable(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("keeper_868", |reader| keeper(reader, map))?;
    variable_settings(reader, map)
}

fn variable_settings(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["870", "878"] {
        declared(reader, map, &format!("list_{offset}"), &["suObList"])?;
    }
    fields(reader, Octet, "81c 80c")?;
    fields(reader, Signed, "810")?;
    for offset in ["880", "828"] {
        declared(reader, map, &format!("list_{offset}"), &["suObList"])?;
    }
    fields(reader, Signed, "830")?;
    model_lists::unsigned_words(reader, "array_838")?;
    fields(reader, Octet, "808")?;
    fields(reader, Signed, "814")?;
    declared(reader, map, "list_888", &["suObList"])?;
    fields(reader, Octet, "818")?;
    if reader.version() > 17000 {
        fields(reader, Octet, "820")?;
    }
    Ok(())
}

pub(super) fn edge_radius(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "dimension", DIMENSIONS)?;
    declared(reader, map, "edge", &["moCompEdge_c"])?;
    reader.i32("word_20")?;
    Ok(())
}

pub(super) fn face_radius(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "dimension", DIMENSIONS)?;
    declared(reader, map, "face", &["moCompFace_c"])
}

pub(super) fn loop_radius(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "dimension", DIMENSIONS)?;
    declared(reader, map, "loop", &["moCompLoop_c"])?;
    declared(reader, map, "edge", &["moCompEdge_c"])
}

pub(super) fn edge_dimension(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "edge_10", &["moCompEdge_c"])?;
    reader.i32s(&["raw_08", "raw_0c"])
}

pub(super) fn vertex_dimension(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "dimension_08", DIMENSIONS)?;
    declared(reader, map, "vertex_10", &["moCompVertex_c"])?;
    reader.u8("raw_28")?;
    declared(reader, map, "dimension_18", DIMENSIONS)?;
    declared(reader, map, "dimension_20", DIMENSIONS)
}
