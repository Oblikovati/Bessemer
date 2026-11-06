//! Exact fresh mixed-kind medium records with isolated whole-carrier edits.
mod decode;
mod edit;
mod entries;
mod scan;
mod types;
mod view;
use super::profile_wire::{self, *};
use super::*;
use std::collections::BTreeSet;
pub use types::*;
impl FlowCarrier {
    /// Reads complete fresh mixed-kind framing; e.g. `let profile=carrier.record_424_mixed_profile(&id,options)?`.
    pub fn record_424_mixed_profile(
        &self,
        id: &FlowRecordId,
        options: FlowMediumMixedOptions,
    ) -> Result<FlowMediumMixedProfile> {
        decode::read(self, id, options).map_err(|error| table_boundary(error, 424))
    }
}
