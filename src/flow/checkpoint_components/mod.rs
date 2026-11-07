//! Explicit logical checkpoint components do not decode physical record 121 or chunk codecs.
mod budget;
mod edit;
mod field;
mod layout;
mod read;
mod types;
mod view;
use super::*;
use crate::archive_codec::ArchiveCursor;
use layout::*;
pub use types::*;
pub use view::*;
const OPERATION: &str = "Flow logical checkpoint components";
fn invalid(entity: &str, cause: impl Into<String>) -> BessemerError {
    BessemerError::new(OPERATION, entity, FailureClass::Invalid, cause)
}
fn boundary(mut error: BessemerError) -> BessemerError {
    error.operation = OPERATION;
    error
}
impl FlowCheckpointComponent {
    /// Reads one complete explicitly selected logical layout; e.g. `let view = FlowCheckpointComponent::from_bytes(&raw, FlowCheckpointContext::LogicalVersionSix, FlowCheckpointKind::Graph1, options)?`.
    pub fn from_bytes(
        bytes: &[u8],
        context: FlowCheckpointContext,
        kind: FlowCheckpointKind,
        options: FlowCheckpointOptions,
    ) -> Result<Self> {
        read::component(bytes, context, kind, options).map_err(boundary)
    }
    fn field(&self, start: usize, shape: ValueShape, rule: ValueRule) -> FlowCheckpointField {
        FlowCheckpointField {
            revision: self.revision.clone(),
            start,
            shape,
            rule,
        }
    }
}
fn checked_index(index: usize, count: usize, entity: &str) -> Result<usize> {
    if index < count {
        return Ok(index);
    }
    Err(invalid(
        entity,
        format!("index {index}; expected an ordinal below {count}"),
    ))
}
