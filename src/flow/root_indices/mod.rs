//! Native root storage arithmetic and complete local wire paths, without global block ordering.
mod project;
mod types;
mod view;
use super::*;
pub use types::*;
use types::{LeafSpan, RootSpan};

impl FlowDerivedLeafCount {
    /// Projects complete local paths under the retained fresh-context assumption; e.g. `let roots = context.project_root_indices(limits)?`.
    pub fn project_root_indices(
        &self,
        limits: FlowRootIndexLimits,
    ) -> Result<FlowRootIndexProjection> {
        project::read(self, limits)
    }
}

fn projection_failure(class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new(
        "Flow root indices",
        "base grid and record 400",
        class,
        cause,
    )
}

fn invalid_projection(cause: String) -> BessemerError {
    projection_failure(FailureClass::Invalid, cause)
}
