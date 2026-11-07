use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::{
    ModelDecoder, ModelObjectRole, body_features, deform_geometry, deform_matching, draft_keepers,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve the complete body feature, both matching passes and direct keeper.
/// Example: `deform_features::read(reader, map)?` for `moDeform_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn arrays(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, labels: &[&str]) -> Result<()> {
    for label in labels {
        reader.scope(label, |reader| body_features::nullable_array(reader, map))?;
    }
    Ok(())
}

const INITIAL_ARRAYS: &[&str] = &[
    "array_828",
    "array_temporary_1",
    "array_temporary_2",
    "array_830",
    "array_840",
    "array_858",
];
const LATER_ARRAYS: &[&str] = &[
    "array_880",
    "array_888",
    "array_890",
    "array_898",
    "array_8a0",
];

fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "dimension_800", DIMENSIONS)?;
    declared(reader, map, "dimension_808", DIMENSIONS)?;
    reader.i32("raw_810")?;
    declared(reader, map, "direction_818", &["moDirectionSpec_c"])?;
    arrays(reader, map, INITIAL_ARRAYS)?;
    reader.i32("raw_860")?;
    arrays(reader, map, LATER_ARRAYS)?;
    double_vector(reader, "point_8a8")
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    prefix(reader, map)?;
    let subcounts = deform_matching::first(reader, map)?;
    let version = reader.i32("stored_version_8d0")?;
    versioned(reader, map, version)?;
    reader.f64s(&["raw_8e0", "raw_8e8", "raw_8f0"])?;
    reader.i32("raw_864")?;
    double_vector(reader, "vector_868")?;
    reader.scope("geometry_8f8", deform_geometry::read)?;
    if version >= 8 {
        arrays(reader, map, &["array_850"])?;
    }
    deform_matching::second(reader, map, version, &subcounts)?;
    if version >= 7 {
        extensions(reader, map)?;
    }
    reader.scope("keeper_10f0", |reader| draft_keepers::read(reader, map))
}

fn versioned(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, version: i32) -> Result<()> {
    if version >= 1 {
        declared(reader, map, "direction_820", &["moDirectionSpec_c"])?;
    }
    if version >= 2 {
        arrays(reader, map, &["array_838"])?;
    }
    if version >= 3 {
        arrays(reader, map, &["array_848"])?;
    }
    if version >= 4 {
        reader.i32("raw_8d4")?;
    }
    if version >= 6 {
        reader.i32("raw_8d8")?;
    }
    Ok(())
}

fn extensions(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0_usize.. {
        reader.entries(1, "deform extension records")?;
        let more = reader.scope(&format!("extensions[{index}]"), |reader| {
            let code = reader.i32("code")?;
            extension_payload(reader, map, code)
        })?;
        if !more {
            return Ok(());
        }
    }
    Ok(())
}

fn extension_payload(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    code: i32,
) -> Result<bool> {
    match code {
        0x3d2 => reader.i32("value").map(|_| true),
        0x13e => {
            reader.f64("value")?;
            Ok(true)
        }
        0x1466 => {
            reader.object("object", map, ModelObjectRole::Any)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}
