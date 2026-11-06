//! Complete two-group reference lists, without native reference repair or refinement.
mod decode;
mod edit;
mod selection;
mod types;
mod view;
use super::super::block_data::{FieldSelection, ScalarKind, encode};
use super::super::entry_continuation::{self, EntryContinuationContext, EntryContinuationSpan};
use super::*;
pub use types::*;
use types::{ReferenceEntrySpan, ReferenceGroupSpan};

impl FlowCarrier {
    /// Reads complete reference groups under explicit mapping context; e.g. `let groups = carrier.record_416_references(&id, options)?`.
    pub fn record_416_references(
        &self,
        id: &FlowRecordId,
        options: FlowReferenceGroupOptions,
    ) -> Result<FlowReferenceGroups> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 416))
    }
}

fn reference_invalid(cause: String) -> BessemerError {
    table_failure(416, FailureClass::Invalid, cause)
}

fn reference_missing(field: String) -> BessemerError {
    table_failure(
        416,
        FailureClass::Missing,
        format!("{field} absent; expected an existing serialized field"),
    )
}
