use super::{
    DisplayArchiveContents, DisplayArchiveContext, DisplayArchiveLimits, archive_encode,
    archive_error::invalid, modern_root,
};
use crate::{Result, archive_codec::PreservedContents};

/// Immutable complete display-list stream; the initial checkpoint supports modern part/assembly layouts.
#[derive(Debug, Clone)]
pub struct DisplayArchive {
    context: DisplayArchiveContext,
    source: PreservedContents<DisplayArchiveContents>,
    limits: DisplayArchiveLimits,
}

impl DisplayArchive {
    /// Reads the entire stream or returns a typed failure; no unresolved remainder is published.
    ///
    /// ```no_run
    /// # fn example(bytes:&[u8],context:bessemer::display_lists::DisplayArchiveContext,limits:bessemer::display_lists::DisplayArchiveLimits)->bessemer::Result<()> {
    /// let archive=bessemer::display_lists::DisplayArchive::from_bytes(bytes,context,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(
        bytes: &[u8],
        context: DisplayArchiveContext,
        limits: DisplayArchiveLimits,
    ) -> Result<Self> {
        let contents = modern_root::read(bytes, context, limits)?;
        Ok(Self {
            context,
            source: PreservedContents::decoded(bytes, contents),
            limits,
        })
    }

    /// Forces typed-field reconstruction and complete revalidation without original-byte shortcuts.
    ///
    /// ```no_run
    /// # fn example(contents:bessemer::display_lists::DisplayArchiveContents,context:bessemer::display_lists::DisplayArchiveContext,limits:bessemer::display_lists::DisplayArchiveLimits)->bessemer::Result<()> {
    /// let archive=bessemer::display_lists::DisplayArchive::new(contents,context,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        contents: DisplayArchiveContents,
        context: DisplayArchiveContext,
        limits: DisplayArchiveLimits,
    ) -> Result<Self> {
        validate(&contents, context, limits)?;
        Ok(Self {
            context,
            source: PreservedContents::constructed(contents),
            limits,
        })
    }

    /// Borrows complete ordered typed fields and their parser-derived paths.
    ///
    /// ```no_run
    /// # fn example(archive:&bessemer::display_lists::DisplayArchive) {
    /// let fields=&archive.contents().fields;
    /// # }
    /// ```
    pub fn contents(&self) -> &DisplayArchiveContents {
        self.source.current()
    }

    /// Returns the explicit version and document context used for decoding.
    ///
    /// ```no_run
    /// # fn example(archive:&bessemer::display_lists::DisplayArchive) {
    /// let context=archive.context();
    /// # }
    /// ```
    pub fn context(&self) -> DisplayArchiveContext {
        self.context
    }

    /// Revalidates a complete owned edit before publishing an isolated stream snapshot.
    ///
    /// ```no_run
    /// # fn example(archive:&bessemer::display_lists::DisplayArchive,limits:bessemer::display_lists::DisplayArchiveLimits)->bessemer::Result<()> {
    /// let copy=archive.with_contents(archive.contents().clone(),limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_contents(
        &self,
        contents: DisplayArchiveContents,
        limits: DisplayArchiveLimits,
    ) -> Result<Self> {
        validate(&contents, self.context, limits)?;
        let source = self.source.edited(contents);
        if let Some(bytes) = source.original_bytes() {
            super::view_support::require_size(
                bytes.len(),
                limits.maximum_bytes,
                "preserved display-list bytes",
            )?;
        }
        Ok(Self {
            context: self.context,
            source,
            limits,
        })
    }

    /// Writes complete typed contents, retaining equivalent edits and authentic reverts.
    ///
    /// ```no_run
    /// # fn example(archive:&bessemer::display_lists::DisplayArchive)->bessemer::Result<()> {
    /// let bytes=archive.to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.source.original_bytes() {
            return Ok(bytes.to_vec());
        }
        archive_encode::encode(self.contents(), self.context, self.limits)
    }
}

fn validate(
    contents: &DisplayArchiveContents,
    context: DisplayArchiveContext,
    limits: DisplayArchiveLimits,
) -> Result<()> {
    let encoded = archive_encode::encode(contents, context, limits)?;
    let parsed = modern_root::read(&encoded, context, limits)?;
    if &parsed == contents {
        return Ok(());
    }
    let first = parsed
        .fields
        .iter()
        .zip(&contents.fields)
        .position(|(expected, actual)| expected != actual);
    Err(invalid(format!(
        "typed display fields differ at {first:?}, supplied count {}, parsed count {}; expected exact schema types, paths and complete wire order",
        contents.fields.len(),
        parsed.fields.len()
    )))
}
