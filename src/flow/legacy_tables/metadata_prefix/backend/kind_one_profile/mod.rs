//! Complete fresh-backend kind-one record framing and isolated validated edits.
mod decode;
mod edit;
mod types;
mod view;
use super::profile_wire::*;
use super::*;
use std::collections::BTreeSet;
pub use types::*;
impl FlowCarrier {
    /// Reads the complete explicit fresh kind-one profile; e.g. `let profile = carrier.record_424_kind_one_profile(&id, options)?`.
    pub fn record_424_kind_one_profile(
        &self,
        id: &FlowRecordId,
        options: FlowMediumProfileOptions,
    ) -> Result<FlowMediumKindOneProfile> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 424))
    }
}
