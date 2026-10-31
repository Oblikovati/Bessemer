/// Explicit study-header reader/current-writer framing, independent of inferred carrier dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationMeshControlHeaderProfile {
    /// Apply the documented >800027 and >=840013 study-reader gates.
    Reader,
    /// Retain the current writer's unconditional three signed words.
    Writer,
}
