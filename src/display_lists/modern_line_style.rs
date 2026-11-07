use super::{archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("internal_name")?;
    archive.text("external_name")?;
    let count = usize::from(archive.u16("segments.count")?);
    if count > archive.cursor.remaining() / 8 {
        return Err(invalid(format!(
            "line-style segment count {count}; expected doubles fitting {} bytes",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count, "line-style segments")?;
    for index in 0..count {
        archive.f64(&format!("segments[{index}]"))?;
    }
    archive.u8("thick_thin")?;
    Ok(())
}
