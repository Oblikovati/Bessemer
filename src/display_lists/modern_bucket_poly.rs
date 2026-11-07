use super::archive_decode::DisplayDecoder;
use crate::Result;
pub(super) use crate::archive_codec::bucket_polygons::edge;

pub(super) fn list(archive: &mut DisplayDecoder<'_>, two_dimensional: bool) -> Result<()> {
    crate::archive_codec::bucket_polygons::list(archive, &mut (), two_dimensional)
}
