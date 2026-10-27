use super::ModelDecoder;
use super::display_records::declared;
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })?;
    // #54 / C-203: this class's +AB0 also returns zero, so sheets remain inline.
    reader.scope("surface", |reader| {
        super::planar_surface_records::surface(reader, map)
    })?;
    declared(reader, map, "revolve_data_7b0", &["moRevolvSurfData_c"])?;
    declared(reader, map, "axis_7b8", &["moLineRef_w"])
}

pub(super) fn data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "dimension_08", &["moDisplayAngularDim_c"])?;
    reader.i32("raw_1c")?;
    declared(reader, map, "dimension_10", &["moDisplayAngularDim_c"])?;
    reader.i32("raw_18")?;
    declared(reader, map, "end_spec_20", &["moRevEndSpec_c"])
}
