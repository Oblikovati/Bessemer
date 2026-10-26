use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn chooser(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["array_08", "array_28", "array_48"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    reader.i32s(&["raw_68", "raw_6c"])?;
    model_lists::unsigned_words(reader, "dwords_78")?;
    reader.i32("raw_70")?;
    Ok(())
}

pub(super) fn named_chooser(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    chooser(reader, map)?;
    reader.text("filename_98")?;
    reader.object(
        "reference_a0",
        map,
        ModelObjectRole::Declared(&["moRefWrapper_c"]),
    )?;
    reader.i32("raw_a8")?;
    Ok(())
}

pub(super) fn bounding_center(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_08")?;
    reader.f64s(&["point_10.x", "point_10.y", "point_10.z"])?;
    reader.f64("raw_28")
}
