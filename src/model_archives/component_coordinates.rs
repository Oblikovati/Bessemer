use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::component_references::reference_geometry(reader, map)?;
    if reader.version() > 0x3b27 {
        super::fixed_transform_records::read(
            reader,
            "transform_100",
            "component coordinate-system",
        )?;
    }
    Ok(())
}
