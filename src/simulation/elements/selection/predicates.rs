use crate::simulation::SimulationElementRecord;
use std::collections::BTreeSet;

/// Native mask bit pattern; aliases retain their inspected meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElementFamilyMask(u32);

impl SimulationElementFamilyMask {
    /// Preserve all signed/unsigned caller bits, e.g. `SimulationElementFamilyMask::from_pattern((-1i32) as u32)`.
    pub fn from_pattern(pattern: u32) -> Self {
        Self(pattern)
    }

    /// Exact original bits, e.g. `mask.pattern()`; zero has distinct helper/caller behavior.
    pub fn pattern(self) -> u32 {
        self.0
    }
}

/// Exact native postprocessing key domain; CAD/assembly ownership remains unassigned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElementSelectionView {
    /// Zero-extended stored key at offset 28; also named selection_key by the reader.
    pub body_selection_id: u16,
    /// Stored family, distinct from an API engineering enum.
    pub family: u8,
}

impl SimulationElementRecord {
    /// Project native key/family without assigning CAD ownership, e.g. `record.selection()`.
    pub fn selection(&self) -> SimulationElementSelectionView {
        SimulationElementSelectionView {
            body_selection_id: u16::from_le_bytes([self.bytes[28], self.bytes[29]]),
            family: self.bytes[20],
        }
    }

    /// Apply only the named family-mask helper, e.g. `record.valid_for_mask(mask)`; zero rejects.
    pub fn valid_for_mask(&self, mask: SimulationElementFamilyMask) -> bool {
        match self.selection().family {
            4 => mask.0 & 1 != 0,
            8 => mask.0 & 0x12 != 0,
            2 => mask.0 & 4 != 0,
            _ => false,
        }
    }

    /// Apply the caller's scalar path, e.g. `record.solid_averaging_scalar_gate(mask, 0)`; zero mask bypasses the helper.
    pub fn solid_averaging_scalar_gate(&self, mask: SimulationElementFamilyMask, key: i32) -> bool {
        self.has_solid_ten_gate()
            && (mask.0 == 0 || self.valid_for_mask(mask))
            && (key == 0 || i32::from(self.selection().body_selection_id) == key)
    }

    /// Apply zero-scalar, enabled-exclusion-set selection, e.g. `record.solid_averaging_exclusion_gate(mask, &excluded)`; membership rejects.
    pub fn solid_averaging_exclusion_gate(
        &self,
        mask: SimulationElementFamilyMask,
        excluded: &BTreeSet<i32>,
    ) -> bool {
        self.solid_averaging_scalar_gate(mask, 0)
            && !excluded.contains(&i32::from(self.selection().body_selection_id))
    }

    pub(in crate::simulation::elements) fn has_solid_ten_gate(&self) -> bool {
        self.bytes[0] != 0 && self.bytes[20] == 4 && self.bytes[23] == 10
    }
}
