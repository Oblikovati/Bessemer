use super::{ModelDecoder, invalid};
use crate::Result;
use crate::archive_codec::MaterialRead;

pub(super) fn count(reader: &mut ModelDecoder<'_>, label: &str) -> Result<usize> {
    reader.scope(label, read)
}

fn read(reader: &mut ModelDecoder<'_>) -> Result<usize> {
    let size = reader.u32("element_size")?;
    if size == 0 {
        return Ok(0);
    }
    reader.checked(require_element_size(size))?;
    reader.u32("capacity")?;
    reader.u32("growth")?;
    let count = reader.u32("count")?;
    let bytes = u64::from(size) * u64::from(count);
    let words = bounded_words(reader, bytes)?;
    for index in 0..words {
        reader.u64(&format!("payload_words[{index}]"))?;
    }
    Ok(count as usize)
}

fn require_element_size(size: u32) -> Result<()> {
    if size != 0 && size.is_multiple_of(8) {
        return Ok(());
    }
    Err(invalid(format!(
        "sheet-metal array element size {size}; expected a nonzero multiple of eight"
    )))
}

fn bounded_words(reader: &mut ModelDecoder<'_>, bytes: u64) -> Result<usize> {
    if bytes > reader.remaining() as u64 {
        return reader.checked(Err(invalid(format!(
            "sheet-metal array payload {bytes} bytes; expected <= {} remaining bytes",
            reader.remaining(),
        ))));
    }
    let words = (bytes / 8) as usize;
    reader.entries(words, "sheet-metal array payload words")?;
    Ok(words)
}
