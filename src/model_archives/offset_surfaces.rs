use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })?;
    // #54 / C-251: +AB0 returns zero, so the complete surface payload stays inline.
    reader.scope("surface", |reader| {
        super::planar_surface_records::surface(reader, map)
    })?;
    reader.required_object(
        "offset_data_7c8",
        map,
        ModelObjectRole::Declared(&["moOffsetSurfData_c"]),
    )?;
    Ok(())
}

pub(super) fn data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("array_08", |reader| {
        super::body_features::nullable_array(reader, map)
    })?;
    declared(reader, map, "dimension_10", &["moDisplayDistanceDim_c"])?;
    reader.i32("raw_18")?;
    Ok(())
}
