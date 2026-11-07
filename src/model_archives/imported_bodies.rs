use super::{ModelDecoder, body_features, kernel_frames, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    with_base_suffix: bool,
) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("imported", imported_payload)?;
    if with_base_suffix {
        reader.scope("base_body", base_suffix)?;
    }
    Ok(())
}

fn imported_payload(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.text("file_name_800")?;
    if reader.i32("external_body_marker")? == 0 {
        reader.scope("body_810", kernel_frames::read_body)?;
    }
    reader.text("body_name_808")
}

fn base_suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() <= 0x2bcc {
        return Ok(());
    }
    let count = reader.signed_count("raw_words_850.count", 4)?;
    sketch_arrays::integers(reader, "raw_words_850", count)
}
