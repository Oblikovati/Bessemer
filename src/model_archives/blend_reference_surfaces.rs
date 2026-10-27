use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })?;
    // #54 / C-382 proves +AB0 is zero: the inherited surface stays inline.
    reader.scope("surface", |reader| {
        super::planar_surface_records::surface(reader, map)
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("data", |reader| {
        super::blend_records::shared_contents(reader, map)
    })?;
    for label in ["raw_c10", "raw_c14", "raw_c18", "raw_c1c", "raw_c20"] {
        reader.u8(label)?;
    }
    Ok(())
}
