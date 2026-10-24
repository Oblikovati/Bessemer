use super::{DisplayVersion, DisplayViewLimits, NamedView, named_read, named_write};
use crate::{Result, archive_codec::PreservedContents};

/// Immutable complete named-view collection for an explicit display-list version.
#[derive(Debug, Clone)]
pub struct NamedViews {
    version: DisplayVersion,
    source: PreservedContents<Option<Vec<NamedView>>>,
    limits: DisplayViewLimits,
}

impl NamedViews {
    /// Reads the complete named-view body, retaining absent versus present-empty collections.
    ///
    /// ```no_run
    /// # fn example(bytes:&[u8],version:bessemer::display_lists::DisplayVersion,limits:bessemer::display_lists::DisplayViewLimits)->bessemer::Result<()> {
    /// let views=bessemer::display_lists::NamedViews::from_bytes(bytes,version,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(
        bytes: &[u8],
        version: DisplayVersion,
        limits: DisplayViewLimits,
    ) -> Result<Self> {
        let views = named_read::read(bytes, version, limits)?;
        Ok(Self {
            version,
            source: PreservedContents::decoded(bytes, views),
            limits,
        })
    }

    /// Constructs a collection only after version gates, payload shapes and budgets pass.
    ///
    /// ```no_run
    /// # fn example(version:bessemer::display_lists::DisplayVersion,limits:bessemer::display_lists::DisplayViewLimits)->bessemer::Result<()> {
    /// let views=bessemer::display_lists::NamedViews::new(Some(Vec::new()),version,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        views: Option<Vec<NamedView>>,
        version: DisplayVersion,
        limits: DisplayViewLimits,
    ) -> Result<Self> {
        named_write::encode(version, &views, limits)?;
        Ok(Self {
            version,
            source: PreservedContents::constructed(views),
            limits,
        })
    }

    /// Borrows the exact stored collection; None represents an inactive version gate.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::NamedViews) {
    /// let records=views.contents();
    /// # }
    /// ```
    pub fn contents(&self) -> &Option<Vec<NamedView>> {
        self.source.current()
    }

    /// Returns the parsed unsigned version controlling every conditional field.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::NamedViews) {
    /// let version=views.version();
    /// # }
    /// ```
    pub fn version(&self) -> DisplayVersion {
        self.version
    }

    /// Validates an owned edit before publishing a source-isolated collection.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::NamedViews,limits:bessemer::display_lists::DisplayViewLimits)->bessemer::Result<()> {
    /// let edited=views.with_contents(views.contents().clone(),limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_contents(
        &self,
        views: Option<Vec<NamedView>>,
        limits: DisplayViewLimits,
    ) -> Result<Self> {
        named_write::encode(self.version, &views, limits)?;
        let source = self.source.edited(views);
        // #29: CString-equivalent fields can retain a longer authentic prefix encoding.
        if let Some(bytes) = source.original_bytes() {
            super::view_support::require_size(
                bytes.len(),
                limits.maximum_bytes,
                "preserved named-view bytes",
            )?;
        }
        Ok(Self {
            version: self.version,
            source,
            limits,
        })
    }

    /// Writes all typed fields, preserving equivalent edits and authentic source reverts.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::NamedViews)->bessemer::Result<()> {
    /// let bytes=views.to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.source.original_bytes() {
            return Ok(bytes.to_vec());
        }
        named_write::encode(self.version, self.contents(), self.limits)
    }
}
