use super::context_reader::ArchiveGraphRead;
use crate::Result;

pub(crate) fn edge_array<R: ArchiveGraphRead>(
    archive: &mut R,
    context: &mut R::ObjectContext,
    two_dimensional: bool,
) -> Result<()> {
    if archive.version() >= 0xa5a {
        return Ok(());
    }
    let minimum = match archive.version() {
        0..=0x231 => 20,
        0x744..=0x74a => 28,
        0x232..=0x743 | 0x74b..=0xa4a => 24,
        _ => 20,
    };
    let count = super::scene_data::nonnegative_count(archive, "edge_vis_data.count", minimum)?;
    for index in 0..count {
        archive.scope(&format!("edge_vis_data[{index}]"), |archive| {
            edge_record(archive, context, two_dimensional)
        })?;
    }
    Ok(())
}

fn edge_record<R: ArchiveGraphRead>(
    archive: &mut R,
    context: &mut R::ObjectContext,
    two_dimensional: bool,
) -> Result<()> {
    archive.i32s(&["unknown_0", "unknown_4"])?;
    if archive.version() < 0xa4b {
        // #28: the native reader drops this local; exact reconstruction still needs its word.
        archive.i32("discarded_old_8")?;
    }
    archive.i32("unknown_8")?;
    if archive.version() > 0x231 {
        archive.i32("unknown_c")?;
    }
    if (0x744..=0x74a).contains(&archive.version()) {
        archive.i32("discarded_v0744")?;
    }
    archive.scope("polys", |archive| {
        super::bucket_polygons::list(archive, context, two_dimensional)
    })
}
