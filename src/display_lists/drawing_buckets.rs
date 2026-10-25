use super::{DisplayArchive, DisplayArchiveContents, DisplayArchiveContext, DisplayArchiveLimits};
use super::{DisplayDocumentKind, DisplayVersion, archive_layout::ArchiveLayout};
use crate::{Result, header::ArchiveVersion};

/// Complete drawing `Contents/VBLists`, using the model archive version.
/// Unknown meanings remain raw typed fields; no opaque remainder is accepted.
#[derive(Debug, Clone)]
pub struct DrawingBucketArchive(DisplayArchive);

impl DrawingBucketArchive {
    /// Reads a complete drawing bucket stream with the document's model version.
    ///
    /// ```no_run
    /// # fn example(bytes: &[u8], limits: bessemer::display_lists::DisplayArchiveLimits) -> bessemer::Result<()> {
    /// let archive = bessemer::display_lists::DrawingBucketArchive::from_bytes(bytes, bessemer::header::ArchiveVersion::new(19000), limits)?;
    /// # Ok(()) }
    /// ```
    pub fn from_bytes(
        bytes: &[u8],
        model_version: ArchiveVersion,
        limits: DisplayArchiveLimits,
    ) -> Result<Self> {
        DisplayArchive::decode_layout(
            bytes,
            context(model_version),
            limits,
            ArchiveLayout::DrawingBuckets,
        )
        .map(Self)
        .map_err(failure)
    }

    /// Forces reconstruction and complete validation of owned drawing fields.
    ///
    /// ```no_run
    /// # fn example(contents: bessemer::display_lists::DisplayArchiveContents, limits: bessemer::display_lists::DisplayArchiveLimits) -> bessemer::Result<()> {
    /// let archive = bessemer::display_lists::DrawingBucketArchive::new(contents, bessemer::header::ArchiveVersion::new(19000), limits)?;
    /// # Ok(()) }
    /// ```
    pub fn new(
        contents: DisplayArchiveContents,
        model_version: ArchiveVersion,
        limits: DisplayArchiveLimits,
    ) -> Result<Self> {
        DisplayArchive::construct_layout(
            contents,
            context(model_version),
            limits,
            ArchiveLayout::DrawingBuckets,
        )
        .map(Self)
        .map_err(failure)
    }

    /// Borrows all ordered stored fields, including shared object tags.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::display_lists::DrawingBucketArchive) {
    /// let fields = &archive.contents().fields;
    /// # }
    /// ```
    pub fn contents(&self) -> &DisplayArchiveContents {
        self.0.contents()
    }

    /// Returns the model version that selected this stream's layout.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::display_lists::DrawingBucketArchive) {
    /// let version = archive.model_version();
    /// # }
    /// ```
    pub fn model_version(&self) -> ArchiveVersion {
        ArchiveVersion::new(self.0.context().version.value())
    }

    /// Validates an isolated edit, preserving equivalent fields and authentic reverts.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::display_lists::DrawingBucketArchive, limits: bessemer::display_lists::DisplayArchiveLimits) -> bessemer::Result<()> {
    /// let copy = archive.with_contents(archive.contents().clone(), limits)?;
    /// # Ok(()) }
    /// ```
    pub fn with_contents(
        &self,
        contents: DisplayArchiveContents,
        limits: DisplayArchiveLimits,
    ) -> Result<Self> {
        self.0
            .with_contents(contents, limits)
            .map(Self)
            .map_err(failure)
    }

    /// Writes the entire stream, preserving stored scalar and text encodings.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::display_lists::DrawingBucketArchive) -> bessemer::Result<()> {
    /// let bytes = archive.to_bytes()?;
    /// # Ok(()) }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.0.to_bytes().map_err(failure)
    }
}

fn context(model_version: ArchiveVersion) -> DisplayArchiveContext {
    // Drawing models always carry inline facets; no assembly external-face substitution applies.
    DisplayArchiveContext {
        version: DisplayVersion::new(model_version.value()),
        kind: DisplayDocumentKind::Part,
        file_version: model_version.value(),
        face_tessellation_components: Default::default(),
    }
}

pub(super) fn failure(mut error: crate::BessemerError) -> crate::BessemerError {
    error.operation = "drawing VBLists archive";
    error.entity = "Contents/VBLists".into();
    error
}
