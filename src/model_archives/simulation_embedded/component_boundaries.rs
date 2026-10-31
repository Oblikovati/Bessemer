use super::{bounds, components, wrappers};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::model_archives::{ModelDecoder, ModelObjectRole};

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    Some(match name {
        "moCompFace_c" | "moCompVertex_c" => boundary(reader, map),
        "moCompEdge_c" => edge(reader, map),
        "moCompSolidBody_c" => solid(reader, map),
        "moCompFeature_c" => feature(reader, map),
        "moCompSketchEntHandle_c" => sketch(reader, map),
        "sgLineHandle" => line_handle(reader),
        _ => return None,
    })
}

fn boundary(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    components::reference_base(reader, map)?;
    if reader.u8("has_alias")? != 0 {
        reader.text("alias")?;
    }
    reader.object("boundary_reference", map, ModelObjectRole::Any)?;
    reader.object("external_reference", map, ModelObjectRole::Any)
}

fn edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    boundary(reader, map)?;
    let flags = reader.u32("edge_flags")?;
    let short_return = flags & 6 == 6;
    if short_return || reader.archive_version().value() > 0xfa2 {
        edge_records(reader, map)?;
    }
    if !short_return && reader.archive_version().value() > 0xabb && flags & 8 != 0 {
        for index in 0..2 {
            reader.object(
                &format!("composite_references[{index}]"),
                map,
                ModelObjectRole::Any,
            )?;
        }
    }
    Ok(())
}

fn edge_records(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    wrappers::doubles(reader, "point_vectors", 6)?;
    reader.u32("unknown_18c")?;
    let count = bounds::signed_count(reader, "references.count", 10000, 2)?;
    for index in 0..count {
        reader.object(&format!("references[{index}]"), map, ModelObjectRole::Any)?;
    }
    Ok(())
}

fn require_modern(reader: &ModelDecoder<'_>, name: &str) -> Result<()> {
    if reader.archive_version().value() >= 4307 {
        return Ok(());
    }
    Err(super::unsupported(
        name,
        format!(
            "class {name:?} version {}; expected embedded version >= 4307",
            reader.archive_version().value(),
        ),
    ))
}

pub(super) fn solid(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    require_modern(reader, "moCompSolidBody_c")?;
    components::reference_base(reader, map)?;
    solid_tail(reader, map)
}

pub(super) fn solid_tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object("face_reference", map, ModelObjectRole::Any)?;
    let count = bounds::signed_count(reader, "words.count", 100000, 4)?;
    for index in 0..count {
        reader.i32(&format!("words[{index}]"))?;
    }
    reader.i32s(&["unknown_108", "unknown_104", "unknown_100"])
}

fn feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    components::reference_base(reader, map)?;
    reader.i32("feature_id")?;
    reader.u32("feature_time")?;
    Ok(())
}

fn sketch(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    require_modern(reader, "moCompSketchEntHandle_c")?;
    feature(reader, map)?;
    reader.object("handle", map, ModelObjectRole::Any)?;
    reader.u32("unknown_100")?;
    Ok(())
}

fn line_handle(reader: &mut ModelDecoder<'_>) -> Result<()> {
    require_modern(reader, "sgLineHandle")?;
    reader.escaped_index("entity_index", 0x777f)?;
    reader.i32("reference_id")?;
    if reader.archive_version().value() >= 0x17d7 {
        reader.i32("dimension_on_cm")?;
    }
    Ok(())
}
