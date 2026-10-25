use super::{DisplayArchiveContents, DisplayArchiveContext, DisplayArchiveLimits};
use crate::Result;

#[derive(Debug, Clone, Copy)]
pub(super) enum ArchiveLayout {
    DisplayLists,
    DrawingBuckets,
}

impl ArchiveLayout {
    pub fn read(
        self,
        bytes: &[u8],
        context: &DisplayArchiveContext,
        limits: DisplayArchiveLimits,
    ) -> Result<DisplayArchiveContents> {
        match self {
            Self::DisplayLists => super::modern_root::read(bytes, context, limits),
            Self::DrawingBuckets => super::drawing_bucket_read::read(bytes, context, limits),
        }
    }

    pub fn encode(
        self,
        contents: &DisplayArchiveContents,
        context: &DisplayArchiveContext,
        limits: DisplayArchiveLimits,
    ) -> Result<Vec<u8>> {
        match self {
            Self::DisplayLists => super::archive_encode::encode(contents, context, limits),
            Self::DrawingBuckets => {
                super::archive_encode::encode_layout(contents, context, limits, true)
            }
        }
    }
}
