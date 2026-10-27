use super::{ModelDecoder, ModelObjectRole, kernel_frames, model_lists, model_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) const LINE_REFERENCE_CLASSES: &[&str] = &["moLineRef_w", "moTempAxisRef_w"];

// #46 / C-192 and C-232: both derived serializers retain the complete point-reference base.
pub(super) const POINT_REFERENCE_CLASSES: &[&str] =
    &["moPointRef_w", "moEndPointRef_w", "moLoftSynchRef_c"];

pub(super) const SURFACE_CLASSES: &[&str] = &[
    "moSurfaceRef_w",
    "moGeneralSurfRef_w",
    "moSingleFaceRef_w",
    "moCylinderRef_w",
    "moSphereRef_w",
    "moPlaneRef_w",
];

pub(super) fn surface(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_references::wrapper(reader, map)?;
    reader.f64s(&["unknown_a8", "unknown_b0", "unknown_b8", "unknown_c0"])
}

pub(super) fn general_curve(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_references::wrapper(reader, map)?;
    reader.f64s(&["unknown_a8", "unknown_b0"])?;
    if reader.u8("has_embedded_body")? != 0 {
        reader.scope("embedded_body", kernel_frames::read_body)?;
    }
    reader.u32("unknown_c8")?;
    reader.i32("unknown_cc")?;
    for offset in ["d8", "d0", "e0"] {
        reader.object(&format!("unknown_{offset}"), map, ModelObjectRole::Any)?;
    }
    Ok(())
}

pub(super) fn single_face(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    surface(reader, map)?;
    if reader.i32("has_embedded_body")? != 0 {
        reader.scope("embedded_body", kernel_frames::read_body)?;
        model_lists::pointers(reader, map, "body_attributes", ModelObjectRole::Any)?;
    }
    reader.u32("unknown_c8")?;
    reader.i32("unknown_d8")?;
    Ok(())
}

fn optional_frame(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let mode = reader.u16("frame_mode")?;
    if mode == 0 {
        return Ok(());
    }
    double_vector(reader, "frame_vector")?;
    if mode > 1 && mode < 0x8000 {
        double_vector(reader, "frame_point")?;
    }
    Ok(())
}

pub(super) fn circle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    general_curve(reader, map)?;
    double_vector(reader, "unknown_f8")?;
    double_vector(reader, "unknown_110")?;
    reader.f64("unknown_128")?;
    reader.i32("unknown_130")?;
    optional_frame(reader)
}

pub(super) fn cylinder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    surface(reader, map)?;
    double_vector(reader, "unknown_c8")?;
    double_vector(reader, "unknown_e0")?;
    reader.f64("unknown_f8")?;
    optional_frame(reader)
}

pub(super) fn sphere(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    surface(reader, map)?;
    double_vector(reader, "unknown_c8")?;
    reader.f64("unknown_e0")?;
    Ok(())
}

pub(super) fn point(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_references::wrapper(reader, map)?;
    double_vector(reader, "point_a8")?;
    Ok(())
}

pub(super) fn line(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_references::wrapper(reader, map)?;
    reader.f64s(&["parameter_a8", "parameter_b0"])?;
    double_vector(reader, "point_c0")?;
    double_vector(reader, "vector_d8")?;
    reader.u8("byte_f0")?;
    Ok(())
}

pub(super) fn loop_body(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_references::wrapper(reader, map)?;
    if reader.i32("body_presence")? != 0 {
        reader.scope("body_a8", kernel_frames::read_body)?;
    }
    Ok(())
}

pub(super) fn solid(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_references::wrapper(reader, map)?;
    reader.u32("raw_a8")?;
    if reader.i32("body_presence")? != 0 {
        reader.scope("body_b8", kernel_frames::read_body)?;
        // #54 / C-248: the direct list follows even a suppressed selected body.
        model_lists::pointers(reader, map, "body_attributes", ModelObjectRole::Any)?;
    }
    Ok(())
}
