use super::view_envelope::ViewEnvelope;
use super::{
    AnnotationViewContents, DisplayVersion, DisplayViewLimits, annotation_read, annotation_write,
};
use crate::Result;

/// Immutable complete annotation-view collection for an explicit display-list version.
#[derive(Debug, Clone)]
pub struct AnnotationViews {
    envelope: ViewEnvelope<Option<AnnotationViewContents>>,
}

impl AnnotationViews {
    /// Reads the complete annotation-view body, retaining absent versus present-empty collections.
    ///
    /// ```no_run
    /// # fn example(bytes:&[u8],version:bessemer::display_lists::DisplayVersion,limits:bessemer::display_lists::DisplayViewLimits)->bessemer::Result<()> {
    /// let views=bessemer::display_lists::AnnotationViews::from_bytes(bytes,version,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(
        bytes: &[u8],
        version: DisplayVersion,
        limits: DisplayViewLimits,
    ) -> Result<Self> {
        let views = annotation_read::read(bytes, version, limits)?;
        Ok(Self {
            envelope: ViewEnvelope::decoded(
                bytes,
                views,
                version,
                limits,
                annotation_write::encode,
                "preserved annotation-view bytes",
            ),
        })
    }

    /// Constructs a collection only after version gates, payload shapes and budgets pass.
    ///
    /// ```no_run
    /// # fn example(version:bessemer::display_lists::DisplayVersion,limits:bessemer::display_lists::DisplayViewLimits)->bessemer::Result<()> {
    /// let contents=bessemer::display_lists::AnnotationViewContents {views:Vec::new(),unknown_bitset:Some([0;2])};
    /// let views=bessemer::display_lists::AnnotationViews::new(Some(contents),bessemer::display_lists::DisplayVersion::new(19000),limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        views: Option<AnnotationViewContents>,
        version: DisplayVersion,
        limits: DisplayViewLimits,
    ) -> Result<Self> {
        Ok(Self {
            envelope: ViewEnvelope::new(
                views,
                version,
                limits,
                annotation_write::encode,
                "preserved annotation-view bytes",
            )?,
        })
    }

    /// Borrows the exact stored collection; None represents an inactive version gate.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::AnnotationViews) {
    /// let records=views.contents();
    /// # }
    /// ```
    pub fn contents(&self) -> &Option<AnnotationViewContents> {
        self.envelope.contents()
    }

    /// Returns the parsed unsigned version controlling every conditional field.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::AnnotationViews) {
    /// let version=views.version();
    /// # }
    /// ```
    pub fn version(&self) -> DisplayVersion {
        self.envelope.version()
    }

    /// Validates an owned edit before publishing a source-isolated collection.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::AnnotationViews,limits:bessemer::display_lists::DisplayViewLimits)->bessemer::Result<()> {
    /// let edited=views.with_contents(views.contents().clone(),limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_contents(
        &self,
        views: Option<AnnotationViewContents>,
        limits: DisplayViewLimits,
    ) -> Result<Self> {
        Ok(Self {
            envelope: self.envelope.edited(views, limits)?,
        })
    }

    /// Writes all typed fields, preserving equivalent edits and authentic source reverts.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::AnnotationViews)->bessemer::Result<()> {
    /// let bytes=views.to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.envelope.to_bytes()
    }
}
