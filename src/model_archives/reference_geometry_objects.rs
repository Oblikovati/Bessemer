use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
        "moOffsetRefSurface_c" => super::offset_surfaces::read(reader, map),
        "moOffsetSurfData_c" => super::offset_surfaces::data(reader, map),
        // #54 / C-251 binds the ordinary binary su_CObject zero-byte return.
        "moOffsetRefSurfIdKeeper_c" => super::extruded_surfaces::keeper(reader),
        "moThicken_c" => super::thicken_records::read(reader, map),
        "moCompRefSurface_c" => super::thicken_records::component_surface(reader, map),
        // #54 / C-249 binds the identical modern atom-geometry wire helper.
        "moConstSurfRef_w" => super::geometric_references::solid(reader, map),
        "moSolidRef_w" => super::geometric_references::solid(reader, map),
        "moHelix_c" => super::helix_records::read(reader, map),
        // #54 / C-213 binds the existing complete component-feature helper, with no suffix.
        "moCompReferenceCurve_c" => super::component_references::feature(reader, map),
        "moRevolvRefSurf_c" => super::revolved_surfaces::read(reader, map),
        "moRevolvSurfData_c" => super::revolved_surfaces::data(reader, map),
        // C-203 binds this exact zero-byte ordinary su_CObject serializer.
        "moRevolvRefSurfIdKeeper_c" => super::extruded_surfaces::keeper(reader),
        "moCompCoordSys_c" => super::component_coordinates::read(reader, map),
        "moCompRefAxis_c" | "moCompRefPoint_c" => {
            super::component_references::reference_geometry(reader, map)
        }
        "moRefPlane_c" => super::reference_planes::plane(reader, map),
        "moPlaneRef_w" => super::reference_planes::plane_reference(reader, map),
        "moRefAxis_c" => super::reference_axes::axis(reader, map),
        "moRefAxisData_c"
        | "moSurfaceAxisData_c"
        | "moFacePtAxisData_c"
        | "moPlanePtAxisData_c"
        | "moPlaneInterAxisData_c"
        | "moLineAxisData_c"
        | "moConFaceAxisData_c"
        | "moTwoPtsAxisData_c" => super::reference_axes::data(reader, map, name),
        _ => return None,
    };
    Some(result)
}
