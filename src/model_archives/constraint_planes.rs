use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, reference_planes};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

const GEOMETRIES: &[&str] = &[
    "moRefPlaneGeometry_c",
    "moRefPlaneCoincidentGeom_c",
    "moRefPlaneParallelGeom_c",
    "moRefPlaneMidPlaneGeom_c",
    "moRefPlanePerpendicularGeom_c",
    "moRefPlaneTangentGeom_c",
    "moRefPlaneAngleGeom_c",
];

const CLASS_READERS: &[(&str, super::sketch_geometry::DirectReader)] = &[
    ("moPntSurfRefPlnData_c", point_surface),
    ("moConstraintRefplaneData_c", constraint),
    ("moConstraintMidPlaneRefplaneData_c", constraint),
    (
        "moConstraintCoincLineParallelPlaneRefplaneData_c",
        constraint,
    ),
    (
        "moConstraintCoincTwoPointsPerpPlnRefplaneData_c",
        constraint,
    ),
    (
        "moConstraintPrllPlnTanOneCylinderRefplaneData_c",
        constraint,
    ),
    ("moConstraintCoincTwoLinesRefplaneData_c", word_constraint),
    (
        "moConstraintCoincLinePerpPlaneRefplaneData_c",
        word_constraint,
    ),
    (
        "moConstraintCoincLineAtAnglePlaneRefplaneData_c",
        angle_constraint,
    ),
    ("moConstraintRefplaneInterfaceData_c", interface),
    ("moRefPlaneGeometry_c", geometry),
    ("moRefPlaneCoincidentGeom_c", geometry),
    ("moRefPlaneParallelGeom_c", geometry),
    ("moRefPlaneMidPlaneGeom_c", flagged_geometry),
    ("moRefPlanePerpendicularGeom_c", flagged_geometry),
    ("moRefPlaneTangentGeom_c", flagged_geometry),
    ("moRefPlaneAngleGeom_c", angle_geometry),
];

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    CLASS_READERS
        .iter()
        .find(|(class, _)| *class == name)
        .map(|(_, read)| read(reader, map))
}

fn point_surface(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference_planes::base(reader, map, false)?;
    reader.i32s(&["raw_9c", "raw_98"])?;
    reader.required_object(
        "surface_88",
        map,
        ModelObjectRole::Declared(super::geometric_references::SURFACE_CLASSES),
    )?;
    reader.required_object(
        "point_90",
        map,
        ModelObjectRole::Declared(super::geometric_references::POINT_REFERENCE_CLASSES),
    )?;
    reader.i32s(&["raw_a0", "raw_a4", "raw_a8"])
}

fn constraint(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference_planes::base(reader, map, false)?;
    declared(
        reader,
        map,
        "interface_88",
        &["moConstraintRefplaneInterfaceData_c"],
    )
}

fn word_constraint(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    constraint(reader, map)?;
    reader.i32("raw_e8")?;
    Ok(())
}

fn angle_constraint(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    constraint(reader, map)?;
    reader.i32("raw_148")?;
    declared(
        reader,
        map,
        "angular_dimension_150",
        &["moDisplayAngularDim_c"],
    )
}

fn interface(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["geometry_08", "geometry_10", "geometry_18"] {
        declared(reader, map, label, GEOMETRIES)?;
    }
    Ok(())
}

fn geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(
        reader,
        map,
        "reference_08",
        super::model_references::WRAPPER_CLASSES,
    )?;
    reader.f64s(&["surface_u_10", "surface_v_18"])?;
    double_vector(reader, "point_20")
}

fn flagged_geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    geometry(reader, map)?;
    // C-280 / C-378: native loading uses equality with one, never nonzero normalization.
    reader.u8("raw_plane_geometry_flag_38")?;
    Ok(())
}

fn angle_geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    geometry(reader, map)?;
    reader.f64("raw_38")?;
    reader.u8("raw_plane_geometry_flag_40")?;
    Ok(())
}
