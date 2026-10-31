use super::*;

/// Exact legacy word change status, independent of map insertion or application validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegacyMapEditCompletion {
    /// Every byte already matched and original identities survived.
    Unchanged,
    /// One four-byte word changed on an isolated complete validated map.
    Changed,
}

/// Ordered source relation for legacy map counts and record cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegacyMapRelation {
    /// Original complete field survived byte for byte.
    Copied,
    /// Word derives from its original position and retained supplied replacement pattern.
    Derived,
}

/// One relation per physical record word, including every unchanged word.
#[derive(Debug, Clone)]
pub struct SimulationLegacyMapCellHistory {
    /// Original normalized physical cell.
    pub parent: SimulationLegacyMapCellId,
    /// Validated result physical cell.
    pub result: SimulationLegacyMapCellId,
    /// Exact copy or derivation relation.
    pub relation: SimulationLegacyMapRelation,
}

/// Exact preservation of each original group count header.
#[derive(Debug, Clone)]
pub struct SimulationLegacyMapCountHistory {
    /// Original/result zero-based physical group.
    pub group: usize,
    /// Original count byte position.
    pub parent_offset: usize,
    /// Result count byte position.
    pub result_offset: usize,
    /// Original exact signed count bytes.
    pub parent_bytes: [u8; 4],
    /// Result exact signed count bytes.
    pub result_bytes: [u8; 4],
    /// Always Copied for a same-width cell edit.
    pub relation: SimulationLegacyMapRelation,
}

/// Complete isolated cell edit with all source/replacement parents and count/cell history.
#[derive(Debug, Clone)]
pub struct SimulationLegacyMapCellEdit {
    /// Entire original immutable map.
    pub source: SimulationLegacyMidsurfMap,
    /// Complete validated result map.
    pub archive: SimulationLegacyMidsurfMap,
    /// Original requested cell.
    pub parent: SimulationLegacyMapCellId,
    /// Result requested cell.
    pub result: SimulationLegacyMapCellId,
    /// Full supplied four-byte signed pattern.
    pub replacement: i32,
    /// Changed versus authentic no-op completion.
    pub completion: SimulationLegacyMapEditCompletion,
    /// Complete exact frame/preservation checks and exclusions.
    pub coverage: SimulationLbcFinalCoverage,
    /// All three original/result count headers.
    pub count_history: [SimulationLegacyMapCountHistory; 3],
    /// Every record word in total group/record/component order.
    pub cell_history: Arc<[SimulationLegacyMapCellHistory]>,
}

impl SimulationLegacyMidsurfMap {
    /// Replace a normalized raw word without changing counts, e.g. `map.edit_cell(&id, i32::MIN)?`.
    pub fn edit_cell(
        &self,
        id: &SimulationLegacyMapCellId,
        replacement: i32,
    ) -> Result<SimulationLegacyMapCellEdit> {
        let original = self.cell(id)?;
        let archive = self.replaced_cell(id, original, replacement)?;
        let changed = self.revision != archive.revision;
        let count_history = count_history(self, &archive);
        let cell_history = cell_history(self, &archive, id, changed)?;
        let coverage = edit_coverage(&archive);
        Ok(SimulationLegacyMapCellEdit {
            source: self.clone(),
            result: archive.cell_id(id.group, id.record, id.component)?,
            archive,
            parent: id.clone(),
            replacement,
            completion: edit_completion(changed),
            coverage,
            count_history,
            cell_history,
        })
    }

    fn replaced_cell(
        &self,
        id: &SimulationLegacyMapCellId,
        original: i32,
        replacement: i32,
    ) -> Result<Self> {
        if original == replacement {
            return Ok(self.clone());
        }
        let mut bytes = self.bytes.to_vec();
        bytes[id.offset..id.offset + 4].copy_from_slice(&replacement.to_le_bytes());
        let archive = Self::from_bytes(&bytes, self.maximum_records)?;
        verify_preservation(self, &archive, id.offset, replacement)?;
        Ok(archive)
    }
}

fn verify_preservation(
    source: &SimulationLegacyMidsurfMap,
    result: &SimulationLegacyMidsurfMap,
    offset: usize,
    replacement: i32,
) -> Result<()> {
    verify_layout(source, result)?;
    let end = offset + 4;
    require(
        result.bytes[..offset] == source.bytes[..offset]
            && result.bytes[offset..end] == replacement.to_le_bytes()
            && result.bytes[end..] == source.bytes[end..],
        "legacy map edit preservation",
        format!(
            "{} bytes at {offset}..{end}; expected unchanged prefix/suffix and replacement {replacement}",
            result.bytes.len()
        ),
    )
}

fn verify_layout(
    source: &SimulationLegacyMidsurfMap,
    result: &SimulationLegacyMidsurfMap,
) -> Result<()> {
    require(
        source.offsets == result.offsets
            && source.groups.each_ref().map(|group| group.len())
                == result.groups.each_ref().map(|group| group.len()),
        "legacy map edit layout",
        format!(
            "result counts {:?}; expected unchanged count positions and group sizes {:?}",
            result.offsets, source.offsets
        ),
    )
}

fn edit_completion(changed: bool) -> SimulationLegacyMapEditCompletion {
    if changed {
        SimulationLegacyMapEditCompletion::Changed
    } else {
        SimulationLegacyMapEditCompletion::Unchanged
    }
}

fn count_history(
    source: &SimulationLegacyMidsurfMap,
    result: &SimulationLegacyMidsurfMap,
) -> [SimulationLegacyMapCountHistory; 3] {
    std::array::from_fn(|group| {
        let parent_offset = source.offsets[group];
        let result_offset = result.offsets[group];
        SimulationLegacyMapCountHistory {
            group,
            parent_offset,
            result_offset,
            parent_bytes: header_bytes(&source.bytes, parent_offset),
            result_bytes: header_bytes(&result.bytes, result_offset),
            relation: SimulationLegacyMapRelation::Copied,
        }
    })
}

fn header_bytes(bytes: &[u8], offset: usize) -> [u8; 4] {
    [
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]
}

fn cell_history(
    source: &SimulationLegacyMidsurfMap,
    result: &SimulationLegacyMidsurfMap,
    requested: &SimulationLegacyMapCellId,
    changed: bool,
) -> Result<Arc<[SimulationLegacyMapCellHistory]>> {
    let mut history = Vec::new();
    for (group_index, group) in source.groups.iter().enumerate() {
        for record in 0..group.len() {
            append_record_history(
                &mut history,
                source,
                result,
                (group_index, record),
                requested,
                changed,
            )?;
        }
    }
    Ok(history.into())
}

fn append_record_history(
    history: &mut Vec<SimulationLegacyMapCellHistory>,
    source: &SimulationLegacyMidsurfMap,
    result: &SimulationLegacyMidsurfMap,
    position: (usize, usize),
    requested: &SimulationLegacyMapCellId,
    changed: bool,
) -> Result<()> {
    for component in 0..4 {
        let parent = source.cell_id(position.0, position.1, component)?;
        let relation = if changed && &parent == requested {
            SimulationLegacyMapRelation::Derived
        } else {
            SimulationLegacyMapRelation::Copied
        };
        history.push(SimulationLegacyMapCellHistory {
            parent,
            result: result.cell_id(position.0, position.1, component)?,
            relation,
        });
    }
    Ok(())
}

fn edit_coverage(archive: &SimulationLegacyMidsurfMap) -> SimulationLbcFinalCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationLbcFinalCheckMode::ExactLegacyMapAndPreservation;
    coverage
}
