use super::display_records::declared;
use super::geometric_references::LINE_REFERENCE_CLASSES;
use super::{ModelDecoder, ModelObjectRole, reference_planes};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) const AXIS_DATA: &[&str] = &[
    "moRefAxisData_c",
    "moSurfaceAxisData_c",
    "moFacePtAxisData_c",
    "moPlanePtAxisData_c",
    "moPlaneInterAxisData_c",
    "moLineAxisData_c",
    "moConFaceAxisData_c",
    "moTwoPtsAxisData_c",
];

pub(super) fn axis(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference_planes::reference_geometry(reader, map)?;
    reader.required_object("axis_data", map, ModelObjectRole::Declared(AXIS_DATA))
}

pub(super) fn data(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &str,
) -> Result<()> {
    if !AXIS_DATA.contains(&class) {
        return Err(super::invalid(format!(
            "axis data class {class:?}; expected {AXIS_DATA:?}"
        )));
    }
    reader.require_ordinary_part()?;
    for label in ["point_18", "point_30"] {
        double_vector(reader, label)?;
    }
    reader.f64("raw_08")?;
    reader.f64("raw_10")?;
    double_vector(reader, "vector_50")?;
    double_vector(reader, "point_68")?;
    suffix(reader, map, class)
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, class: &str) -> Result<()> {
    let references: &[(&str, &[&str])] = match class {
        "moSurfaceAxisData_c" => &[("80", SURFACES)],
        "moFacePtAxisData_c" => &[("80", SURFACES), ("88", &["moPointRef_w"])],
        "moPlanePtAxisData_c" => &[("80", &["moPlaneRef_w"]), ("88", &["moPointRef_w"])],
        "moPlaneInterAxisData_c" => &[("80", &["moPlaneRef_w"]), ("88", &["moPlaneRef_w"])],
        "moTwoPtsAxisData_c" => &[("80", &["moPointRef_w"]), ("88", &["moPointRef_w"])],
        // #46: the native temporary-axis serializer delegates to the line body.
        "moLineAxisData_c" => &[("80", LINE_REFERENCE_CLASSES)],
        "moConFaceAxisData_c" => &[("80", &["moCompFace_c"])],
        _ => &[],
    };
    for (offset, classes) in references {
        declared(reader, map, &format!("reference_{offset}"), classes)?;
    }
    Ok(())
}

const SURFACES: &[&str] = &[
    "moSurfaceRef_w",
    "moGeneralSurfRef_w",
    "moSingleFaceRef_w",
    "moPlaneRef_w",
    "moCylinderRef_w",
    // #54 / C-383 proves the complete surface and reference-wrapper ancestry.
    "moConeRef_w",
    "moSphereRef_w",
];
