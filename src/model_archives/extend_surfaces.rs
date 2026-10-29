use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the complete inline reference surface before its tagged extension data.
/// Example: `extend_surfaces::read(reader, map)?` for `moExtendRefSurface_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })?;
    reader.scope("surface", |reader| {
        super::planar_surface_records::surface(reader, map)
    })?;
    declared(
        reader,
        map,
        "extend_data_7b0",
        &["moExtendRefSurfaceData_c"],
    )
}

/// Retain direct lists and select only the documented end-condition pointer.
/// Example: `extend_surfaces::data(reader, map)?` for the extension data object.
pub(super) fn data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for offset in ["30", "c0", "78"] {
        model_lists::pointers(reader, map, &format!("list_{offset}"), ModelObjectRole::Any)?;
    }
    reader.i32("raw_type_08")?;
    match reader.i32("end_condition_0c")? {
        0 => declared(reader, map, "distance_28", &["moDisplayDistanceDim_c"]),
        1 => declared(
            reader,
            map,
            "reference_20",
            super::model_references::WRAPPER_CLASSES,
        ),
        2 => declared(
            reader,
            map,
            "reference_18",
            super::model_references::WRAPPER_CLASSES,
        ),
        _ => Ok(()),
    }?;
    reader.i32("raw_method_10")?;
    Ok(())
}
