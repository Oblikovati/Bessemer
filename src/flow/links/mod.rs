//! Explicitly ordered, caller-classified GUID lookup; no filesystem discovery.
mod select;
mod types;
mod view;
use super::*;
pub use types::*;

impl FlowCarrier {
    /// Reads a non-nil fresh-result lookup key; e.g. `let key = carrier.lookup_guid(FlowResultType::Cpt)?`.
    pub fn lookup_guid(&self, kind: FlowResultType) -> Result<Option<FlowGuid>> {
        Ok(lookup_keys(self)?[kind.key_index()])
    }
    /// Projects related-file GUID selection; e.g. `let related = carrier.related_carriers(&candidates)?`.
    pub fn related_carriers(
        &self,
        candidates: &[FlowLinkCandidate],
    ) -> Result<FlowRelatedCarriers> {
        select::related(self, Arc::from(candidates))
    }
}

/// Searches the supplied order exactly; e.g. `let found = find_flow_carrier_by_guid(&candidates, kind, guid)?`.
pub fn find_flow_carrier_by_guid(
    candidates: &[FlowLinkCandidate],
    kind: FlowResultType,
    query: FlowGuid,
) -> Result<FlowGuidLookup> {
    select::find(Arc::from(candidates), kind, query)
}

fn lookup_keys(carrier: &FlowCarrier) -> Result<[Option<FlowGuid>; 3]> {
    let Some(header) = carrier.header_metadata()? else {
        return Ok([None; 3]);
    };
    Ok([
        non_nil(header.second_guid),
        non_nil(header.first_guid),
        header
            .settings
            .and_then(|settings| non_nil(settings.cfld_uuid)),
    ])
}

fn non_nil(guid: FlowGuid) -> Option<FlowGuid> {
    (guid.as_bytes() != &[0; 16]).then_some(guid)
}
