use super::display_records::declared;
use super::{ModelDecoder, geometric_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn endpoint(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    geometric_references::point(reader, map)?;
    reader.f64("raw_e8")?;
    reader.i32("raw_f0")?;
    Ok(())
}

pub(super) fn synchronization(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    endpoint(reader, map)?;
    reader.i32("raw_f8")?;
    reader.u32("raw_fc")?;
    reader.u32("raw_100")?;
    reader.f64("raw_108")?;
    declared(
        reader,
        map,
        "reference_1a0",
        super::model_references::WRAPPER_CLASSES,
    )?;
    reader.i32s(&["raw_19c", "raw_1a8"])?;
    declared(
        reader,
        map,
        "point_reference_1b0",
        &["moPointRef_w", "moEndPointRef_w", "moLoftSynchRef_c"],
    )?;
    reader.f64s(&["raw_1b8", "raw_1c0", "raw_110", "raw_118"])?;
    double_vector(reader, "point_120")?;
    double_vector(reader, "point_138")?;
    curve_references(reader, map)
}

pub(super) const CURVE_REFERENCE_CLASSES: &[&str] =
    &["moCurveRef_w", "moGeneralCurveRef_w", "moCircleRef_w"];

fn curve_references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["curve_reference_1c8", "curve_reference_1d0"] {
        declared(reader, map, label, CURVE_REFERENCE_CLASSES)?;
    }
    Ok(())
}
