mod edit;
mod proposal;
use super::{
    Result, SimulationBodySelectionId, SimulationGenArchive, SimulationGenRevision, invalid,
    require,
};
use crate::archive_codec::ArchiveRevision;
pub use edit::*;
pub use proposal::*;
use std::sync::Arc;

/// Normalized stored auxiliary row profile, distinct from its runtime workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryBodyProfile {
    /// Original 284-byte row with bounds at 12 and 16.
    Stored284,
    /// Original 288-byte row with bounds at 16 and 20.
    Stored288,
}

impl SimulationAuxiliaryBodyProfile {
    /// Normalize a stored stride once, e.g. `SimulationAuxiliaryBodyProfile::from_stride(284)?`.
    pub fn from_stride(stride: usize) -> Result<Self> {
        match stride {
            284 => Ok(Self::Stored284),
            288 => Ok(Self::Stored288),
            _ => Err(invalid(
                "auxiliary body profile",
                format!("stride {stride}; expected 284 or 288"),
            )),
        }
    }

    /// Exact stored width, e.g. `profile.stride()`; no normalization rewrites bytes.
    pub fn stride(self) -> usize {
        match self {
            Self::Stored284 => 284,
            Self::Stored288 => 288,
        }
    }

    fn bounds_offset(self) -> usize {
        match self {
            Self::Stored284 => 12,
            Self::Stored288 => 16,
        }
    }
}

/// Identity of one complete immutable auxiliary-row snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationAuxiliaryBodyRevision(ArchiveRevision);

/// The three bound fields and exact mathematical descriptor decrements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxiliaryBodyFields {
    /// Full identifier bits, also retaining the existing vector-selection role.
    pub body_id: SimulationBodySelectionId,
    /// Stored signed first ordinal, one-based inclusive in installed intervals.
    pub stored_low: i32,
    /// Stored signed last ordinal, one-based inclusive in installed intervals.
    pub stored_high: i32,
    /// Mathematical L-1, before the proposal's checked native-i32 boundary.
    pub runtime_start: i64,
    /// Mathematical H-1, without adjusting differing consumer endpoints.
    pub runtime_upper: i64,
    /// Zero/zero is skipped by the inspected conditional builder.
    pub descriptor_appended: bool,
}

/// Complete stored row with only three interpreted fields and all remaining bytes retained.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryBodyRow {
    bytes: Arc<[u8]>,
    profile: SimulationAuxiliaryBodyProfile,
    revision: SimulationAuxiliaryBodyRevision,
    gen_parent: Option<(SimulationGenRevision, usize)>,
}

impl SimulationAuxiliaryBodyRow {
    /// Own one complete stored profile, e.g. `SimulationAuxiliaryBodyRow::from_bytes(raw, profile)?`.
    pub fn from_bytes(bytes: &[u8], profile: SimulationAuxiliaryBodyProfile) -> Result<Self> {
        require(
            bytes.len() == profile.stride(),
            "auxiliary body row",
            format!(
                "{} bytes for {profile:?}; expected exactly {}",
                bytes.len(),
                profile.stride()
            ),
        )?;
        Ok(Self {
            bytes: bytes.into(),
            profile,
            revision: SimulationAuxiliaryBodyRevision(ArchiveRevision::default()),
            gen_parent: None,
        })
    }

    /// Complete exact stored bytes, e.g. `row.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    /// Validated stored profile, e.g. `row.profile()`.
    pub fn profile(&self) -> SimulationAuxiliaryBodyProfile {
        self.profile
    }

    /// Immutable snapshot identity, e.g. `row.revision()`.
    pub fn revision(&self) -> &SimulationAuxiliaryBodyRevision {
        &self.revision
    }

    /// Original paired GEN and physical row copy parent, e.g. `row.gen_parent()`; absent for loose bytes.
    pub fn gen_parent(&self) -> Option<(&SimulationGenRevision, usize)> {
        self.gen_parent
            .as_ref()
            .map(|(revision, index)| (revision, *index))
    }

    /// Project only the documented three fields, e.g. `row.fields()`; wider arithmetic preserves overflow evidence.
    pub fn fields(&self) -> SimulationAuxiliaryBodyFields {
        let offset = self.profile.bounds_offset();
        let low = read_pattern(&self.bytes, offset) as i32;
        let high = read_pattern(&self.bytes, offset + 4) as i32;
        SimulationAuxiliaryBodyFields {
            body_id: SimulationBodySelectionId::from_pattern(read_pattern(&self.bytes, 0)),
            stored_low: low,
            stored_high: high,
            runtime_start: i64::from(low) - 1,
            runtime_upper: i64::from(high) - 1,
            descriptor_appended: low != 0 || high != 0,
        }
    }
}

impl SimulationGenArchive {
    /// Snapshot a complete stored auxiliary row with its actual GEN copy parent, e.g. `image.auxiliary_body_row(0)?`.
    pub fn auxiliary_body_row(&self, index: usize) -> Result<SimulationAuxiliaryBodyRow> {
        let profile = SimulationAuxiliaryBodyProfile::from_stride(self.layout().auxiliary_stride)?;
        let rows = self.block(7)?.chunks_exact(profile.stride());
        let count = rows.len();
        let raw = rows.into_iter().nth(index).ok_or_else(|| {
            invalid(
                "GEN auxiliary body row",
                format!("index {index}; expected less than {count}"),
            )
        })?;
        let mut row = SimulationAuxiliaryBodyRow::from_bytes(raw, profile)?;
        row.gen_parent = Some((self.revision().clone(), index));
        Ok(row)
    }
}

fn read_pattern(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}
