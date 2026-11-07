use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    Some(match name {
        "moNewFeatComment_c" => comment(reader, map),
        "moCommentExtraData_c" => extra(reader),
        "moCommentTextItem_c" => text(reader),
        _ => return None,
    })
}

fn comment(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "comments_30", ModelObjectRole::Any)?;
    reader.i32("raw_28")?;
    reader.scope("direct_extra_88", extra)?;
    if reader.version() < 0x3b10 {
        return Ok(());
    }
    super::display_records::declared(
        reader,
        map,
        "feature_78",
        super::component_references::FEATURE_CLASSES,
    )?;
    super::display_records::declared(reader, map, "chain_80", &["moChainComponent_c"])
}

fn extra(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    if reader.version() >= 0x3b10 {
        reader.i32s(&["raw_0c", "raw_08", "raw_10"])?;
    }
    Ok(())
}

fn text(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_08")?;
    reader.text("string_10")?;
    reader.u32("time_18")?;
    reader.text("string_20")?;
    reader.text("string_28")?;
    if reader.version() >= 0x3b10 {
        reader.text("string_30")?;
    }
    Ok(())
}
