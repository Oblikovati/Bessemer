/// Explicit ShellManager wire grammar, independent of runtime version dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationShellWireProfile {
    /// Apply the historical reader's version gates; e.g. `Self::Reader`.
    Reader,
    /// Apply the installed current writer's unconditional late fields; e.g. `Self::Writer`.
    Writer,
}
