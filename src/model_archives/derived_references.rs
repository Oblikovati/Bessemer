use super::{ModelDecoder, ModelObjectRole, model_lists, model_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) const BODY_REFERENCES: &[&str] = &["moBodyRef_w", "moSurfaceBodyRef_w"];

/// Preserve the complete derived-feature reference after its wrapper prefix.
/// Example: `derived_references::read(reader, map)?` for `moDerivedFeatRef_w`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("wrapper", |reader| model_references::wrapper(reader, map))?;
    reader.object(
        "feature_reference_a8",
        map,
        ModelObjectRole::Declared(&["moFR_c"]),
    )?;
    reader.u32("raw_b8")?;
    reference_data(reader, map)?;
    model_lists::pointers(reader, map, "array_e0", ModelObjectRole::Any)?;
    reader.i32s(&["raw_100", "raw_104"])?;
    if reader.version() > 0x2f85 && reader.i32("point_presence")? != 0 {
        double_vector(reader, "point_d8")?;
    }
    Ok(())
}

fn reference_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object(
        "plane_data_c0",
        map,
        ModelObjectRole::Declared(super::reference_planes::CLASSES),
    )?;
    reader.object(
        "axis_data_c8",
        map,
        ModelObjectRole::Declared(super::reference_axes::AXIS_DATA),
    )?;
    reader.object(
        "coordinate_data_d0",
        map,
        ModelObjectRole::Declared(&["moRefCoordSysData_c"]),
    )
}
