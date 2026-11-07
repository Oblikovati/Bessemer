use super::*;
impl FlowTextSlotState {
    /// Takes ownership of fifty lossless UTF-16 values under an actual-unit bound; e.g. `let state = FlowTextSlotState::from_units(values, 4096)?`.
    pub fn from_units(values: [Vec<u16>; SLOT_COUNT], maximum_units: usize) -> Result<Self> {
        let units = values.iter().try_fold(0, |total, value| {
            admit_total(total, value.len(), maximum_units, "previous text state")
        })?;
        Ok(Self {
            slots: values.map(Arc::<[u16]>::from),
            units,
        })
    }
    /// Borrows a checked complete state value, including embedded NULs; e.g. `let units = state.units(0)?`.
    pub fn units(&self, ordinal: usize) -> Result<&[u16]> {
        self.slots
            .get(ordinal)
            .map(AsRef::as_ref)
            .ok_or_else(|| table_index_error(311, "state slot", ordinal, SLOT_COUNT))
    }
    /// Returns the actual unit total across all fifty state values; e.g. `let count = state.total_units()`.
    pub fn total_units(&self) -> usize {
        self.units
    }
}
