use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) const CLASSES: &[&str] = &[
    "Dimension_c",
    "moNumberDim_c",
    "moScalerDim_c",
    "moScalerLengthDim_c",
    "ParallelPlaneDistanceDim_c",
    "AngleDim_c",
    "ThreeDRadiusDim_c",
    "ThreeDdiameterDim_c",
    "ThreeDRadialOrDiameterDim_c",
    "moRevolvedRadiusDim_c",
    "moRefDim_c",
    "moRefDim1E_c",
    "moRefDim2E_c",
];
pub(super) const GEOMETRIC: &[&str] = &[
    "moNumberDim_c",
    "moScalerDim_c",
    "moScalerLengthDim_c",
    "ParallelPlaneDistanceDim_c",
    "ThreeDRadiusDim_c",
    "ThreeDdiameterDim_c",
];
const TOLERANCE: ModelObjectRole<'_> = ModelObjectRole::Declared(&["moDimTolerance_c"]);
const ATTACHMENT: ModelObjectRole<'_> = ModelObjectRole::Declared(&["moAttachPoint_c"]);
const DIMENSION_WORDS: &[&str] = &[
    "am_object_id",
    "is_design_table_dimension",
    "mask",
    "unknown_5c",
];

pub(super) fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("reference_count")?;
    reader.object("parameter", map, ModelObjectRole::Any)?;
    reader.object("tolerance", map, TOLERANCE)?;
    reader.i32s(DIMENSION_WORDS)?;
    reader.object("attachment_point", map, ATTACHMENT)?;
    reader.u16("driven_dimension")?;
    if reader.version() > 0x42ee && reader.u8("has_string_78")? != 0 {
        reader.text("unknown_78")?;
    }
    Ok(())
}

pub(super) fn integer_parameter(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    super::part_objects::parameter_node(reader, map)?;
    reader.i32("value")?;
    if reader.u8("has_alternate_value")? != 0 {
        reader.i32("alternate_value")?;
    }
    Ok(())
}

pub(super) fn geometric(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    for label in ["point_80", "point_a0", "point_c0"] {
        double_vector(reader, label)?;
    }
    if reader.u16("has_vectors")? != 0 {
        double_vector(reader, "vector_e0")?;
        double_vector(reader, "vector_e8")?;
    }
    if reader.u16("has_transform")? != 0 {
        reader.transform("transform_f0")?;
    }
    flagged_points(reader)
}

fn flagged_points(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let flags = reader.u16("point_flags")?;
    for (bit, label) in [(1, "point_98"), (2, "point_b8")] {
        if flags & bit != 0 {
            double_vector(reader, label)?;
        }
    }
    Ok(())
}

pub(super) fn angle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    geometric(reader, map)?;
    reader.u16("unknown_f8")?;
    Ok(())
}

pub(super) fn revolved(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    geometric(reader, map)?;
    reader.object(
        "sketch_dimension",
        map,
        ModelObjectRole::Declared(&["moSkDimHandleValG2_c"]),
    )?;
    reader.i32("word_128")?;
    Ok(())
}

pub(super) fn tolerance(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["type", "fit_type"])?;
    reader.f64s(&["maximum", "minimum"])?;
    reader.u8("use_text_scale")?;
    reader.f64s(&["text_scale", "unknown_88", "height"])?;
    reader.text("upper_string")?;
    reader.text("lower_string")?;
    reader.u8("show_parenthesis")?;
    reader.i32("fit_display_style")?;
    reader.u8("fit_use_text_scale")?;
    reader.f64s(&["fit_text_scale", "unknown_a8", "fit_text_height"])?;
    tolerance_suffix(reader)
}

fn tolerance_suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.u8("unknown_5c")?;
    reader.u8("unknown_60")?;
    reader.i32("unknown_b8")?;
    if reader.version() > 0x2b6d {
        reader.text("upper_limit")?;
        reader.text("lower_limit")?;
        reader.text("general_limit")?;
    }
    if reader.version() > 0x2b76 {
        reader.i32("mask")?;
    }
    Ok(())
}

pub(super) fn attachment(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    double_vector(reader, "point_08")?;
    double_vector(reader, "point_28")?;
    reader.i32("word_20")?;
    reader.object(
        "drawing_view",
        map,
        ModelObjectRole::Declared(&["moDrView"]),
    )?;
    double_vector(reader, "point_58")?;
    double_vector(reader, "point_70")?;
    reader.u8("byte_88")?;
    Ok(())
}
