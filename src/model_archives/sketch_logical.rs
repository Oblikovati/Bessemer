use super::{ModelDecoder, ModelObjectRole, sketch_arrays, sketch_collections, sketch_dimensions};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sketch_dimensions::base(reader, map)?;
    reader.u16("reference_index")?;
    reader.u16("derived")?;
    Ok(())
}

pub(super) fn one(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    sketch_dimensions::handles(reader, map, &["entity0"])
}

pub(super) fn two(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    sketch_dimensions::handles(reader, map, &["entities[0]", "entities[1]"])?;
    reader.u16("tangent_condition_f8")?;
    reader.u16("tangent_condition_fc")?;
    reader.object(
        "help_data",
        map,
        // #46: both concrete names store the same documented shared object-list grammar.
        ModelObjectRole::Declared(&["su_CObArray", "suObArray"]),
    )?;
    two_tail(reader, map)
}

fn two_tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.u16("word_100")?;
    reader.i32s(&["unknown_110", "unknown_114"])?;
    reader.f64s(&["unknown_118", "unknown_120"])?;
    external_handles(reader, map)?;
    reader.u16("word_150")?;
    if reader.version() > 0x46cb {
        reader.i32("logical_group_dimension_index")?;
    }
    Ok(())
}

pub(super) fn three(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    sketch_dimensions::handles(reader, map, &["entities[0]", "entities[1]", "entities[2]"])?;
    external_handles(reader, map)
}

fn external_handles(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    const LABELS: &[&str] = &[
        "external_handles[0]",
        "external_handles[1]",
        "external_handles[2]",
    ];
    sketch_dimensions::handles(reader, map, LABELS)
}

pub(super) fn isoparam(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    reader.i32s(&["keep_last_index", "last_segment_count", "id", "flags"])?;
    reader.scope("segment_states", segment_states)
}

pub(super) fn segment_states(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let width = reader.u32("element_width")?;
    if width == 0 {
        return Ok(());
    }
    if width != 2 {
        return reader.checked(Err(crate::BessemerError::new(
            "model archive",
            "isoparametric states",
            crate::FailureClass::Unsupported,
            format!("state element width {width}; expected 0 or 2"),
        )));
    }
    reader.u32("metadata_4")?;
    reader.u32("metadata_8")?;
    let count = reader.unsigned_count("count", 2)?;
    sketch_arrays::words(reader, "values", count)
}

pub(super) fn collections(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let groups: &[(&str, &str, super::sketch_geometry::DirectReader)] = &[
        ("logical_g1_count", "logical_g1", one),
        ("logical_g2_count", "logical_g2", two),
        ("logical_g3_count", "logical_g3", three),
    ];
    for (count, label, body) in groups {
        sketch_collections::direct(reader, map, (count, label), *body)?;
    }
    Ok(())
}

pub(super) fn isoparam_collection(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    sketch_collections::direct(reader, map, ("isoparam_count", "isoparams"), isoparam)
}
