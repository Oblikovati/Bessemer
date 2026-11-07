use super::{archive_decode::DisplayDecoder, modern_scene::wall_count};
use crate::Result;

pub(super) fn edge_array(archive: &mut DisplayDecoder<'_>, two_dimensional: bool) -> Result<()> {
    crate::archive_codec::bucket_visibility::edge_array(archive, &mut (), two_dimensional)
}

pub(super) fn view_triples(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if !(0xa43..0xa5a).contains(&archive.context.version.value()) {
        return Ok(());
    }
    let count = wall_count(archive, "unknown_old_triples.count", 12)?;
    for index in 0..count {
        for column in 0..3 {
            archive.i32(&format!("unknown_old_triples[{index}][{column}]"))?;
        }
    }
    Ok(())
}
