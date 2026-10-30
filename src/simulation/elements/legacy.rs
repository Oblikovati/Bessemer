use super::{
    Result, SimulationElementDiagnostic, SimulationElementRecord, SimulationElementRecordId,
    require,
};
use crate::display_lists::DisplayEditCompletion;

/// Caller-supplied context for one record assumed visited by the legacy loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElementLegacyContext {
    version: i32,
    analysis_code: i32,
}

impl SimulationElementLegacyContext {
    /// Normalize the supported legacy window once, e.g. `SimulationElementLegacyContext::new(820024, 4)?`.
    pub fn new(version: i32, analysis_code: i32) -> Result<Self> {
        require(
            (800050..820025).contains(&version),
            "Simulation legacy element context",
            format!(
                "version {version}, analysis code {analysis_code}; expected version 800050..=820024 and an explicit analysis code"
            ),
        )?;
        Ok(Self {
            version,
            analysis_code,
        })
    }

    /// Supplied version, e.g. `context.version()`; this is not inferred from a loose record.
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Supplied analysis domain, e.g. `context.analysis_code()`; independent of the family byte.
    pub fn analysis_code(&self) -> i32 {
        self.analysis_code
    }
}

/// Exact non-destructive record projection; no GEN file or native loop was executed.
#[derive(Debug, Clone)]
pub struct SimulationElementLegacyProjection {
    /// Complete resulting snapshot; only stored-code bytes may differ.
    pub record: SimulationElementRecord,
    /// Byte-change completion, including an authentic no-op.
    pub completion: DisplayEditCompletion,
    /// Original snapshot identity.
    pub parent: SimulationElementRecordId,
    /// Result identity; unchanged projections reuse the parent.
    pub result: SimulationElementRecordId,
    /// Exact supplied, normalized context.
    pub context: SimulationElementLegacyContext,
    /// Stored low sixteen bits after the inspected rewrite.
    pub rewritten_code: u16,
    /// All 140 bytes were compared; only 26..28 may change.
    pub validation: SimulationElementLegacyCoverage,
    /// Exact requested word fidelity; always zero differing bits.
    pub requested_pattern_error_bits: u16,
    /// Runtime and engineering exclusions accompany the result.
    pub diagnostics: [SimulationElementDiagnostic; 4],
}

/// Exact byte comparison scope, without geometry or native-loop acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElementLegacyCoverage {
    /// Complete 140-byte copy with the two-byte replacement checked exactly.
    ExactCompleteRecordWithStoredCodeReplacement,
}

impl SimulationElementRecord {
    /// Project one explicitly visited legacy record on a copy, e.g. `record.project_legacy(context)`; never applied automatically.
    pub fn project_legacy(
        &self,
        context: SimulationElementLegacyContext,
    ) -> SimulationElementLegacyProjection {
        let fields = self.classification();
        let code = match (fields.accessor_node_count, context.analysis_code) {
            (2, 4) => 28,
            (2, _) => 37,
            _ => fields.selected_code as u16,
        };
        let mut record = self.clone();
        if code != fields.stored_code {
            record.bytes[26..28].copy_from_slice(&code.to_le_bytes());
            record.id = SimulationElementRecordId(crate::archive_codec::ArchiveRevision::default());
        }
        legacy_result(self, record, context, code)
    }
}

fn legacy_result(
    parent: &SimulationElementRecord,
    record: SimulationElementRecord,
    context: SimulationElementLegacyContext,
    rewritten_code: u16,
) -> SimulationElementLegacyProjection {
    SimulationElementLegacyProjection {
        completion: if parent.id == record.id {
            DisplayEditCompletion::Unchanged
        } else {
            DisplayEditCompletion::Changed
        },
        parent: parent.id.clone(),
        result: record.id.clone(),
        record,
        context,
        rewritten_code,
        validation: SimulationElementLegacyCoverage::ExactCompleteRecordWithStoredCodeReplacement,
        requested_pattern_error_bits: 0,
        diagnostics: legacy_diagnostics(),
    }
}

fn legacy_diagnostics() -> [SimulationElementDiagnostic; 4] {
    let [engineering, runtime, geometry] = super::projection_diagnostics();
    [
        engineering,
        runtime,
        geometry,
        SimulationElementDiagnostic::LoopVisitationNotProven,
    ]
}
