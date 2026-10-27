use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    prefix(reader, map)?;
    integer_pairs(reader)?;
    reader.i32("raw_91c")?;
    array(reader, map, "data_868.entity_keeper_78")?;
    reader.i32("data_868.raw_160")?;
    reader.i32s(&[
        "raw_924", "raw_920", "raw_928", "raw_918", "raw_914", "raw_9d8", "raw_a20",
    ])
}

fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("data_868", |reader| {
        super::surface_operations::trim_prefix(reader, map)
    })?;
    reader.scope("chooser_870", |reader| {
        body_features::direct_chooser(reader, map)
    })?;
    array(reader, map, "array_808")?;
    reader.i32("raw_910")?;
    model_lists::unsigned_words(reader, "data_868.dwords_c0")?;
    reader.i32("raw_804")?;
    array(reader, map, "array_828")
}

fn array(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    model_lists::pointers(reader, map, label, ModelObjectRole::Any)
}

fn integer_pairs(reader: &mut ModelDecoder<'_>) -> Result<()> {
    // #54 / C-276: retain signed count and raw pairs, before native filtering and replacement.
    let count = reader.i32("map_9a8_count")?;
    if count <= 0 {
        return Ok(());
    }
    for index in 0..reader.counted_entries("split integer map", i64::from(count), 8)? {
        reader.i32(&format!("raw_map_9a8[{index}].key"))?;
        reader.i32(&format!("raw_map_9a8[{index}].value"))?;
    }
    Ok(())
}
