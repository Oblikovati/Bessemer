use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<(i32, bool)> {
    prefix(reader, map)?;
    let count = pairs(reader)?;
    middle(reader)?;
    text_fields(reader, &["a30", "a38"])?;
    let enabled = reader.i32("raw_a40")? != 0;
    reader.f64("raw_a48")?;
    reader.i32s(&["raw_a58", "raw_a5c", "raw_a60"])?;
    Ok((count, enabled))
}

fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_80c")?;
    reader.object("object_810", map, ModelObjectRole::Any)?;
    reader.f64("raw_818")?;
    reader.i32("raw_820")?;
    text_fields(reader, &["828", "830"])?;
    reader.i32s(&["raw_844", "raw_848", "raw_84c"])?;
    text_fields(reader, &["850", "858", "860"])?;
    reader.f64s(&["raw_870", "raw_878", "raw_880", "raw_888", "raw_890"])?;
    reader.text("raw_868")?;
    reader.i32s(&["raw_898", "raw_8c8"])?;
    text_fields(reader, &["8a0", "8b0"])?;
    reader.i32s(&["raw_910", "raw_914"])?;
    reader.text("raw_8b8")?;
    reader.i32("raw_840")?;
    Ok(())
}

fn pairs(reader: &mut ModelDecoder<'_>) -> Result<i32> {
    let stored = reader.i32("pairs_958.count")?;
    for index in 0..reader.signed_count_value("pairs_958", stored, 12)? {
        reader.scope(&format!("pairs_958[{index}]"), |reader| {
            reader.i32("raw_key")?;
            reader.f64("raw_value")
        })?;
    }
    Ok(stored)
}

fn middle(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32s(&[
        "raw_8cc", "raw_8d0", "raw_8d4", "raw_8d8", "raw_8dc", "raw_8e0", "raw_8e8", "raw_8ec",
    ])?;
    reader.f64("raw_8f0")?;
    reader.i32s(&["raw_8f8", "raw_8fc", "raw_8e4"])?;
    reader.text("raw_8c0")?;
    reader.i32s(&[
        "raw_980", "raw_900", "raw_904", "raw_908", "raw_90c", "raw_918", "raw_a2c", "raw_a28",
    ])
}

fn text_fields(reader: &mut ModelDecoder<'_>, offsets: &[&str]) -> Result<()> {
    for offset in offsets {
        reader.text(&format!("raw_{offset}"))?;
    }
    Ok(())
}
