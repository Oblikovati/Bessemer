//! Complete kind-one serialized body, independent of numeric engineering interpretation.
mod decode;
mod edit;
mod fields;
mod types;
mod view;
use super::super::decode as metadata_decode;
use super::*;
pub use types::*;
impl FlowMediumBackendPrefix {
    /// Reads the first reached kind-one body under explicit limits; e.g. `let body = backend.first_kind_one_body(options)?`.
    pub fn first_kind_one_body(&self, options: FlowMediumBodyOptions) -> Result<FlowMediumKindOne> {
        decode::read(self, options).map_err(|error| table_boundary(error, 424))
    }
}

impl FlowMediumKindOne {
    pub(super) fn read_at(
        backend: &FlowMediumBackendPrefix,
        start: usize,
        options: FlowMediumBodyOptions,
    ) -> Result<Self> {
        decode::read_at(backend, start, options).map_err(|error| table_boundary(error, 424))
    }
}
