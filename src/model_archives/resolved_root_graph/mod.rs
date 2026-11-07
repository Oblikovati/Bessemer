//! Stored root order and loaded object identities, separate from feature dependencies (#201).
mod accessors;
mod projection;
mod types;
use super::PartResolvedFeaturesArchive;
use crate::Result;
pub use types::*;

impl PartResolvedFeaturesArchive {
    /// Project every stored root and its loaded target; e.g. `let graph = archive.root_graph()?;`.
    pub fn root_graph(&self) -> Result<ModelResolvedRootGraph> {
        projection::project(self)
    }
}
