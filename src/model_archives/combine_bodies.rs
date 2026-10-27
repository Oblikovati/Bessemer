use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn combine(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    declared(reader, map, "body_800", &["moCompSolidBody_c"])?;
    model_lists::pointers(reader, map, "array_808", ModelObjectRole::Any)?;
    reader.i32("raw_828")?;
    if reader.version() > 0x3f07 {
        reader.u8("raw_838")?;
    }
    Ok(())
}

pub(super) fn delete(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    declared(reader, map, "data_800", &["moDeleteBodyData_c"])
}

pub(super) fn delete_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "array_08", ModelObjectRole::Any)?;
    reader.i32("raw_28")?;
    Ok(())
}
