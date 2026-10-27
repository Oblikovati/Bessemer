use super::{ModelDecoder, ModelObjectRole, part_objects::absent_payload};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) const FEATURE_CLASSES: &[&str] =
    &["moCompFeature_c", "moCompBodyFeature_c", "moCompProfile_c"];

const GEOMETRY_CLASSES: &[&str] = &[
    "moRefGeom_c",
    "moRefPlane_c",
    "moRefAxis_c",
    "moRefPoint_c",
    // #46 / C-198: the coordinate-system serializer inherits moRefGeom_c.
    "moCoordSys_c",
    // #54 / C-251 proves this concrete surface's moRefGeom_c ancestry.
    "moOffsetRefSurface_c",
    // #54 / C-382 proves this concrete blend surface's moRefGeom_c ancestry.
    "moBlendRefSurface_c",
];

pub(super) fn reference_plane(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    feature(reader, map)?;
    reader.object(
        "reference_geometry",
        map,
        ModelObjectRole::Declared(&["moRefGeom_c", "moRefPlane_c"]),
    )?;
    reader.i32("word_100")?;
    Ok(())
}

pub(super) fn reference_geometry(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    feature(reader, map)?;
    reader.object(
        "reference_geometry",
        map,
        ModelObjectRole::Declared(GEOMETRY_CLASSES),
    )?;
    Ok(())
}

pub(super) fn feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("component_reference", |reader| base(reader, map))?;
    reader.i32("feature_id")?;
    reader.u32("feature_time")?;
    Ok(())
}

pub(super) fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("component", map, ModelObjectRole::Any)?;
    reader.i32s(BASE_PREFIX)?;
    let old = reader.i32("unknown_84")?;
    absent_payload(reader, "old component-reference payload", old)?;
    reader.i32("unknown_a0")?;
    reader.u32("unknown_34")?;
    reader.i32s(BASE_SUFFIX)?;
    if reader.i32("has_faceted_data")? != 0 {
        reader.object("faceted_data", map, ModelObjectRole::Any)?;
    }
    optional_text(reader, "has_string_b8", "unknown_b8")?;
    modern_suffix(reader)
}

const BASE_PREFIX: &[&str] = &[
    "unknown_68",
    "unknown_18",
    "unknown_1c",
    "unknown_20",
    "unknown_30",
    "unknown_70",
];
const BASE_SUFFIX: &[&str] = &[
    "unknown_48",
    "unknown_90",
    "unknown_94",
    "unknown_98",
    "unknown_9c",
];

fn optional_text(reader: &mut ModelDecoder<'_>, presence: &str, value: &str) -> Result<()> {
    if reader.i32(presence)? != 0 {
        reader.text(value)?;
    }
    Ok(())
}

fn modern_suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x2b60 {
        let present = reader.i32("has_reference_info")?;
        absent_payload(reader, "direct component-reference info", present)?;
    }
    if reader.version() > 0x2bb8 && reader.i32("has_transform_c8")? != 0 {
        reader.transform("transform_c8")?;
    }
    if reader.version() > 0x2f55 {
        optional_text(reader, "has_string_d0", "unknown_d0")?;
    }
    if reader.version() >= 0x3ef6 {
        reader.u32("unknown_4c")?;
    }
    Ok(())
}

pub(super) fn boundary(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    if reader.u8("has_alias")? != 0 {
        reader.text("alias")?;
    }
    reader.object("boundary_reference", map, ModelObjectRole::Any)?;
    reader.object(
        "external_reference",
        map,
        ModelObjectRole::Declared(&["moExtObject_c"]),
    )?;
    Ok(())
}

pub(super) fn edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    boundary(reader, map)?;
    let flags = reader.i32("unknown_188")?;
    vector(reader, "unknown_158")?;
    vector(reader, "unknown_170")?;
    reader.u32("unknown_18c")?;
    for index in 0..reader.signed_count("reference_count", 2)? {
        reader.object(&format!("references[{index}]"), map, ModelObjectRole::Any)?;
    }
    if flags & 6 != 6 && flags & 8 != 0 {
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

fn vector(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..3 {
        reader.f64(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn loop_body(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    boundary(reader, map)?;
    reader.object(
        "edge_128",
        map,
        ModelObjectRole::Declared(&["moCompEdge_c"]),
    )?;
    Ok(())
}

pub(super) fn silhouette_edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    edge(reader, map)?;
    if reader.i32("has_transform_1b0")? != 0 {
        reader.transform("transform_1b0")?;
    }
    Ok(())
}

pub(super) fn body(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    reader.object(
        "face_reference",
        map,
        ModelObjectRole::Declared(&["moFaceRef_c"]),
    )?;
    for index in 0..reader.signed_count("unknown_110.count", 4)? {
        reader.i32(&format!("unknown_110[{index}]"))?;
    }
    reader.i32s(&["unknown_108", "body_atom_id", "face_node_id"])?;
    Ok(())
}
