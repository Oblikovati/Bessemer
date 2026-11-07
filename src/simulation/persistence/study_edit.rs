use super::{
    Result, SimulationPersistCheckMode, SimulationPersistCoverage, SimulationPersistText,
    SimulationStudyOrder, SimulationStudyTextId, require,
};
use std::sync::Arc;

/// Whether an exact study-order edit changed any bytes or preserved the original identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStudyEditCompletion {
    /// Every byte was already identical and original identities survived.
    Unchanged,
    /// A complete isolated result passed framing and preservation checks.
    Changed,
}

/// Exact relationship of one resulting text to its source parents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStudyTextRelation {
    /// Complete original frame survived byte for byte.
    Copied,
    /// Frame derives from its original position and the retained supplied replacement.
    Derived,
}

/// Complete ordered per-text history, including the leading text and unchanged items.
#[derive(Debug, Clone)]
pub struct SimulationStudyTextHistory {
    /// Original revision-bound position.
    pub parent: SimulationStudyTextId,
    /// Result revision-bound position.
    pub result: SimulationStudyTextId,
    /// Exact copy or derivation relation.
    pub relation: SimulationStudyTextRelation,
}

/// Completed isolated edit retaining every input, exact coverage and all text relations.
#[derive(Debug, Clone)]
pub struct SimulationStudyTextEdit {
    /// Complete immutable source image.
    pub source: SimulationStudyOrder,
    /// Complete validated result image.
    pub archive: SimulationStudyOrder,
    /// Entire supplied replacement frame with its own original parent.
    pub replacement: SimulationPersistText,
    /// Original requested position.
    pub parent: SimulationStudyTextId,
    /// Resulting requested position.
    pub result: SimulationStudyTextId,
    /// Changed versus authentic byte no-op.
    pub completion: SimulationStudyEditCompletion,
    /// Exact complete framing and unchanged-byte checks with exclusions.
    pub coverage: SimulationPersistCoverage,
    /// Ordered relation for every original text.
    pub history: Arc<[SimulationStudyTextHistory]>,
}

impl SimulationStudyOrder {
    /// Replace one normalized text without changing item count, e.g. `order.edit_text(&id, &replacement)?`.
    pub fn edit_text(
        &self,
        id: &SimulationStudyTextId,
        replacement: &SimulationPersistText,
    ) -> Result<SimulationStudyTextEdit> {
        let original = self.text(id)?;
        replacement.require_limit(self.maximum_text_bytes)?;
        let archive = self.replace_text(original, replacement)?;
        let changed = archive.revision != self.revision;
        let history = edit_history(self, &archive, id.ordinal, changed);
        let coverage = edit_coverage(&archive);
        Ok(SimulationStudyTextEdit {
            source: self.clone(),
            result: archive.text_id(id.ordinal),
            archive,
            replacement: replacement.clone(),
            parent: id.clone(),
            completion: if changed {
                SimulationStudyEditCompletion::Changed
            } else {
                SimulationStudyEditCompletion::Unchanged
            },
            coverage,
            history,
        })
    }

    fn replace_text(
        &self,
        original: &SimulationPersistText,
        replacement: &SimulationPersistText,
    ) -> Result<Self> {
        if original.encoded() == replacement.encoded() {
            return Ok(self.clone());
        }
        let bytes = super::text_sequence::replace_frame(
            &self.bytes,
            original,
            replacement,
            "study text edit extent",
        )?;
        let archive = Self::from_bytes(&bytes, self.maximum_items, self.maximum_text_bytes)?;
        verify_unchanged(self, &archive, original, replacement)?;
        Ok(archive)
    }
}

fn verify_unchanged(
    source: &SimulationStudyOrder,
    result: &SimulationStudyOrder,
    original: &SimulationPersistText,
    replacement: &SimulationPersistText,
) -> Result<()> {
    verify_text_count(source, result)?;
    super::text_sequence::verify_frame_preservation(
        &source.bytes,
        &result.bytes,
        original,
        replacement,
        "study edit preservation",
    )
}

fn edit_coverage(archive: &SimulationStudyOrder) -> SimulationPersistCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationPersistCheckMode::ExactStudyOrderAndPreservation;
    coverage
}

fn verify_text_count(source: &SimulationStudyOrder, result: &SimulationStudyOrder) -> Result<()> {
    require(
        result.texts.len() == source.texts.len(),
        "study edit text count",
        format!(
            "{} texts; expected {}",
            result.texts.len(),
            source.texts.len()
        ),
    )
}

fn edit_history(
    source: &SimulationStudyOrder,
    result: &SimulationStudyOrder,
    ordinal: usize,
    changed: bool,
) -> Arc<[SimulationStudyTextHistory]> {
    (0..source.texts.len())
        .map(|index| SimulationStudyTextHistory {
            parent: source.text_id(index),
            result: result.text_id(index),
            relation: if changed && index == ordinal {
                SimulationStudyTextRelation::Derived
            } else {
                SimulationStudyTextRelation::Copied
            },
        })
        .collect()
}
