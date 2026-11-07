/// Explicit composite stream/direction framing, never selected by trying alternatives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationShellCompositeProfile {
    /// Counted four-word records under the named reader context.
    FlipTurnReader,
    /// The same counted layout under the named writer context.
    FlipTurnWriter,
    /// Selector-conditioned Extra fields, with a mandatory mode-4 persist ID.
    ExtraReader,
    /// Unconditional four-word/six-double Extra prefix, with optional mode-4 tail.
    ExtraWriter,
}
