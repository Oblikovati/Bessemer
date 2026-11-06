//! Complete fixed owner-state fields; selection precedes parsing and never falls back on failure.
mod edit;
mod projection;
mod read;
mod types;
mod view;
use super::*;
pub use types::*;
impl FlowCarrier {
    /// Reads all legacy words without header dispatch; e.g. `let state = carrier.record_122_owner_state(&id, options)?`.
    pub fn record_122_owner_state(
        &self,
        id: &FlowRecordId,
        options: FlowOwnerStateOptions,
    ) -> Result<FlowOwnerState> {
        read::read(self, id, FlowOwnerStateKind::Legacy122, options)
    }
    /// Reads complete modern members and state byte; e.g. `let state = carrier.record_434_owner_state(&id, options)?`.
    pub fn record_434_owner_state(
        &self,
        id: &FlowRecordId,
        options: FlowOwnerStateOptions,
    ) -> Result<FlowOwnerState> {
        read::read(self, id, FlowOwnerStateKind::Modern434, options)
    }
    /// Selects latest 434 before latest 122, preserving selected parse failures; e.g. `let state = carrier.owner_state(options)?`.
    pub fn owner_state(&self, options: FlowOwnerStateOptions) -> Result<Option<FlowOwnerState>> {
        if let Some(record) = self.latest(434)? {
            return self.record_434_owner_state(&record.id, options).map(Some);
        }
        self.latest(122)?
            .map(|record| self.record_122_owner_state(&record.id, options))
            .transpose()
    }
}
