use super::view_envelope::ViewEnvelope;
use super::{DisplayVersion, DisplayViewLimits, SpatialView, spatial_read, spatial_write};
use crate::Result;

/// Immutable complete 3D-view collection for an explicit display-list version.
#[derive(Debug, Clone)]
pub struct SpatialViews {
    envelope: ViewEnvelope<Option<Vec<SpatialView>>>,
}

impl SpatialViews {
    /// Reads the complete 3D-view body, retaining absent versus present-empty collections.
    ///
    /// ```no_run
    /// # fn example(bytes:&[u8],version:bessemer::display_lists::DisplayVersion,limits:bessemer::display_lists::DisplayViewLimits)->bessemer::Result<()> {
    /// let views=bessemer::display_lists::SpatialViews::from_bytes(bytes,version,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(
        bytes: &[u8],
        version: DisplayVersion,
        limits: DisplayViewLimits,
    ) -> Result<Self> {
        let views = spatial_read::read(bytes, version, limits)?;
        Ok(Self {
            envelope: ViewEnvelope::decoded(
                bytes,
                views,
                version,
                limits,
                spatial_write::encode,
                "preserved 3D-view bytes",
            ),
        })
    }

    /// Constructs a collection only after version gates, payload shapes and budgets pass.
    ///
    /// ```no_run
    /// # fn example(version:bessemer::display_lists::DisplayVersion,limits:bessemer::display_lists::DisplayViewLimits)->bessemer::Result<()> {
    /// let views=bessemer::display_lists::SpatialViews::new(Some(Vec::new()),version,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        views: Option<Vec<SpatialView>>,
        version: DisplayVersion,
        limits: DisplayViewLimits,
    ) -> Result<Self> {
        Ok(Self {
            envelope: ViewEnvelope::new(
                views,
                version,
                limits,
                spatial_write::encode,
                "preserved 3D-view bytes",
            )?,
        })
    }

    /// Borrows the exact stored collection; None represents an inactive version gate.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::SpatialViews) {
    /// let records=views.contents();
    /// # }
    /// ```
    pub fn contents(&self) -> &Option<Vec<SpatialView>> {
        self.envelope.contents()
    }

    /// Returns the parsed unsigned version controlling every conditional field.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::SpatialViews) {
    /// let version=views.version();
    /// # }
    /// ```
    pub fn version(&self) -> DisplayVersion {
        self.envelope.version()
    }

    /// Validates an owned edit before publishing a source-isolated collection.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::SpatialViews,limits:bessemer::display_lists::DisplayViewLimits)->bessemer::Result<()> {
    /// let edited=views.with_contents(views.contents().clone(),limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_contents(
        &self,
        views: Option<Vec<SpatialView>>,
        limits: DisplayViewLimits,
    ) -> Result<Self> {
        Ok(Self {
            envelope: self.envelope.edited(views, limits)?,
        })
    }

    /// Writes all typed fields, preserving equivalent edits and authentic source reverts.
    ///
    /// ```no_run
    /// # fn example(views:&bessemer::display_lists::SpatialViews)->bessemer::Result<()> {
    /// let bytes=views.to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.envelope.to_bytes()
    }
}
