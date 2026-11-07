use super::archive_decode::DisplayDecoder;
use crate::{
    Result,
    archive_codec::light_payload::{self, LightPayloadBranch},
};

pub(super) fn ambient(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    light_payload::ambient(archive, LightPayloadBranch::OldDisplay)
}

pub(super) fn directional(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    light_payload::directional(archive, LightPayloadBranch::OldDisplay)
}

pub(super) fn spot(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    light_payload::spot(archive, LightPayloadBranch::OldDisplay)
}

pub(super) fn point(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    light_payload::point(archive, LightPayloadBranch::OldDisplay)
}
