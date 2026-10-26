use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.entries(8, "embedded model arrays")?;
    for index in 0..8 {
        reader.scope(&format!("arrays[{index}]"), |reader| {
            model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
        })?;
    }
    Ok(())
}
