use super::{ModelDecoder, ModelObjectRole, features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

const PRE_TRANSFORM: &[&str] = &[
    "350", "398", "3e0", "428", "4b8", "548", "590", "5d8", "620", "668", "6f8", "740", "7d0",
    "818", "860", "8a8", "8f0", "470",
];
const POST_TRANSFORM: &[&str] = &[
    "938", "980", "9c8", "a10", "aa0", "ae8", "b30", "500", "bc0", "788", "c08", "c50", "c98",
    "ce0",
];

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("feature", |reader| features::read(reader, map))?;
    lists(reader, map, PRE_TRANSFORM)?;
    reader.transform("transform_d88")?;
    reader.object("bom", map, ModelObjectRole::Declared(&["moDisplayBom_c"]))?;
    lists(reader, map, POST_TRANSFORM)?;
    if reader.i32("bucket_presence")? != 0 {
        reader.scope("bucket", |reader| {
            crate::archive_codec::bucket_envelope::read(reader, map)
        })?;
    }
    lists(reader, map, &["b78", "d28"])?;
    reader.i32("raw_de0")?;
    Ok(())
}

fn lists(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, members: &[&str]) -> Result<()> {
    for member in members {
        model_lists::pointers(
            reader,
            map,
            &format!("lists.{member}"),
            ModelObjectRole::Any,
        )?;
    }
    Ok(())
}
