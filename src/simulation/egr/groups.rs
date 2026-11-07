use super::{
    Result, SimulationEgrCheckMode, SimulationEgrRowImage, SimulationEgrRowView,
    SimulationSolverNameState, require,
};
use std::sync::Arc;

/// Positive signed32 caller ordinal, normalized independently of the stored group word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationEgrLogicalOrdinal(u32);

impl SimulationEgrLogicalOrdinal {
    /// Normalize explicit caller context once, e.g. `SimulationEgrLogicalOrdinal::new(1)?`.
    pub fn new(value: u32) -> Result<Self> {
        require(
            (1..=i32::MAX as u32).contains(&value),
            "EGROUP logical ordinal",
            format!("{value}; expected 1..={}", i32::MAX),
        )?;
        Ok(Self(value))
    }
    /// Exact one-based caller context, e.g. `ordinal.one_based()`.
    pub fn one_based(self) -> u32 {
        self.0
    }
}

/// Completed skip/known output versus an explicitly incomplete native-name state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationEgroupCompletion {
    /// Stored identifier differs from the supplied logical position; no line was requested.
    SkippedIdentifierMismatch,
    /// Complete known-name ASCII line projected, without a native output call.
    ProjectedKnownNameLine,
    /// Parameters and wrapping are known, but missing/null lookup state prevents output.
    IncompleteNameLookupState,
}

/// Explicit narrow EGROUP line projection with every supplied/stored parent retained.
#[derive(Debug, Clone)]
pub struct SimulationEgroupProjection {
    /// Complete stored row and real file parent when known.
    pub source: SimulationEgrRowImage,
    /// Normalized supplied logical ordinal.
    pub logical_ordinal: SimulationEgrLogicalOrdinal,
    /// Original supplied signed group offset.
    pub group_offset: i32,
    /// Completed skip/line or explicitly incomplete name state.
    pub completion: SimulationEgroupCompletion,
    /// Exact scope achieved in the inspected branch.
    pub check_mode: SimulationEgrCheckMode,
    /// Signed wrapping sum when the identifier gate selected the row.
    pub group_number: Option<i32>,
    /// Eight signed words after the documented final-pair swap, without changing stored bytes.
    pub ordered_parameters: Option<[i32; 8]>,
    /// Complete ASCII logical line ending in LF, absent for skipped/incomplete output.
    pub output: Option<Arc<[u8]>>,
    /// Achieved line-pattern error, absent when no line was completed.
    pub requested_pattern_error_bits: Option<u32>,
    /// Always false: native output/format/cache functions did not execute.
    pub native_output_executed: bool,
    /// Always false: this is a conditional line projection, not a complete solver-file schema.
    pub complete_file_schema: bool,
}

impl SimulationEgrRowImage {
    /// Project only an established known-name line, e.g. `row.project_egroup(ordinal, offset)`; unknown/null names remain incomplete.
    pub fn project_egroup(
        &self,
        logical: SimulationEgrLogicalOrdinal,
        offset: i32,
    ) -> SimulationEgroupProjection {
        let view = self.view();
        let mut projection = initial_projection(self, logical, offset);
        if view.group_identifier != logical.one_based() as i32 {
            return projection;
        }
        let number = (logical.one_based() as i32).wrapping_add(offset);
        let mut parameters = view.parameter_words;
        parameters.swap(6, 7);
        projection.group_number = Some(number);
        projection.ordered_parameters = Some(parameters);
        complete_group(&mut projection, &view, number, parameters);
        projection
    }
}

fn initial_projection(
    source: &SimulationEgrRowImage,
    logical: SimulationEgrLogicalOrdinal,
    offset: i32,
) -> SimulationEgroupProjection {
    SimulationEgroupProjection {
        source: source.clone(),
        logical_ordinal: logical,
        group_offset: offset,
        completion: SimulationEgroupCompletion::SkippedIdentifierMismatch,
        check_mode: SimulationEgrCheckMode::ExactConditionalGroupIdentifierGate,
        group_number: None,
        ordered_parameters: None,
        output: None,
        requested_pattern_error_bits: None,
        native_output_executed: false,
        complete_file_schema: false,
    }
}

fn complete_group(
    projection: &mut SimulationEgroupProjection,
    view: &SimulationEgrRowView,
    number: i32,
    parameters: [i32; 8],
) {
    let SimulationSolverNameState::Known(name) = view.name_state else {
        projection.completion = SimulationEgroupCompletion::IncompleteNameLookupState;
        projection.check_mode =
            SimulationEgrCheckMode::ExactConditionalParametersWithIncompleteName;
        return;
    };
    projection.output = Some(group_line(number, name, parameters));
    projection.completion = SimulationEgroupCompletion::ProjectedKnownNameLine;
    projection.check_mode = SimulationEgrCheckMode::ExactConditionalKnownNameLine;
    projection.requested_pattern_error_bits = Some(0);
}

fn group_line(number: i32, name: &str, parameters: [i32; 8]) -> Arc<[u8]> {
    let mut line = format!(" EGROUP, {number} {name} ,");
    for parameter in parameters {
        line.push_str(&format!(" {parameter},"));
    }
    line.push('\n');
    line.into_bytes().into()
}
