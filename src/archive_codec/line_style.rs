use super::MaterialRead;
use crate::Result;

pub(crate) fn width<R: MaterialRead>(archive: &mut R) -> Result<()> {
    if archive.version() < 0xf41 {
        return archive.i32("standard_width").map(|_| ());
    }
    short_width(archive)
}

pub(crate) fn short_width<R: MaterialRead>(archive: &mut R) -> Result<()> {
    archive.i16("standard_width")?;
    if archive.version() >= 0xf41 {
        archive.f32("custom_width")?;
    }
    Ok(())
}
