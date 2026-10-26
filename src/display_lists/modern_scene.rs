use super::archive_decode::DisplayDecoder;
use crate::Result;
pub(super) use crate::archive_codec::vectors::double_vector;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.scope("concept", super::modern_concept::read)?;
    archive.object("pw_scene_data", &["pwSceneData_c"])?;
    let count = wall_count(archive, "walls.count", 8)?;
    for index in 0..count {
        archive.scope(&format!("walls[{index}]"), super::modern_walls::read)?;
    }
    archive.bounds("bbox")?;
    archive.i32("floor_orient")?;
    archive.transform("required_xform")?;
    archive.cobject("floor_selection")?;
    crate::archive_codec::scene_floor::read(archive)
}

pub(super) fn wall_count(
    archive: &mut DisplayDecoder<'_>,
    label: &str,
    minimum: usize,
) -> Result<usize> {
    crate::archive_codec::scene_data::nonnegative_count(archive, label, minimum)
}
