use super::{Result, invalid};
use std::sync::Arc;

const SOLVER_NAMES: [(i32, &str); 33] = [
    (1, "TRUSS2D"),
    (8, "TRUSS3D"),
    (3, "BEAM2D"),
    (4, "BEAM3D"),
    (9, "PIPE"),
    (29, "ELBOW"),
    (14, "BOUND"),
    (18, "SPRING"),
    (21, "MASS"),
    (27, "GENSTIF"),
    (42, "PLANE2D"),
    (11, "SHELLAX"),
    (43, "SHELL3T"),
    (44, "SHELL4T"),
    (53, "SHELL3L"),
    (54, "SHELL4L"),
    (63, "SHELL3"),
    (46, "SOLIDL"),
    (31, "RLINK"),
    (34, "CLINK"),
    (28, "RBAR"),
    (36, "HLINK"),
    (37, "GAP"),
    (98, "TETRA4"),
    (45, "SOLID"),
    (2, "TRIANG"),
    (91, "SHELL9L"),
    (93, "SHELL9"),
    (90, "SHELL6T"),
    (64, "SHELL4"),
    (92, "TETRA10"),
    (99, "SHELL6L"),
    (94, "SHELL6"),
];

/// Signed native lookup value, distinct from an inferred GEN element field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationSolverElementCode(i32);

/// Complete installed lookup state, without executing its unchecked native copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSolverNameState {
    /// A complete known ASCII name, without its terminal NUL.
    Known(&'static str),
    /// Code zero matches a null name source; no successful native copy is predicted.
    MatchedNullSource,
    /// A nonzero unmatched code leaves the supplied destination unchanged.
    UnmatchedNoWrite,
}

/// Completion of the explicit memory projection, separate from native execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSolverNameCompletion {
    /// Known prefix and NUL projected into a complete preserved destination.
    ProjectedKnownCopy,
    /// Complete destination retained with no writes for an unmatched code.
    ProjectedUnmatchedNoWrite,
    /// Lookup resolved a null source, but native copy/output remains incomplete.
    IncompleteMatchedNullSource,
}

/// Exact check scope differs when the native null-source copy cannot be completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSolverNameCheckMode {
    /// Known name, terminal NUL and every preserved destination suffix byte.
    ExactNamePrefixAndRetainedDestination,
    /// Every supplied destination byte remains unchanged.
    ExactUnmatchedDestinationUnchanged,
    /// Exact lookup state only; no output or native return is fabricated.
    ExactLookupOnlyNativeCopyIncomplete,
}

/// Owned conditional name projection retaining both the code and complete destination parent.
#[derive(Debug, Clone)]
pub struct SimulationSolverNameProjection {
    /// Typed input code.
    pub code: SimulationSolverElementCode,
    /// Complete immutable supplied destination parent.
    pub destination: Arc<[u8]>,
    /// Exact lookup state, including the distinct matched-null-source case.
    pub lookup: SimulationSolverNameState,
    /// Complete projected destination, absent for the unresolved null-source copy.
    pub output: Option<Arc<[u8]>>,
    /// Completion versus explicitly incomplete native-copy state.
    pub completion: SimulationSolverNameCompletion,
    /// Exact scope actually checked.
    pub check_mode: SimulationSolverNameCheckMode,
    /// Projected native return, absent when null-source execution is unresolved.
    pub native_return: Option<i32>,
    /// Known prefix bytes including the terminal NUL, or zero.
    pub copied_prefix_bytes: usize,
    /// Checked preserved suffix/full-destination bytes, absent with no completed output.
    pub preserved_destination_bytes: Option<usize>,
    /// Achieved requested byte-pattern error, absent for incomplete output.
    pub requested_pattern_error_bits: Option<u32>,
    /// Always false: this helper does not assign a general file or GEN field binding.
    pub wire_field_binding: bool,
    /// Always false: no native code, pointer read or unchecked copy executes.
    pub native_execution: bool,
}

impl SimulationSolverElementCode {
    /// Capture one signed32 code, e.g. `SimulationSolverElementCode::from_signed(98)`.
    pub fn from_signed(code: i32) -> Self {
        Self(code)
    }
    /// Exact original signed value, e.g. `code.signed()`.
    pub fn signed(self) -> i32 {
        self.0
    }
    /// Inspect all three lookup states, e.g. `code.lookup()`; zero is not an ordinary unmatched code.
    pub fn lookup(self) -> SimulationSolverNameState {
        if self.0 == 0 {
            return SimulationSolverNameState::MatchedNullSource;
        }
        SOLVER_NAMES
            .iter()
            .find(|(code, _)| *code == self.0)
            .map_or(SimulationSolverNameState::UnmatchedNoWrite, |(_, name)| {
                SimulationSolverNameState::Known(name)
            })
    }
    /// Project into an owned immutable destination, e.g. `code.project_destination(buffer)?`; no native call executes.
    pub fn project_destination(
        self,
        destination: Arc<[u8]>,
    ) -> Result<SimulationSolverNameProjection> {
        let lookup = self.lookup();
        let (output, copied) = projected_output(lookup, &destination)?;
        Ok(name_result(self, destination, lookup, output, copied))
    }
}

fn projected_output(
    lookup: SimulationSolverNameState,
    destination: &Arc<[u8]>,
) -> Result<(Option<Arc<[u8]>>, usize)> {
    match lookup {
        SimulationSolverNameState::MatchedNullSource => Ok((None, 0)),
        SimulationSolverNameState::UnmatchedNoWrite => Ok((Some(destination.clone()), 0)),
        SimulationSolverNameState::Known(name) => {
            project_known(name, destination).map(|bytes| (Some(bytes), name.len() + 1))
        }
    }
}

fn project_known(name: &str, destination: &Arc<[u8]>) -> Result<Arc<[u8]>> {
    let length = name.len() + 1;
    if destination.len() < length {
        return Err(invalid(
            "solver name destination",
            format!(
                "{} bytes for {name:?}; expected at least {length} including NUL, without assuming a native size check",
                destination.len()
            ),
        ));
    }
    if destination[..name.len()] == *name.as_bytes() && destination[name.len()] == 0 {
        return Ok(destination.clone());
    }
    let mut bytes = destination.to_vec();
    bytes[..name.len()].copy_from_slice(name.as_bytes());
    bytes[name.len()] = 0;
    Ok(bytes.into())
}

fn name_result(
    code: SimulationSolverElementCode,
    destination: Arc<[u8]>,
    lookup: SimulationSolverNameState,
    output: Option<Arc<[u8]>>,
    copied: usize,
) -> SimulationSolverNameProjection {
    let completed = output.is_some();
    let (completion, check_mode) = name_completion(lookup);
    SimulationSolverNameProjection {
        code,
        preserved_destination_bytes: completed.then_some(destination.len() - copied),
        destination,
        lookup,
        output,
        completion,
        check_mode,
        native_return: completed.then_some(0),
        copied_prefix_bytes: copied,
        requested_pattern_error_bits: completed.then_some(0),
        wire_field_binding: false,
        native_execution: false,
    }
}

fn name_completion(
    lookup: SimulationSolverNameState,
) -> (
    SimulationSolverNameCompletion,
    SimulationSolverNameCheckMode,
) {
    use SimulationSolverNameCheckMode as Mode;
    use SimulationSolverNameCompletion as Completion;
    match lookup {
        SimulationSolverNameState::Known(_) => (
            Completion::ProjectedKnownCopy,
            Mode::ExactNamePrefixAndRetainedDestination,
        ),
        SimulationSolverNameState::MatchedNullSource => (
            Completion::IncompleteMatchedNullSource,
            Mode::ExactLookupOnlyNativeCopyIncomplete,
        ),
        SimulationSolverNameState::UnmatchedNoWrite => (
            Completion::ProjectedUnmatchedNoWrite,
            Mode::ExactUnmatchedDestinationUnchanged,
        ),
    }
}
