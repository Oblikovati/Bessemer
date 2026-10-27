use super::{ModelDecoder, invalid, sketch_arrays};
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn read(reader: &mut ModelDecoder<'_>, count: usize) -> Result<()> {
    if count > reader.remaining() / 36 {
        return reader.checked(Err(invalid(format!(
            "blend column count {count}; expected <= {} complete 36-byte rows",
            reader.remaining() / 36,
        ))));
    }
    reader.entries(count * 6, "blend numeric columns")?;
    words(reader, "2f0", count)?;
    words(reader, "2f8", count)?;
    for index in 0..count {
        sketch_arrays::doubles(reader, &format!("raw_points_2d8[{index}]"), 3)?;
    }
    sketch_arrays::integers(reader, "raw_words_2e8", count)
}

fn words(reader: &mut ModelDecoder<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        reader.u32(&format!("raw_words_{label}[{index}]"))?;
    }
    Ok(())
}
