//! Named fixed-slot roles from specification 14.461–14.462, without inferred geometry (#166).

/// Physical group containing six stored slots, independent of active-count words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPostInfoSlotGroup {
    /// First six cut primitives.
    Section,
    /// Second six cut primitives.
    Iso,
}

/// Known constructor meaning; unknown stored integers have no inferred variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPostInfoPrimitiveKind {
    /// Cylinder constructor parameters.
    Cylinder,
    /// Plane constructor parameters.
    Plane,
    /// Sphere constructor parameters.
    Sphere,
    /// Explicit type-three iso route.
    Iso,
}

/// Installed display token, without inferred world or screen direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPostInfoDirectionToken {
    /// Stored direction minus one.
    Yon,
    /// Stored direction one.
    Hither,
    /// Stored direction two.
    Both,
}

/// Complete named view of one physical slot, retaining all original scalar patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPostInfoCutView {
    /// Physical six-slot group.
    pub group: SimulationPostInfoSlotGroup,
    /// Zero-based physical index within the group, in 0..6.
    pub index: usize,
    /// Original signed primitive-type word.
    pub primitive_code: i32,
    /// Known constructor meaning, absent for unknown codes.
    pub primitive_kind: Option<SimulationPostInfoPrimitiveKind>,
    /// Original signed cut-direction word.
    pub direction_code: i32,
    /// Known literal display token, absent for unknown codes.
    pub direction_token: Option<SimulationPostInfoDirectionToken>,
    /// Three stored normal float32 patterns, without normalization.
    pub normal_bits: [u32; 3],
    /// Three stored center float32 patterns, without an inferred coordinate frame.
    pub center_bits: [u32; 3],
    /// Shared iso/radius scalar pattern; context determines its meaning.
    pub shared_scalar_bits: u32,
    /// Derived D getter pattern: the shared scalar with its sign bit flipped.
    pub negated_d_bits: u32,
}

pub(super) fn primitive_kind(code: i32) -> Option<SimulationPostInfoPrimitiveKind> {
    match code {
        0 => Some(SimulationPostInfoPrimitiveKind::Cylinder),
        1 => Some(SimulationPostInfoPrimitiveKind::Plane),
        2 => Some(SimulationPostInfoPrimitiveKind::Sphere),
        3 => Some(SimulationPostInfoPrimitiveKind::Iso),
        _ => None,
    }
}

pub(super) fn direction_token(code: i32) -> Option<SimulationPostInfoDirectionToken> {
    match code {
        -1 => Some(SimulationPostInfoDirectionToken::Yon),
        1 => Some(SimulationPostInfoDirectionToken::Hither),
        2 => Some(SimulationPostInfoDirectionToken::Both),
        _ => None,
    }
}
