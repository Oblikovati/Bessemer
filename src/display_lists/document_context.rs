use super::{DisplayArchiveContext, DisplayDocumentKind, DisplayVersion};
use crate::{
    Document, Result,
    tessellation::{TessellationContext, TessellationDirectory, TessellationLimits},
};
use std::{collections::BTreeSet, sync::Arc};

const DIRECTORY: &str = "FaceTessellations/Directory";

impl DisplayArchiveContext {
    /// Resolves display-version markers and external component declarations once for a document.
    /// Native file and tessellation versions remain explicit application inputs. This resolves
    /// context only; it does not check external blocks or certify their geometry.
    ///
    /// ```no_run
    /// # fn example(document: &bessemer::Document, tessellation: bessemer::tessellation::TessellationContext, limits: bessemer::tessellation::TessellationLimits) -> bessemer::Result<()> {
    /// let context = bessemer::display_lists::DisplayArchiveContext::from_document(
    ///     document, bessemer::display_lists::DisplayDocumentKind::Assembly,
    ///     19000, tessellation, limits, 8 * 1024 * 1024,
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_document(
        document: &Document,
        kind: DisplayDocumentKind,
        file_version: u32,
        tessellation: TessellationContext,
        limits: TessellationLimits,
        maximum_directory_bytes: usize,
    ) -> Result<Self> {
        let version =
            DisplayVersion::from_stream_names(document.streams().map(|stream| stream.name))?;
        let face_tessellation_components =
            directory_ids(document, tessellation, limits, maximum_directory_bytes)?;
        Ok(Self {
            version,
            kind,
            file_version,
            face_tessellation_components,
        })
    }
}

fn directory_ids(
    document: &Document,
    context: TessellationContext,
    limits: TessellationLimits,
    maximum_bytes: usize,
) -> Result<Arc<BTreeSet<String>>> {
    if !document.streams().any(|stream| stream.name == DIRECTORY) {
        return Ok(Arc::new(BTreeSet::new()));
    }
    let bytes = document.read_stream_limited(DIRECTORY, maximum_bytes)?;
    let directory = TessellationDirectory::from_bytes(&bytes, context, limits)?;
    Ok(Arc::new(directory.component_ids()))
}
