use super::raw_fields::RawScalarKind::{Double, Signed, Text, Unsigned};
use super::{ModelDecoder, ModelObjectRole, configuration_io, model_lists, raw_fields};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

const DECAL: ModelObjectRole<'_> = ModelObjectRole::Declared(&["moDecalProperties_c"]);
const FRESH_CLASSES: &[&str] = &[
    "moUnitComponent_c",
    "moCompFace_c",
    "moFaceRef_c",
    "moVertexRef_c",
    "moEndFaceSurfIdRep_c",
    "moNetSideSurfIdRep_c",
    "moFR_c",
    "moFromSktEntSurfIdRep_c",
    "moSimpleSurfIdRep_c",
    "moFromSktEnt3IntSurfIdRep_c",
];
const PREFIX: &[(raw_fields::RawScalarKind, &str)] = &[
    (Signed, "48 4c"),
    (Double, "68 70 78 80 88"),
    (Signed, "98 9c"),
    (Double, "b0 b8"),
    (Signed, "a0 a4 a8"),
    (Text, "e0 e8"),
];
const MIDDLE: &[(raw_fields::RawScalarKind, &str)] = &[
    (Signed, "f8"),
    (Double, "100 108 110 118 120 128"),
    (Unsigned, "130"),
    (Double, "138 140 150 158 168 170 178 180 188 190 198 1a0"),
    (Signed, "1a8"),
];
const SUFFIX: &[(raw_fields::RawScalarKind, &str)] = &[
    (Double, "c0 c8"),
    (Signed, "1ac 1b0 1b4 1b8 1bc 2ac"),
    (Double, "1c8"),
];
const OBJECT_SUFFIX: &[(raw_fields::RawScalarKind, &str)] =
    &[(Double, "70 78"), (Signed, "80 84 88 8c 90 94")];

/// Preserve the condition-200 decal objects and nested association arrays.
/// Example: `configuration_decals::owned_map(reader, map)?` at manager +6D8.
pub(super) fn owned_map(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..configuration_io::positive_count(reader, "owned_6d8.objects_count", 6)? {
        reader.scope(&format!("owned_6d8.objects[{index}]"), |reader| {
            reader.object("decal", map, DECAL)?;
            reader.i32("raw_value").map(|_| ())
        })?;
    }
    for index in 0..configuration_io::positive_count(reader, "owned_6d8.associations_count", 8)? {
        reader.scope(&format!("owned_6d8.associations[{index}]"), |reader| {
            association(reader, map)
        })?;
    }
    Ok(())
}

fn association(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_key")?;
    for index in 0..configuration_io::positive_count(reader, "declared_count", 2)? {
        reader.object(&format!("objects[{index}]"), map, DECAL)?;
    }
    Ok(())
}

/// Retain complete typed decal parameters without applying native conversions.
/// Example: `configuration_decals::properties(reader, map)?` for a decal object.
pub(super) fn properties(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    raw_fields::groups(reader, PREFIX)?;
    reader.scope("object_fields", object_prefix)?;
    indices(reader)?;
    raw_fields::groups(reader, MIDDLE)?;
    double_vector(reader, "vector_1d8")?;
    double_vector(reader, "vector_1f0")?;
    reader.scope("object_fields", |reader| reader.i32("raw_68").map(|_| ()))?;
    reader.scope("list_08", |reader| nested_list(reader, map))?;
    double_vector(reader, "point_208")?;
    reader.scope("list_20", |reader| nested_list(reader, map))?;
    raw_fields::groups(reader, SUFFIX)?;
    reader.scope("object_fields", |reader| {
        raw_fields::groups(reader, OBJECT_SUFFIX)
    })
}

fn object_prefix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("raw_44")?;
    reader.text("raw_48")?;
    reader.i32("raw_50").map(|_| ())
}

fn indices(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..configuration_io::positive_count(reader, "index_count", 4)? {
        reader.u32(&format!("indices[{index}]"))?;
    }
    Ok(())
}

fn nested_list(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.object_count("count")? {
        model_lists::scoped_pointer(
            reader,
            map,
            &format!("items[{index}]"),
            FRESH_CLASSES,
            "configuration decal",
        )?;
    }
    Ok(())
}
