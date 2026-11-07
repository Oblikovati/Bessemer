use super::display_records::declared;
use super::{ModelDecoder, component_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    reader.f64("raw_810")?;
    reader.i32s(&["raw_818", "raw_81c"])?;
    declared(reader, map, "list_808", &["suObList"])?;
    reader.i32("raw_820")?;
    declared(
        reader,
        map,
        "dimension_800",
        super::display_dimension::CLASSES,
    )?;
    reader.i32s(&["raw_824", "raw_828"])?;
    if reader.version() >= 0x2f74 {
        declared(reader, map, "direction_830", &["moDirectionSpec_c"])?;
    }
    Ok(())
}

pub(super) fn component_surface(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    component_references::reference_geometry(reader, map)?;
    reader.i32("raw_100")?;
    Ok(())
}
