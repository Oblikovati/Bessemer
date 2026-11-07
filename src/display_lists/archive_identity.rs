use super::{
    DisplayArchive, DisplayArchiveContents, DisplayField, DisplayFieldValue, DisplayObjectId,
    DisplayObjectPointer,
};
pub(super) use crate::archive_codec::ArchiveRevision as DisplayRevision;
use crate::{BessemerError, FailureClass, Result};

/// Object identity bound to one immutable archive revision, independent of pointer-tag spelling.
/// Clones and equivalent edits share a revision. Changed edits and separately decoded archives do not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayObjectHandle {
    revision: DisplayRevision,
    object_id: DisplayObjectId,
}

impl DisplayObjectHandle {
    /// Returns the persisted map index without dropping the handle's revision binding.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::display_lists::DisplayArchive, id: bessemer::display_lists::DisplayObjectId) -> bessemer::Result<()> {
    /// assert_eq!(archive.object_handle(id)?.object_id(), id);
    /// # Ok(())
    /// # }
    /// ```
    pub fn object_id(&self) -> DisplayObjectId {
        self.object_id
    }
}

impl DisplayArchive {
    /// Binds an existing loaded object to this revision; null and class-map slots are not objects.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::display_lists::DisplayArchive) -> bessemer::Result<()> {
    /// let id = bessemer::display_lists::DisplayObjectId::new(2)?;
    /// let handle = archive.object_handle(id)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn object_handle(&self, object_id: DisplayObjectId) -> Result<DisplayObjectHandle> {
        self.object_definition(object_id)?;
        Ok(DisplayObjectHandle {
            revision: self.revision.clone(),
            object_id,
        })
    }

    /// Resolves the defining tag only in its owning revision; aliases identify the same definition.
    /// Lookup scans the bounded ordered field sequence and never changes stored map indices.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::display_lists::DisplayArchive, id: bessemer::display_lists::DisplayObjectId) -> bessemer::Result<()> {
    /// let handle = archive.object_handle(id)?;
    /// let definition = archive.resolve_object(&handle)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn resolve_object(&self, handle: &DisplayObjectHandle) -> Result<&DisplayField> {
        if self.revision != handle.revision {
            return Err(identity_error(
                handle.object_id,
                "handle belongs to another archive revision; expected this revision",
            ));
        }
        self.object_definition(handle.object_id)
    }

    fn object_definition(&self, object_id: DisplayObjectId) -> Result<&DisplayField> {
        self.contents().fields.iter().find(|field| {
            matches!(&field.value, DisplayFieldValue::ObjectTag(tag) if tag.target == DisplayObjectPointer::New(object_id))
        }).ok_or_else(|| identity_error(object_id, "map slot has no defining object; expected a loaded object rather than a class or missing slot"))
    }

    pub(super) fn edited_revision(&self, contents: &DisplayArchiveContents) -> DisplayRevision {
        if self.contents() == contents {
            return self.revision.clone();
        }
        DisplayRevision::default()
    }
}

fn identity_error(object_id: DisplayObjectId, cause: &str) -> BessemerError {
    BessemerError {
        operation: "resolve display object",
        entity: format!("Contents/DisplayLists object {}", object_id.value()),
        class: FailureClass::Invalid,
        cause: cause.into(),
    }
}
