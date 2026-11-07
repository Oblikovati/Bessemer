//! Fresh-map replay of modern terminal indexes, separate from physical record selection.
mod build;
mod recognize;
mod types;
mod view;
use super::*;
use std::collections::BTreeMap;
pub use types::*;
const INDEX: u32 = 0xffffff9b;
const SAVED: u32 = 0xffffff9c;
const DELETED: u32 = 0xffffff9d;

impl FlowCarrier {
    /// Replays into fresh maps under an explicit policy; e.g. `let index = carrier.logical_index(FlowIndexPolicy::ForcePhysicalScan, options)?`.
    pub fn logical_index(
        &self,
        policy: FlowIndexPolicy,
        options: FlowIndexOptions,
    ) -> Result<FlowLogicalIndex> {
        build::read(self, policy, options).map_err(|error| BessemerError {
            operation: "Flow logical index replay",
            ..error
        })
    }
}
fn invalid(entity: &str, cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "Flow logical index replay",
        entity,
        FailureClass::Invalid,
        cause,
    )
}
fn physical_span(record: &FlowRecord<'_>) -> FlowIndexedSpan {
    FlowIndexedSpan {
        record_start: record.header_offset as u64,
        record_span: 12 + record.payload.len() as u64,
    }
}
