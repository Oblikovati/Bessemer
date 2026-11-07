use super::{Result, SimulationGenArchive, SimulationGenRevision, SimulationNodeId};
use std::sync::Arc;

/// Contiguous signed-tag diagnostic with real source node identities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationNodeGroupRun {
    /// Signed stored word, including zero and negative patterns.
    pub tag: i16,
    /// One-based first ordinal in complete stored record order.
    pub first_ordinal: usize,
    /// Inclusive one-based last ordinal in complete stored record order.
    pub last_ordinal: usize,
    /// First record bound to the retained GEN revision.
    pub first_node: SimulationNodeId,
    /// Last record bound to the same GEN revision.
    pub last_node: SimulationNodeId,
}

/// Exact stored-tag scope; does not emulate the element-driven native producer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationNodeGroupCheckMode {
    /// Every signed tag in every complete stored node, including inactive nodes.
    ExactAllStoredTagsAndContiguousRuns,
}

/// A successful empty projection remains distinct from failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationNodeGroupCompletion {
    /// Complete GEN image contains no node records.
    CompletedEmpty,
    /// All stored nodes were partitioned into diagnostic runs.
    CompletedRuns,
}

/// Responsibilities excluded from signed tag diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationNodeGroupDiagnostic {
    /// Tags do not prevent other bodies' elements from referencing these nodes.
    ExclusiveBodyOwnershipUnassigned,
    /// Native producer traversal depends on additional element/cursor context.
    NativeElementDrivenTraversalNotExecuted,
}

/// Complete read projection retaining its exact source and derived record ranges.
#[derive(Debug, Clone)]
pub struct SimulationNodeGroupProjection {
    /// Immutable complete paired image, including unassigned node bytes.
    pub source: SimulationGenArchive,
    /// Identity shared by every first/last node in the derived ranges.
    pub parent: SimulationGenRevision,
    /// Empty versus populated successful completion.
    pub completion: SimulationNodeGroupCompletion,
    /// Separate repeated runs remain separate in stored order.
    pub runs: Arc<[SimulationNodeGroupRun]>,
    /// Exact diagnostic check scope, without geometry acceptance claims.
    pub check_mode: SimulationNodeGroupCheckMode,
    /// All nodes visited, including inactive nodes.
    pub checked_nodes: usize,
    /// Two signed-tag bytes per visited node.
    pub checked_tag_bytes: usize,
    /// Achieved stored tag-pattern error, zero.
    pub tag_pattern_error_bits: u32,
    /// Ownership and native traversal limits.
    pub diagnostics: [SimulationNodeGroupDiagnostic; 2],
}

impl SimulationGenArchive {
    /// Read contiguous signed-tag diagnostics, e.g. `gen.node_group_runs()?`; shared references remain valid.
    pub fn node_group_runs(&self) -> Result<SimulationNodeGroupProjection> {
        let nodes = self.block(5)?;
        let mut runs = Vec::new();
        for (index, record) in nodes.as_chunks::<52>().0.iter().enumerate() {
            append_run(
                self,
                &mut runs,
                index,
                i16::from_le_bytes([record[22], record[23]]),
            )?;
        }
        Ok(group_projection(self, runs, nodes.len() / 52))
    }
}

fn append_run(
    source: &SimulationGenArchive,
    runs: &mut Vec<SimulationNodeGroupRun>,
    index: usize,
    tag: i16,
) -> Result<()> {
    let node = source.node_id(index)?;
    if let Some(last) = runs.last_mut().filter(|last| last.tag == tag) {
        last.last_ordinal = index + 1;
        last.last_node = node;
        return Ok(());
    }
    runs.push(SimulationNodeGroupRun {
        tag,
        first_ordinal: index + 1,
        last_ordinal: index + 1,
        first_node: node.clone(),
        last_node: node,
    });
    Ok(())
}

fn group_projection(
    source: &SimulationGenArchive,
    runs: Vec<SimulationNodeGroupRun>,
    checked_nodes: usize,
) -> SimulationNodeGroupProjection {
    SimulationNodeGroupProjection {
        source: source.clone(),
        parent: source.revision.clone(),
        completion: group_completion(runs.is_empty()),
        runs: runs.into(),
        check_mode: SimulationNodeGroupCheckMode::ExactAllStoredTagsAndContiguousRuns,
        checked_nodes,
        checked_tag_bytes: checked_nodes * 2,
        tag_pattern_error_bits: 0,
        diagnostics: [
            SimulationNodeGroupDiagnostic::ExclusiveBodyOwnershipUnassigned,
            SimulationNodeGroupDiagnostic::NativeElementDrivenTraversalNotExecuted,
        ],
    }
}

fn group_completion(empty: bool) -> SimulationNodeGroupCompletion {
    if empty {
        return SimulationNodeGroupCompletion::CompletedEmpty;
    }
    SimulationNodeGroupCompletion::CompletedRuns
}
