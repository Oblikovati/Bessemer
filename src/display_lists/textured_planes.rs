use super::archive_decode::DisplayDecoder;
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.count("count", 36)?;
    for index in 0..count {
        archive.scope(&format!("items[{index}]"), plane)?;
    }
    Ok(())
}

fn plane(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.object("sketch_bitmap", &["moSketchBitmap_c"])?;
    archive.transform("xform")
}
