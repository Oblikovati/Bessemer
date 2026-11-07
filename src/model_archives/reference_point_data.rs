use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) const CLASSES: &[&str] = &[
    "moRefPointData_c",
    "moFaceCenterRefPointData_c",
    "moCenterEdgeRefPointData_c",
    "moFaceEdgePointData_c",
    "moFaceVertexProjectionPointData_c",
    "moTwoEdgeRefPointData_c",
    "moAlongCurveRefPointData_c",
];
pub(super) const CURVES: &[&str] = &[
    "moCurveRef_w",
    "moGeneralCurveRef_w",
    "moCircleRef_w",
    "moLineRef_w",
    "moTempAxisRef_w",
];

#[derive(Clone, Copy)]
pub(super) enum PointDataKind {
    Base,
    FaceCenter,
    CenterEdge,
    FaceEdge,
    Projection,
    TwoEdges,
    AlongCurve,
}

impl PointDataKind {
    pub(super) fn parse(name: &str) -> Option<Self> {
        match name {
            "moRefPointData_c" => Some(Self::Base),
            "moFaceCenterRefPointData_c" => Some(Self::FaceCenter),
            "moCenterEdgeRefPointData_c" => Some(Self::CenterEdge),
            "moFaceEdgePointData_c" => Some(Self::FaceEdge),
            "moFaceVertexProjectionPointData_c" => Some(Self::Projection),
            "moTwoEdgeRefPointData_c" => Some(Self::TwoEdges),
            "moAlongCurveRefPointData_c" => Some(Self::AlongCurve),
            _ => None,
        }
    }
}

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: PointDataKind,
) -> Result<()> {
    reader.require_ordinary_part()?;
    match kind {
        PointDataKind::Base => Ok(()),
        PointDataKind::FaceCenter => {
            declared(reader, map, "face_reference_10", &["moSingleFaceRef_w"])
        }
        PointDataKind::CenterEdge => declared(reader, map, "curve_reference_10", CURVES),
        PointDataKind::FaceEdge => face_edge(reader, map),
        PointDataKind::Projection => projection(reader, map),
        PointDataKind::TwoEdges => two_edges(reader, map),
        PointDataKind::AlongCurve => along_curve(reader, map),
    }
}

fn face_edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "curve_reference_18", CURVES)?;
    reader.i32("raw_20")?;
    Ok(())
}

fn projection(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "face_reference_18", &["moSingleFaceRef_w"])?;
    declared(
        reader,
        map,
        "point_reference_20",
        super::geometric_references::POINT_REFERENCE_CLASSES,
    )?;
    declared(reader, map, "plane_reference_28", &["moPlaneRef_w"])
}

fn two_edges(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["curve_reference_10", "curve_reference_18"] {
        declared(reader, map, label, CURVES)?;
    }
    reader.i32s(&["raw_20", "raw_24"])
}

fn along_curve(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "curve_reference_10", CURVES)?;
    reader.f64s(&["raw_18", "raw_28"])?;
    reader.i32s(&["raw_20", "raw_30"])?;
    reader.object(
        "dimension_38",
        map,
        ModelObjectRole::Declared(super::display_dimension::CLASSES),
    )?;
    reader.i32s(&["raw_34", "raw_40"])
}
