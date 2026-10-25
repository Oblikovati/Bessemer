use crate::archive_codec::StoredDouble;

/// Explicit overload of the native mgXform archive serializer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformEncoding {
    /// Variable restore form with optional matrix and flag-selected perspective.
    Compact,
    /// Fixed 120-byte extraction-operator form, used by plane reference data.
    Fixed,
}

/// The base perspective fields, retained without geometric interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerspectiveBase {
    /// Stored eye point.
    pub eye_point: [StoredDouble; 3],
    /// Stored plane distance.
    pub plane_distance: StoredDouble,
}

/// Flag-selected extension; every floating-point bit remains observable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerspectivePayload {
    /// Neither supported perspective selector is present.
    None,
    /// Selector 0x10.
    Base(PerspectiveBase),
    /// Selector 0x20.
    Extended {
        /// Shared base perspective fields.
        base: PerspectiveBase,
        /// Stored minimum far clip plane distance.
        minimum_far_clip: StoredDouble,
        /// Stored camera field-of-view plane distance.
        camera_fov_plane: StoredDouble,
    },
    /// Selector 0x40.
    Matrix5 {
        /// Stored discarded integer; writer observations do not constrain it to zero.
        discarded: i32,
        /// Twenty-five row-major stored doubles.
        matrix: Box<[StoredDouble; 25]>,
    },
}

/// Owned fields of the variable native restore overload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactTransform {
    /// Raw presence byte; every nonzero value requires a matrix.
    pub has_matrix: u8,
    /// Three stored axis vectors, without identity substitution.
    pub matrix: Option<[[StoredDouble; 3]; 3]>,
    /// Stored translation.
    pub translation: [StoredDouble; 3],
    /// Stored scaling, without range correction.
    pub scaling: StoredDouble,
    /// Raw flags; the high nibble selects the perspective payload.
    pub flags: u8,
    /// Payload corresponding to the raw selector.
    pub perspective: PerspectivePayload,
}

/// Owned fields of the fixed 120-byte extraction overload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedTransform {
    /// Stored row-major matrix.
    pub matrix: [StoredDouble; 9],
    /// Two raw words at structure offset 0x48.
    pub unknown_48: [i32; 2],
    /// Stored translation.
    pub translation: [StoredDouble; 3],
    /// Stored scaling, without native in-memory corrections.
    pub scaling: StoredDouble,
    /// Two raw words at structure offset 0x70.
    pub unknown_70: [i32; 2],
}

/// Complete owned transform fields; the overload cannot be guessed from flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformContents {
    /// Variable native restore encoding.
    Compact(CompactTransform),
    /// Fixed native extraction encoding.
    Fixed(FixedTransform),
}
