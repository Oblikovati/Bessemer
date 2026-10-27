use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
        "moHelix_c" => super::helix_records::read(reader, map),
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
