//! Exact legacy pointer-index and raw-word tables with explicit independent context.
mod block_extras;
mod byte_states;
mod conditional_items;
mod context_state;
mod counted_bytes;
mod edit;
mod field_values;
mod fields;
mod fixed_state;
mod id_states;
mod link_records;
mod load_state;
mod message_lists;
mod metadata_prefix;
mod object_records;
mod paired_state;
mod pointer_bodies;
mod pointers;
mod primary_table;
mod reference_groups;
mod selected_values;
mod simple_tables;
mod stored_state;
mod tagged_state;
mod types;
mod view;
mod wire;
mod words;
use super::*;
use crate::archive_codec::ArchiveCursor;
pub use block_extras::*;
pub use byte_states::*;
pub use conditional_items::*;
pub use context_state::*;
pub use counted_bytes::*;
pub use field_values::*;
pub use fields::*;
pub use fixed_state::*;
pub use id_states::*;
pub use link_records::*;
pub use load_state::*;
pub use message_lists::*;
pub use metadata_prefix::*;
pub use object_records::*;
pub use paired_state::*;
pub use pointer_bodies::*;
pub use primary_table::*;
pub use reference_groups::*;
pub use selected_values::*;
pub use simple_tables::*;
pub use stored_state::*;
pub use tagged_state::*;
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
