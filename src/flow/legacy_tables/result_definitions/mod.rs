//! Complete byte-versioned IMS definitions; object identifiers precede all GUID/body pairs.
mod edit;
mod read;
mod types;
mod view;
use super::*;
use std::sync::Arc;
pub use types::*;
const OPERATION: &str = "Flow IMS result definitions";
const IMS_GUID: [u8; 16] = [
    0x28, 0x84, 0x2c, 0xcb, 0xaa, 3, 0x13, 0x46, 0xb1, 0x5a, 0x36, 0x1e, 0x44, 0x61, 0xbd, 0x20,
];
fn invalid(cause: String) -> BessemerError {
    table_failure(500, FailureClass::Invalid, cause)
}
impl FlowCarrier {
    /// Reads the complete version-one IMS-definition profile; e.g. `let definitions = carrier.record_500_ims_definitions(&id, options)?`.
    pub fn record_500_ims_definitions(
        &self,
        id: &FlowRecordId,
        options: FlowResultDefinitionOptions,
    ) -> Result<FlowResultDefinitions> {
        read::read(self, id, options).map_err(|error| table_boundary(error, 500))
    }
}
