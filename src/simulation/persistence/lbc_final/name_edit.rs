use super::*;

/// Exact name-frame edit status, independent of native matching/activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcNameEditCompletion {
    /// Every byte already matched; original IDs survived.
    Unchanged,
    /// One complete frame changed on an isolated validated list.
    Changed,
}

/// Exact per-name relation to its original position and supplied replacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcNameRelation {
    /// Entire original frame survived verbatim.
    Copied,
    /// Frame derives from its original position and retained replacement parent.
    Derived,
}

/// One ordered relation for every original name frame.
#[derive(Debug, Clone)]
pub struct SimulationLbcNameHistory {
    /// Original normalized identity.
    pub parent: SimulationLbcNameId,
    /// Result normalized identity.
    pub result: SimulationLbcNameId,
    /// Exact copy or derivation relationship.
    pub relation: SimulationLbcNameRelation,
}

/// Completed isolated name-frame edit retaining complete input parents and history.
#[derive(Debug, Clone)]
pub struct SimulationLbcNameEdit {
    /// Entire original immutable list.
    pub source: SimulationLbcNameOrder,
    /// Complete validated result list.
    pub archive: SimulationLbcNameOrder,
    /// Entire supplied text and its original parent.
    pub replacement: SimulationPersistText,
    /// Requested original position.
    pub parent: SimulationLbcNameId,
    /// Resulting position.
    pub result: SimulationLbcNameId,
    /// Changed versus authentic no-op completion.
    pub completion: SimulationLbcNameEditCompletion,
    /// Complete frame/preservation scope and exclusions.
    pub coverage: SimulationLbcFinalCoverage,
    /// Every original name in total wire order.
    pub history: Arc<[SimulationLbcNameHistory]>,
}

impl SimulationLbcNameOrder {
    /// Replace one normalized frame without changing count/order, e.g. `order.edit_item(&id, &replacement)?`.
    pub fn edit_item(
        &self,
        id: &SimulationLbcNameId,
        replacement: &SimulationPersistText,
    ) -> Result<SimulationLbcNameEdit> {
        let original = self.item(id)?;
        replacement.require_limit(self.maximum_text_bytes)?;
        let archive = self.replaced_name(original, replacement)?;
        let changed = archive.revision != self.revision;
        let history = name_history(self, &archive, id.index, changed)?;
        let coverage = edit_coverage(&archive);
        Ok(SimulationLbcNameEdit {
            source: self.clone(),
            result: archive.item_id(id.index)?,
            archive,
            replacement: replacement.clone(),
            parent: id.clone(),
            completion: if changed {
                SimulationLbcNameEditCompletion::Changed
            } else {
                SimulationLbcNameEditCompletion::Unchanged
            },
            coverage,
            history,
        })
    }

    fn replaced_name(
        &self,
        original: &SimulationPersistText,
        replacement: &SimulationPersistText,
    ) -> Result<Self> {
        if original.encoded() == replacement.encoded() {
            return Ok(self.clone());
        }
        let bytes = super::super::text_sequence::replace_frame(
            &self.bytes,
            original,
            replacement,
            "LBC name edit extent",
        )?;
        let archive = Self::from_bytes(&bytes, self.maximum_items, self.maximum_text_bytes)?;
        verify_name_count(self, &archive)?;
        super::super::text_sequence::verify_frame_preservation(
            &self.bytes,
            &archive.bytes,
            original,
            replacement,
            "LBC name edit preservation",
        )?;
        Ok(archive)
    }
}

fn name_history(
    source: &SimulationLbcNameOrder,
    result: &SimulationLbcNameOrder,
    requested: usize,
    changed: bool,
) -> Result<Arc<[SimulationLbcNameHistory]>> {
    (0..source.items.len())
        .map(|index| {
            Ok(SimulationLbcNameHistory {
                parent: source.item_id(index)?,
                result: result.item_id(index)?,
                relation: if changed && index == requested {
                    SimulationLbcNameRelation::Derived
                } else {
                    SimulationLbcNameRelation::Copied
                },
            })
        })
        .collect()
}

fn edit_coverage(archive: &SimulationLbcNameOrder) -> SimulationLbcFinalCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationLbcFinalCheckMode::ExactNameOrderAndPreservation;
    coverage
}

fn verify_name_count(
    source: &SimulationLbcNameOrder,
    result: &SimulationLbcNameOrder,
) -> Result<()> {
    require(
        result.items.len() == source.items.len(),
        "LBC name edit count",
        format!(
            "{} items; expected {}",
            result.items.len(),
            source.items.len()
        ),
    )
}
