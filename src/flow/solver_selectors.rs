use super::*;

impl FlowCarrier {
    /// Selects the last solver ordinary record, rejecting all three control IDs; e.g. `let record = carrier.solver_latest(14)?`.
    pub fn solver_latest(&self, identifier: u32) -> Result<Option<FlowRecord<'_>>> {
        if matches!(identifier, 0xffffff9b..=0xffffff9d) {
            return Err(failure(
                "solver lookup",
                FailureClass::Unsupported,
                format!(
                    "identifier {identifier:#x}; expected an ordinary solver ID outside reserved 0xffffff9b..=0xffffff9d"
                ),
            ));
        }
        self.latest(identifier)
    }
}
