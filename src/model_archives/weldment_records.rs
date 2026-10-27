use super::{ModelDecoder, body_features, custom_properties};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| {
        body_features::model_feature(reader, map)
    })?;
    reader.scope("payload", |reader| {
        reader.scope("properties_360", |reader| {
            custom_properties::list(reader, "properties")
        })?;
        reader.i32("raw_370")?;
        Ok(())
    })
}
