use super::{archive_decode::DisplayDecoder, modern_bucket_poly, modern_scene::wall_count};
use crate::Result;

pub(super) fn edge_array(archive: &mut DisplayDecoder<'_>, two_dimensional: bool) -> Result<()> {
    if archive.context.version.value() >= 0xa5a {
        return Ok(());
    }
    let minimum = match archive.context.version.value() {
        0..=0x231 => 20,
        0x744..=0x74a => 28,
        0x232..=0x743 | 0x74b..=0xa4a => 24,
        _ => 20,
    };
    let count = wall_count(archive, "edge_vis_data.count", minimum)?;
    for index in 0..count {
        archive.scope(&format!("edge_vis_data[{index}]"), |archive| {
            edge_record(archive, two_dimensional)
        })?;
    }
    Ok(())
}

fn edge_record(archive: &mut DisplayDecoder<'_>, two_dimensional: bool) -> Result<()> {
    archive.i32s(&["unknown_0", "unknown_4"])?;
    if archive.context.version.value() < 0xa4b {
        // #28: the native reader drops this local; exact reconstruction still needs its word.
        archive.i32("discarded_old_8")?;
    }
    archive.i32("unknown_8")?;
    if archive.context.version.value() > 0x231 {
        archive.i32("unknown_c")?;
    }
    if (0x744..=0x74a).contains(&archive.context.version.value()) {
        archive.i32("discarded_v0744")?;
    }
    archive.scope("polys", |archive| {
        modern_bucket_poly::list(archive, two_dimensional)
    })
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
