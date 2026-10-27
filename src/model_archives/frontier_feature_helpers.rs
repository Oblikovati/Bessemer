use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn pattern_item(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_20", "raw_24"])?;
    reader.u8("raw_08")?;
    reader.f64("raw_10")
}

pub(super) fn edge_parameters(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // #54 / 11.350 / C-392 proves the identical inherited edge-radius wire layer.
    reader.scope("edge_radius", |reader| {
        super::fillets::edge_radius(reader, map)
    })?;
    for offset in ["28", "30"] {
        declared(reader, map, &format!("dimension_{offset}"), DIMENSIONS)?;
    }
    Ok(())
}

pub(super) fn feature_radius(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "dimension_08", DIMENSIONS)?;
    // #54 / C-392: native CObject* has no radius-specific pointer restriction.
    reader.object("object_10", map, ModelObjectRole::Any)
}
