use super::archive_decode::DisplayDecoder;
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    crate::archive_codec::scene_data::read(archive)
}
