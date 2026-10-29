use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, line_style};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.object("manager", map, ModelObjectRole::Declared(&["moLayerMgr_c"]))
}

pub(super) fn layer(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.text("name")?;
    reader.text("description")?;
    reader.u32("color")?;
    reader.u8("line_style")?;
    reader.scope("line_width", line_style::short_width)?;
    reader.i32("id")?;
    Ok(())
}

pub(super) fn mask(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    let count = reader.i32("count")?;
    reader.i32("growth")?;
    let count = reader.signed_count_value("layer mask", count, 4)?;
    for index in 0..count {
        reader.u32(&format!("values[{index}]"))?;
    }
    Ok(())
}

pub(super) fn manager(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for index in 0..reader.signed_count("layer_count", 2)? {
        reader.object(
            &format!("layers[{index}]"),
            map,
            ModelObjectRole::Declared(&["moLayer_c"]),
        )?;
    }
    reader.object(
        "current_layer",
        map,
        ModelObjectRole::Declared(&["moLayer_c"]),
    )?;
    reader.object("owner", map, ModelObjectRole::Any)?;
    reader.object("mask", map, ModelObjectRole::Declared(&["moLayerMask_c"]))?;
    reader.i32("raw_38")?;
    Ok(())
}
