use super::SimulationPersistVersion;

/// Explicit direct manager context; Beam requires a version, Mass never infers one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationManagerPropertyProfile {
    /// Version-selected BeamManager/Property framing.
    Beam(SimulationPersistVersion),
    /// Fixed MassElementManager/Property framing retaining optional supplied context.
    Mass(Option<SimulationPersistVersion>),
}

/// Documented runtime destination, independent of its physical stream offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationManagerPropertyMember {
    /// First Beam word, without assigned business meaning.
    Beam14C,
    /// Second Beam word from version 810024.
    Beam168,
    /// Third Beam word from version 820006.
    Beam148,
    /// Sole Mass word, without object-count semantics.
    Mass28,
}

impl SimulationManagerPropertyMember {
    /// Exact documented owner offset, e.g. `SimulationManagerPropertyMember::Beam168.offset()`.
    pub fn offset(self) -> usize {
        match self {
            Self::Beam14C => 0x14c,
            Self::Beam168 => 0x168,
            Self::Beam148 => 0x148,
            Self::Mass28 => 0x28,
        }
    }
}

impl SimulationManagerPropertyProfile {
    pub(super) fn members(self) -> &'static [SimulationManagerPropertyMember] {
        use SimulationManagerPropertyMember::*;
        match self {
            Self::Mass(_) => &[Mass28],
            Self::Beam(version) if version.value() < 810024 => &[Beam14C],
            Self::Beam(version) if version.value() < 820006 => &[Beam14C, Beam168],
            Self::Beam(_) => &[Beam14C, Beam168, Beam148],
        }
    }
}
