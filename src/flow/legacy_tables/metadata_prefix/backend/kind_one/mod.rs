//! Complete kind-one serialized body, independent of numeric engineering interpretation.
mod decode;
#[cfg(test)]
mod decode_tests;
mod edit;
mod fields;
mod types;
mod view;
use super::medium_body::{self, CurveSpan};
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

impl<'a> FlowMediumCurve<'a> {
    pub(super) fn from_span(
        raw: &'a [u8],
        span: &'a CurveSpan,
        body_start: usize,
        ordinal: usize,
    ) -> Self {
        Self {
            raw,
            span,
            body_start,
            ordinal,
        }
    }
}
