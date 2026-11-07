use super::archive_decode::DisplayDecoder;
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.wide_count(9, "configuration tail")?;
    for index in 0..count {
        archive.scope(&format!("entries[{index}]"), entry)?;
    }
    Ok(())
}

fn entry(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("key")?;
    let count = archive.wide_count(4, "configuration tail values")?;
    for index in 0..count {
        archive.i32(&format!("values[{index}]"))?;
    }
    Ok(())
}
