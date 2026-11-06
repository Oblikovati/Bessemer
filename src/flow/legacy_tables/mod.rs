//! Exact legacy pointer-index and raw-word tables with explicit independent context.
mod block_extras;
mod edit;
mod fields;
mod link_records;
mod pointers;
mod types;
mod view;
mod wire;
mod words;
use super::*;
use crate::archive_codec::ArchiveCursor;
pub use block_extras::*;
pub use fields::*;
pub use link_records::*;
pub use types::*;
use types::{GroupBodySpan, GroupSpan, TableWire};

impl FlowCarrier {
    /// Reads one selected legacy pointer table; e.g. `let table = carrier.record_401_pointer_table(&id, options)?`.
    pub fn record_401_pointer_table(
        &self,
        id: &FlowRecordId,
        options: FlowPointerTableOptions,
    ) -> Result<FlowPointerTables> {
        pointers::read(self, id, options).map_err(|error| table_boundary(error, 401))
    }
    /// Reads one selected legacy raw-word table; e.g. `let words = carrier.record_405_words(&id, options)?`.
    pub fn record_405_words(
        &self,
        id: &FlowRecordId,
        options: FlowLegacyWordOptions,
    ) -> Result<FlowLegacyWords> {
        words::read(self, id, options).map_err(|error| table_boundary(error, 405))
    }
}

fn table_failure(identifier: u32, class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new(
        "Flow legacy tables",
        &format!("record {identifier}"),
        class,
        cause,
    )
}

fn table_index_error(identifier: u32, field: &str, index: usize, count: usize) -> BessemerError {
    table_failure(
        identifier,
        FailureClass::Invalid,
        format!("{field} index {index}; expected below {count}"),
    )
}

fn table_boundary(mut error: BessemerError, identifier: u32) -> BessemerError {
    let entity = format!("record {identifier}");
    if error.operation == "Flow legacy tables" && error.entity == entity {
        return error;
    }
    error.operation = "Flow legacy tables";
    error.entity = format!("{entity}: {}", error.entity);
    error
}
