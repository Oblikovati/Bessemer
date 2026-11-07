use super::{DisplayArchiveLimits, DrawingBucketArchive, archive_error::invalid};
use crate::{Document, Result, header::ArchiveVersion};

impl DrawingBucketArchive {
    /// Reads `Contents/VBLists` with its model marker, independently of display-list markers.
    ///
    /// ```no_run
    /// # fn example(document: &bessemer::Document, limits: bessemer::display_lists::DisplayArchiveLimits) -> bessemer::Result<()> {
    /// let archive = bessemer::display_lists::DrawingBucketArchive::from_document(document, limits)?;
    /// # Ok(()) }
    /// ```
    pub fn from_document(document: &Document, limits: DisplayArchiveLimits) -> Result<Self> {
        let version = model_version(document).map_err(super::drawing_buckets::failure)?;
        let raw = document.read_stream_limited("Contents/VBLists", limits.maximum_bytes)?;
        Self::from_bytes(&raw, version, limits)
    }
}

fn model_version(document: &Document) -> Result<ArchiveVersion> {
    let mut version = None;
    for stream in document
        .streams()
        .filter(|stream| stream.name.starts_with("_MO_VERSION_"))
    {
        let parsed = ArchiveVersion::from_stream_name(stream.name)?;
        if version.is_some_and(|previous| previous != parsed) {
            return Err(invalid(format!(
                "drawing model versions {version:?} and {parsed:?}; expected one normalized _MO_VERSION_ value"
            )));
        }
        version = Some(parsed);
    }
    version.ok_or_else(|| crate::BessemerError {
        operation: "drawing model version",
        entity: "_MO_VERSION_".into(),
        class: crate::FailureClass::Missing,
        cause: "drawing has no model marker; expected _MO_VERSION_<decimal>/...".into(),
    })
}
