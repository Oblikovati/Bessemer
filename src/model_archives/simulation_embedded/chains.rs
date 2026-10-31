use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::model_archives::{ModelDecoder, ModelObjectRole, external_references};

pub(super) fn component(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::components::null_owner(reader, map)?;
    reader.object("chain", map, ModelObjectRole::Any)?;
    if reader.archive_version().value() >= 0x837 {
        reader.u32("unknown_30")?;
    }
    Ok(())
}

pub(super) fn reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.u16("unknown_24")?;
    reader.object("features", map, ModelObjectRole::Any)?;
    reader.i32("unknown_20")?;
    configuration(reader)
}

pub(super) fn feature_list(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let count = super::bounds::signed_count(reader, "features.count", 10000, 2)?;
    for index in 0..count {
        reader.object(&format!("features[{index}]"), map, ModelObjectRole::Any)?;
    }
    Ok(())
}

pub(super) fn feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("file", |reader| {
        external_references::stored_shared_file(reader, map)
    })?;
    configuration(reader)?;
    reader.u8("flag_40")?;
    reader.i32("unknown_44")?;
    reader.required_object(
        "name",
        map,
        ModelObjectRole::Declared(&["moCStringHandle_c"]),
    )?;
    reader.object("reference", map, ModelObjectRole::Any)?;
    if reader.archive_version().value() >= 0x11eb {
        reader.i32("unknown_58")?;
    }
    Ok(())
}

fn configuration(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.text("configuration.name")?;
    reader.i32("configuration.id")?;
    Ok(())
}
