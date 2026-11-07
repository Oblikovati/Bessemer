use super::*;
use crate::simulation::SimulationBodyTypeInputs;

/// Exact field binding scope; all other row/descriptor fields remain unassigned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryBodyCheckMode {
    /// Complete stored framing/raw retention and the three exact documented field bindings.
    ExactCompleteRowAndThreeFieldBinding,
}

/// Exact field binding scope; all other row/descriptor fields remain unassigned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxiliaryBodyCoverage {
    /// Declared exact byte/binding scope.
    pub mode: SimulationAuxiliaryBodyCheckMode,
    /// Complete retained stored row bytes, 284 or 288.
    pub stored_bytes: usize,
    /// Bound field bytes, twelve; includes the previously framed identifier.
    pub interpreted_field_bytes: usize,
    /// Always false: three fields do not establish full descriptor serialization.
    pub complete_descriptor_serialization: bool,
}

/// Exact conditional builder projection, with skip distinct from a populated descriptor proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryDescriptorCompletion {
    /// Both stored bounds are zero and the builder appends no descriptor.
    SkippedBothZero,
    /// Only body ID and two signed bounds are proposed; the full descriptor remains unassigned.
    ProposedThreeRuntimeFields,
}

/// Required evidence boundaries for stored row editing and conditional runtime proposals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryBodyDiagnostic {
    /// Remaining row fields and complete serialized descriptor grammar are not assigned.
    CompleteDescriptorUnassigned,
    /// Fresh-list, study-mode and reused-list runtime conditions were not observed executing.
    ConditionalBuilderNotExecuted,
    /// No CAD/assembly body ownership conversion is bound.
    CadOwnershipUnassigned,
    /// Geometry/topology consistency, intersections and tolerances remain unchecked.
    GeometryNotValidated,
}

/// Immutable source/history with an exact checked three-field proposal or explicit skip.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryDescriptorProposal {
    /// Complete immutable source row and its original copy parents.
    pub source: SimulationAuxiliaryBodyRow,
    /// Original source identity from which all proposed fields derive.
    pub parent_revision: SimulationAuxiliaryBodyRevision,
    /// Proposed inputs without adjusting either consumer's endpoint convention.
    pub inputs: Option<SimulationBodyTypeInputs>,
    /// Conditional skip versus limited field proposal.
    pub completion: SimulationAuxiliaryDescriptorCompletion,
    /// Exact source/semantic scope, not whole descriptor acceptance.
    pub coverage: SimulationAuxiliaryBodyCoverage,
    /// Achieved body/bound pattern error, zero within the checked native-i32 profile.
    pub field_pattern_error_bits: u32,
    /// Explicit unresolved runtime and geometry responsibilities.
    pub diagnostics: [SimulationAuxiliaryBodyDiagnostic; 4],
}

impl SimulationAuxiliaryBodyRow {
    /// Project the inspected conditional three-field bridge, e.g. `row.descriptor_proposal()?`; i32 decrement overflow is rejected.
    pub fn descriptor_proposal(&self) -> Result<SimulationAuxiliaryDescriptorProposal> {
        let fields = self.fields();
        let inputs = if fields.descriptor_appended {
            Some(checked_inputs(fields)?)
        } else {
            None
        };
        Ok(SimulationAuxiliaryDescriptorProposal {
            source: self.clone(),
            parent_revision: self.revision.clone(),
            inputs,
            completion: if inputs.is_some() {
                SimulationAuxiliaryDescriptorCompletion::ProposedThreeRuntimeFields
            } else {
                SimulationAuxiliaryDescriptorCompletion::SkippedBothZero
            },
            coverage: self.field_coverage(),
            field_pattern_error_bits: 0,
            diagnostics: body_diagnostics(),
        })
    }

    pub(super) fn field_coverage(&self) -> SimulationAuxiliaryBodyCoverage {
        SimulationAuxiliaryBodyCoverage {
            mode: SimulationAuxiliaryBodyCheckMode::ExactCompleteRowAndThreeFieldBinding,
            stored_bytes: self.bytes.len(),
            interpreted_field_bytes: 12,
            complete_descriptor_serialization: false,
        }
    }
}

fn checked_inputs(fields: SimulationAuxiliaryBodyFields) -> Result<SimulationBodyTypeInputs> {
    let start = fields
        .stored_low
        .checked_sub(1)
        .ok_or_else(|| decrement_error(fields.stored_low, "first"))?;
    let exclusive_upper = fields
        .stored_high
        .checked_sub(1)
        .ok_or_else(|| decrement_error(fields.stored_high, "last"))?;
    Ok(SimulationBodyTypeInputs {
        body_id: fields.body_id,
        start,
        exclusive_upper,
    })
}

fn decrement_error(value: i32, field: &str) -> crate::BessemerError {
    invalid(
        "auxiliary descriptor decrement",
        format!(
            "{field} ordinal {value}; expected an ordinal whose decrement fits signed i32; raw fields remain retained"
        ),
    )
}

pub(super) fn body_diagnostics() -> [SimulationAuxiliaryBodyDiagnostic; 4] {
    [
        SimulationAuxiliaryBodyDiagnostic::CompleteDescriptorUnassigned,
        SimulationAuxiliaryBodyDiagnostic::ConditionalBuilderNotExecuted,
        SimulationAuxiliaryBodyDiagnostic::CadOwnershipUnassigned,
        SimulationAuxiliaryBodyDiagnostic::GeometryNotValidated,
    ]
}
