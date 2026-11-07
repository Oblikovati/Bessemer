use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, reference_planes, reference_point_data};
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    if name == "moRefPoint_c" {
        return Some(read(reader, map));
    }
    let kind = reference_point_data::PointDataKind::parse(name)?;
    Some(reference_point_data::read(reader, map, kind))
}

fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference_planes::reference_geometry(reader, map)?;
    reader.required_object(
        "data_6f8",
        map,
        ModelObjectRole::Declared(reference_point_data::CLASSES),
    )?;
    declared(reader, map, "profile_6f0", super::profile_features::CLASSES)?;
    declared(
        reader,
        map,
        "surface_reference_700",
        super::geometric_references::SURFACE_CLASSES,
    )
}
