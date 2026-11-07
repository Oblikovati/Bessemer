//! Complete documented kind-two bodies and six typed model layouts, with explicit native exclusions.
mod decode;
mod edit;
mod fields;
mod model;
mod types;
mod view;
use super::medium_body::{self, CurveSpan, PrefixSpan};
use super::*;
pub use types::*;
impl FlowMediumBackendPrefix {
    /// Reads the first kind-two body after its sixteen-byte table key; e.g. `let body=backend.first_kind_two_body(options)?`.
    pub fn first_kind_two_body(
        &self,
        options: FlowMediumKindTwoOptions,
    ) -> Result<FlowMediumKindTwo> {
        decode::read(self, options).map_err(|error| table_boundary(error, 424))
    }
}

impl FlowMediumKindTwo {
    pub(super) fn read_at(
        backend: &FlowMediumBackendPrefix,
        start: usize,
        options: FlowMediumKindTwoOptions,
    ) -> Result<Self> {
        decode::read_at(backend, start, options).map_err(|error| table_boundary(error, 424))
    }
}
