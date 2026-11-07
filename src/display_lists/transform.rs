use super::{TransformContents, TransformEncoding, transform_codec};
use crate::{Result, archive_codec::ArchiveCursor};

/// Immutable stored transform; archive checks do not establish geometric validity.
#[derive(Debug, Clone)]
pub struct DisplayTransform {
    contents: TransformContents,
}

impl DisplayTransform {
    /// Reads one complete transform using the caller's explicit native overload.
    ///
    /// ```no_run
    /// # fn example(bytes: &[u8]) -> bessemer::Result<()> {
    /// use bessemer::display_lists::{DisplayTransform, TransformEncoding};
    /// let transform = DisplayTransform::from_bytes(bytes, TransformEncoding::Fixed)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(bytes: &[u8], encoding: TransformEncoding) -> Result<Self> {
        let mut cursor = ArchiveCursor::new(bytes);
        let contents = transform_codec::read(&mut cursor, encoding)?;
        cursor.finish()?;
        Self::new(contents)
    }

    /// Constructs a complete transform after checking flag/payload consistency.
    ///
    /// ```no_run
    /// # fn example(contents: bessemer::display_lists::TransformContents) -> bessemer::Result<()> {
    /// let transform = bessemer::display_lists::DisplayTransform::new(contents)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(contents: TransformContents) -> Result<Self> {
        transform_codec::validate(&contents)?;
        Ok(Self { contents })
    }

    /// Borrows immutable fields, including every raw flag and stored numeric bit.
    ///
    /// ```no_run
    /// # fn example(transform: &bessemer::display_lists::DisplayTransform) {
    /// let stored = transform.contents();
    /// # }
    /// ```
    pub fn contents(&self) -> &TransformContents {
        &self.contents
    }

    /// Validates an owned edit before publishing an isolated transform.
    ///
    /// ```no_run
    /// # fn example(transform: &bessemer::display_lists::DisplayTransform) -> bessemer::Result<()> {
    /// let unchanged = transform.with_contents(transform.contents().clone())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_contents(&self, contents: TransformContents) -> Result<Self> {
        Self::new(contents)
    }

    /// Reconstructs all wire fields without a stored-source shortcut.
    ///
    /// ```no_run
    /// # fn example(transform: &bessemer::display_lists::DisplayTransform) {
    /// let bytes = transform.to_bytes();
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Vec<u8> {
        transform_codec::encode(&self.contents)
    }
}
