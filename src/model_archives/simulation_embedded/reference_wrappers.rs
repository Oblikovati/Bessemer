use super::wrappers::{common_wrapper, doubles};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::model_archives::{ModelDecoder, ModelObjectRole, part_objects::absent_payload};

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    Some(match name {
        "moSingleFaceRef_w" => face(reader, map),
        "moGeneralCurveRef_w" => curve(reader, map),
        "moLineRef_w" => line(reader, map),
        "moEndPointRef_w" => endpoint(reader, map),
        "moSolidRef_w" => solid(reader, map),
        "moFeatureRefWrapper_c" => feature(reader, map),
        _ => return None,
    })
}

fn face(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    common_wrapper(reader, map)?;
    doubles(reader, "surface_values", 4)?;
    let present = reader.i32("has_geometry")?;
    absent_payload(reader, "embedded face geometry", present)?;
    reader.u32("unknown_c8")?;
    reader.i32("unknown_d8")?;
    Ok(())
}

fn curve(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    common_wrapper(reader, map)?;
    doubles(reader, "parameters", 2)?;
    let present = reader.u8("has_geometry")?;
    absent_payload(reader, "embedded curve geometry", i32::from(present))?;
    reader.u32("unknown_c8")?;
    reader.i32("unknown_cc")?;
    for index in 0..3 {
        reader.object(&format!("references[{index}]"), map, ModelObjectRole::Any)?;
    }
    Ok(())
}

fn line(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    common_wrapper(reader, map)?;
    doubles(reader, "parameters", 2)?;
    doubles(reader, "point_vector", 6)?;
    reader.u8("flag_f0")?;
    Ok(())
}

fn endpoint(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    common_wrapper(reader, map)?;
    doubles(reader, "point", 3)?;
    doubles(reader, "parameters", 1)?;
    reader.i32("unknown_f0")?;
    Ok(())
}

pub(super) fn solid(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.archive_version().value() < 4307 {
        return Err(super::unsupported(
            "moSolidRef_w",
            format!(
                "version {}; expected embedded solid version >= 4307",
                reader.archive_version().value(),
            ),
        ));
    }
    common_wrapper(reader, map)?;
    if reader.archive_version().value() > 0x1bbd {
        reader.u32("unknown_a8")?;
    }
    let present = reader.i32("has_geometry")?;
    absent_payload(reader, "embedded solid geometry or attributes", present)
}

fn feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    common_wrapper(reader, map)?;
    reader.u32("unknown_a8")?;
    Ok(())
}
