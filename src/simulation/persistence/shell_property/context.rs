use std::sync::Arc;

/// Constructor-bound concrete storage family, sharing the documented Property serializer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationShellPropertyKind {
    /// ShellManager/Shells child Property.
    Shells,
    /// ShellManager/MidSurfShells child Property.
    MidSurfShells,
}

/// External runtime lookup context; file extent never selects a presence profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationShellLayerPresence {
    /// No runtime layer lookup result was supplied.
    Unknown,
    /// Exactly one supplied boolean per declared newest-profile layer.
    Known(Arc<[bool]>),
}

/// Once-normalized concrete family and conditional runtime context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationShellPropertyContext {
    /// Concrete constructor-bound storage kind.
    pub kind: SimulationShellPropertyKind,
    /// Required only for nonempty composite layers at version 891106 and above.
    pub layer_presence: SimulationShellLayerPresence,
}
