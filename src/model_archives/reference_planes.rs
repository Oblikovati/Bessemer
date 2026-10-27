use super::display_records::declared;
use super::geometric_references::{LINE_REFERENCE_CLASSES, POINT_REFERENCE_CLASSES};
use super::{ModelDecoder, ModelObjectRole, body_features, model_references, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn plane(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference_geometry(reader, map)?;
    declared(reader, map, "plane_data", CLASSES)
}

pub(super) fn reference_geometry(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    body_features::header(reader, map)?;
    body_features::atom_fields(reader, map)?;
    double_vector(reader, "root_point")?;
    reader.u32("flags_6d4")?;
    reader.i32("word_6d8")?;
    declared(reader, map, "feature_reference", &["moFeatureRefWrapper_c"])?;
    if reader.version() > 0x2f85 {
        reader.i32("word_6e8")?;
    }
    Ok(())
}

pub(super) const CLASSES: &[&str] = &[
    "moRefPlnData_c",
    "moFixedRefPlnData_c",
    "moDefaultRefPlnData_c",
    "moFaceRefPlnData_c",
    "moFacePtRefPlnData_c",
    "moLinePtRefPlnData_c",
    "moPerLinePtRefPlnData_c",
    "moRefAtAngleData_c",
    "moRefThreePtsData_c",
    // #46 / C-280, C-374 and C-378: complete concrete ordinary plane-data bodies.
    "moPntSurfRefPlnData_c",
    "moConstraintRefplaneData_c",
    "moConstraintMidPlaneRefplaneData_c",
    "moConstraintCoincLineAtAnglePlaneRefplaneData_c",
    "moConstraintCoincLineParallelPlaneRefplaneData_c",
    "moConstraintCoincTwoPointsPerpPlnRefplaneData_c",
    "moConstraintPrllPlnTanOneCylinderRefplaneData_c",
    "moConstraintCoincTwoLinesRefplaneData_c",
    "moConstraintCoincLinePerpPlaneRefplaneData_c",
];

pub(super) fn data(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: PlaneDataKind,
) -> Result<()> {
    base(reader, map, kind == PlaneDataKind::Default)?;
    match kind {
        PlaneDataKind::Base | PlaneDataKind::Default => Ok(()),
        PlaneDataKind::Face => face(reader, map),
        PlaneDataKind::FacePoint => face_point(reader, map),
        PlaneDataKind::LinePoint => line_point(reader, map),
        PlaneDataKind::Perpendicular => perpendicular(reader, map),
        PlaneDataKind::Angle => angle(reader, map),
        PlaneDataKind::ThreePoints => three_points(reader, map),
    }
}

pub(super) fn base(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    default: bool,
) -> Result<()> {
    reader.require_ordinary_part()?;
    double_vector(reader, "root_point")?;
    double_vector(reader, "normal")?;
    reader.transform("transform")?;
    sketch_arrays::doubles(reader, "border", 4)?;
    declared(reader, map, "edge", &["moCompEdge_c"])?;
    reader.i32s(&["word_50", "word_54", "word_58"])?;
    reader.u8("discarded_byte")?;
    if default {
        double_vector(reader, "display_root_point")?;
    }
    reader.i32s(&["word_5c", "word_60"])?;
    reader.u16("flag_64")?;
    base_references(reader, map)
}

fn base_references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x2b83 {
        declared(reader, map, "facet_fin", &["moCompFaceMeshFacetFin_c"])?;
    }
    if reader.version() > 0x2ba5 {
        declared(reader, map, "axis", &["moRefAxis_c"])?;
        declared(reader, map, "sketch_entity", &["moCompSketchEntHandle_c"])?;
    }
    Ok(())
}

pub(super) fn plane_reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::geometric_references::surface(reader, map)?;
    double_vector(reader, "root_point")?;
    double_vector(reader, "normal")?;
    reader.transform("transform")?;
    double_vector(reader, "display_root_point")
}

fn plane_data_reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_references::wrapper(reader, map)?;
    reader.transform("transform_a8")?;
    reader.transform("transform_130")?;
    for label in ["point_1b8", "vector_1d0", "vector_1e8"] {
        double_vector(reader, label)?;
    }
    Ok(())
}

fn face(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("plane_reference", |reader| plane_reference(reader, map))?;
    declared(reader, map, "dimension", super::display_dimension::CLASSES)?;
    reader.i32s(&["word_a0", "word_a4"])?;
    if reader.i32("reference_presence")? != 0 {
        reader.scope("plane_data_reference", |reader| {
            plane_data_reference(reader, map)
        })?;
    }
    reader.i32("word_b4")?;
    Ok(())
}

fn face_point(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "point_reference_88", POINT_REFERENCE_CLASSES)?;
    reader.scope("direct_plane_reference_90", |reader| {
        plane_reference(reader, map)
    })?;
    reader.i32s(&["raw_98", "raw_9c"])
}

fn line_point(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "point_reference_88", POINT_REFERENCE_CLASSES)?;
    reader.required_object(
        "curve_reference_90",
        map,
        ModelObjectRole::Declared(&[
            "moCurveRef_w",
            "moGeneralCurveRef_w",
            "moLineRef_w",
            "moTempAxisRef_w",
            "moCircleRef_w",
        ]),
    )?;
    reader.i32s(&["raw_98", "raw_9c"])
}

fn perpendicular(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    line_point(reader, map)?;
    reader.u16("raw_a0")?;
    reader.i32("raw_a4")?;
    Ok(())
}

fn angle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("direct_plane_reference_90", |reader| {
        plane_reference(reader, map)
    })?;
    declared(reader, map, "line_reference_98", LINE_REFERENCE_CLASSES)?;
    declared(
        reader,
        map,
        "angular_dimension_a0",
        &["moDisplayAngularDim_c"],
    )?;
    reader.i32s(&["raw_a8", "raw_ac", "raw_b0", "raw_b4"])
}

fn three_points(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["88", "90", "98"] {
        declared(
            reader,
            map,
            &format!("point_reference_{offset}"),
            &["moPointRef_w"],
        )?;
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum PlaneDataKind {
    Base,
    Default,
    Face,
    FacePoint,
    LinePoint,
    Perpendicular,
    Angle,
    ThreePoints,
}

impl PlaneDataKind {
    pub(super) fn parse(name: &str) -> Option<Self> {
        match name {
            "moRefPlnData_c" | "moFixedRefPlnData_c" => Some(Self::Base),
            "moDefaultRefPlnData_c" => Some(Self::Default),
            "moFaceRefPlnData_c" => Some(Self::Face),
            "moFacePtRefPlnData_c" => Some(Self::FacePoint),
            "moLinePtRefPlnData_c" => Some(Self::LinePoint),
            "moPerLinePtRefPlnData_c" => Some(Self::Perpendicular),
            "moRefAtAngleData_c" => Some(Self::Angle),
            "moRefThreePtsData_c" => Some(Self::ThreePoints),
            _ => None,
        }
    }
}
