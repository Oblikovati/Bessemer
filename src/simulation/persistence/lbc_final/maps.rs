use super::*;
use crate::archive_codec::ArchiveRevision;

/// Immutable identity of one complete explicit legacy MidsurfMap image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLegacyMapRevision(pub(super) ArchiveRevision);

/// Once-normalized physical group/record/component position tied to its original image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLegacyMapCellId {
    pub(super) revision: SimulationLegacyMapRevision,
    pub(super) group: usize,
    pub(super) record: usize,
    pub(super) component: usize,
    pub(super) offset: usize,
}

impl SimulationLegacyMapCellId {
    /// Exact original bare-stream word offset, e.g. `id.stream_offset()`.
    pub fn stream_offset(&self) -> usize {
        self.offset
    }
    /// Normalized zero-based group/record/component coordinates, e.g. `id.coordinates()`.
    pub fn coordinates(&self) -> (usize, usize, usize) {
        (self.group, self.record, self.component)
    }
}

/// Exactly three ordered groups of four-word records; not a persistent-ID map or native insertion model.
#[derive(Debug, Clone)]
pub struct SimulationLegacyMidsurfMap {
    pub(super) bytes: Arc<[u8]>,
    pub(super) groups: [Arc<[[i32; 4]]>; 3],
    pub(super) offsets: [usize; 3],
    pub(super) revision: SimulationLegacyMapRevision,
    pub(super) maximum_records: usize,
}

impl SimulationLegacyMidsurfMap {
    /// Require the explicit legacy profile and exact EOF, e.g. `SimulationLegacyMidsurfMap::from_bytes(bytes, 10000)?`.
    pub fn from_bytes(bytes: &[u8], maximum_records_per_group: usize) -> Result<Self> {
        let decoded = super::map_codec::decode(bytes, maximum_records_per_group)?;
        Ok(Self {
            bytes: bytes.into(),
            groups: decoded.groups,
            offsets: decoded.offsets,
            revision: SimulationLegacyMapRevision(ArchiveRevision::default()),
            maximum_records: maximum_records_per_group,
        })
    }
    /// Construct matching complete group counts and records, e.g. `SimulationLegacyMidsurfMap::from_groups(&groups, 10000)?`.
    pub fn from_groups(
        groups: &[Vec<[i32; 4]>; 3],
        maximum_records_per_group: usize,
    ) -> Result<Self> {
        Self::from_bytes(
            &super::map_codec::encode(groups, maximum_records_per_group)?,
            maximum_records_per_group,
        )
    }
    /// All groups/records in original physical order without assigned field meanings, e.g. `map.groups()`.
    pub fn groups(&self) -> &[Arc<[[i32; 4]]>; 3] {
        &self.groups
    }
    /// Exact original count-header positions, e.g. `map.count_offsets()`.
    pub fn count_offsets(&self) -> [usize; 3] {
        self.offsets
    }
    /// Exact original signed count header for a normalized group, e.g. `map.count_header(0)?`.
    pub fn count_header(&self, group: usize) -> Result<&[u8]> {
        let offset =
            self.offsets.get(group).copied().ok_or_else(|| {
                invalid("legacy map count group", format!("{group}; expected 0..3"))
            })?;
        Ok(&self.bytes[offset..offset + 4])
    }
    /// Entire exact bare-stream image, e.g. `map.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Immutable original image identity, e.g. `map.revision()`.
    pub fn revision(&self) -> &SimulationLegacyMapRevision {
        &self.revision
    }
    /// Normalize an actual physical word position, e.g. `map.cell_id(0, 0, 3)?`.
    pub fn cell_id(
        &self,
        group: usize,
        record: usize,
        component: usize,
    ) -> Result<SimulationLegacyMapCellId> {
        let selected = self
            .groups
            .get(group)
            .ok_or_else(|| invalid("legacy map group", format!("{group}; expected 0..3")))?;
        require(
            record < selected.len() && component < 4,
            "legacy map cell coordinates",
            format!(
                "group {group}, record {record}, component {component}; expected a record below {} and component 0..4",
                selected.len()
            ),
        )?;
        Ok(SimulationLegacyMapCellId {
            revision: self.revision.clone(),
            group,
            record,
            component,
            offset: self.offsets[group] + 4 + record * 16 + component * 4,
        })
    }
    /// Resolve one normalized word on its original image, e.g. `map.cell(&id)?`.
    pub fn cell(&self, id: &SimulationLegacyMapCellId) -> Result<i32> {
        self.require_cell_revision(id)?;
        self.groups
            .get(id.group)
            .and_then(|group| group.get(id.record))
            .and_then(|record| record.get(id.component))
            .copied()
            .ok_or_else(|| {
                invalid(
                    "legacy map cell coordinates",
                    format!(
                        "{:?}; expected a stored four-word record cell",
                        id.coordinates()
                    ),
                )
            })
    }
    fn require_cell_revision(&self, id: &SimulationLegacyMapCellId) -> Result<()> {
        require(
            id.revision == self.revision,
            "legacy map cell revision",
            format!(
                "foreign revision at {:?}; expected this exact legacy map revision",
                id.coordinates()
            ),
        )
    }
    /// Exact complete legacy group/record scope and exclusions, e.g. `map.coverage()`.
    pub fn coverage(&self) -> SimulationLbcFinalCoverage {
        super::coverage::map_coverage(
            self.bytes.len(),
            self.groups.iter().map(|group| group.len()).sum(),
        )
    }
}
