use super::*;
use std::sync::Arc;

/// The exact path that completed; a -1 flag is not by itself a failure status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationBodyTypeCompletion {
    /// A negative descriptor bound returned before callback/traversal.
    GuardedNegativeBounds,
    /// The supplied callback result selected flag sixteen before any range checks.
    CompletedCallbackOverride,
    /// The fixed-mode table completed without traversal, including an unnamed -1.
    CompletedFixedMode,
    /// The exact supplied scan completed without an active association.
    CompletedNoAssociation,
    /// The first active association completed the scan, even for an unnamed family.
    CompletedFirstAssociation,
}

/// Explicit exact check scope; unvisited references remain outside its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationBodyTypeCheckMode {
    /// Only the signed negative-bound guard applied.
    NegativeBoundsOnly,
    /// Only explicit callback/type dispatch applied after the bounds guard.
    SuppliedCallbackAndModeOnly,
    /// Exclusive element traversal stopped at its first active node association or endpoint.
    ExactExclusiveScanUntilFirstActiveAssociation,
}

/// All source parents of the first successful association.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationBodyTypeAssociation {
    /// Zero-based element index in the result's source revision.
    pub element_index: usize,
    /// Zero-based selected node-reference slot.
    pub reference_slot: usize,
    /// Normalized zero-based node slot in the same source revision.
    pub node_slot: usize,
}

/// Complete exact selected-query coverage, without whole-mesh or geometry validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationBodyTypeCoverage {
    /// Declared exact checked path.
    pub mode: SimulationBodyTypeCheckMode,
    /// Elements passing activity/body selection before early termination.
    pub selected_elements: usize,
    /// Every selected reference checked up to termination, including zero/inactive IDs.
    pub visited_references: usize,
}

/// Immutable body-type return, exact visit history and first-association parents.
#[derive(Debug, Clone)]
pub struct SimulationBodyTypeResult {
    /// Owned complete source blocks/revision and optional paired GEN parent.
    pub source: SimulationBodyQueryMesh,
    /// Exact three supplied runtime inputs; no full descriptor was decoded.
    pub inputs: SimulationBodyTypeInputs,
    /// Original signed query mode; mode five differs from the node-mark query.
    pub query_mode: i32,
    /// Actual numeric native return flag, without API enum conversion.
    pub type_flag: i32,
    /// Guard, fixed dispatch, empty scan or first-association completion.
    pub completion: SimulationBodyTypeCompletion,
    /// Every visited element in exact order, including rejected selection gates.
    pub visited_elements: Arc<[usize]>,
    /// First successful node/element/reference parents, if one was reached.
    pub first_association: Option<SimulationBodyTypeAssociation>,
    /// Exact early-stop/dispatch coverage.
    pub coverage: SimulationBodyTypeCoverage,
    /// Callback's initial low-word signed pattern, absent when negative bounds bypass it.
    pub callback_initial_i16: Option<i16>,
    /// Applied supplied callback result; no callback was executed.
    pub supplied_callback_result: Option<i16>,
    /// Always false: these inputs do not establish full descriptor serialization.
    pub file_descriptor_binding: bool,
    /// Achieved numeric flag-pattern error, zero; no physical-error claim.
    pub flag_pattern_error_bits: u32,
    /// Ownership, callback, runtime and geometry exclusions.
    pub diagnostics: [SimulationBodyQueryDiagnostic; 4],
}

pub(super) fn early_result(
    mesh: &SimulationBodyQueryMesh,
    inputs: SimulationBodyTypeInputs,
    query_mode: i32,
    callback: Option<i16>,
    type_flag: i32,
    completion: SimulationBodyTypeCompletion,
) -> SimulationBodyTypeResult {
    SimulationBodyTypeResult {
        source: mesh.clone(),
        inputs,
        query_mode,
        type_flag,
        completion,
        visited_elements: [].into(),
        first_association: None,
        coverage: early_coverage(callback),
        callback_initial_i16: callback.map(|_| inputs.body_id.pattern() as u16 as i16),
        supplied_callback_result: callback,
        file_descriptor_binding: false,
        flag_pattern_error_bits: 0,
        diagnostics: super::super::projection::query_diagnostics(),
    }
}

fn early_coverage(callback: Option<i16>) -> SimulationBodyTypeCoverage {
    let mode = if callback.is_none() {
        SimulationBodyTypeCheckMode::NegativeBoundsOnly
    } else {
        SimulationBodyTypeCheckMode::SuppliedCallbackAndModeOnly
    };
    SimulationBodyTypeCoverage {
        mode,
        selected_elements: 0,
        visited_references: 0,
    }
}
