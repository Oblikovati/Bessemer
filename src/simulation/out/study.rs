use super::{
    SimulationOutArchive, SimulationOutEdit, SimulationOutFieldId, SimulationOutLimits,
    SimulationOutPairedTotals,
};
use crate::compound::{CompoundEdit, CompoundLimits};
use crate::simulation::{Result, SimulationStudy, require};

/// Complete CWR with actual paired MAS/GEN and a complete OUT report.
#[derive(Debug, Clone)]
pub struct SimulationOutStudy {
    paired: SimulationStudy,
    report: SimulationOutArchive,
}

/// Complete report edit composed with carrier history and paired-count diagnostics.
#[derive(Debug, Clone)]
pub struct SimulationOutStudyEdit {
    /// Resulting complete CWR study.
    pub study: SimulationOutStudy,
    /// Exact report edit, coverage, all occurrence derivations and exclusions.
    pub report_edit: SimulationOutEdit,
    /// Complete carrier derivation and logical preservation checks.
    pub carrier_edit: CompoundEdit,
    /// Required printed totals; missing or inconclusive statuses remain explicit.
    pub totals: SimulationOutPairedTotals,
}

impl SimulationOutStudy {
    /// Read actual paired binary context and OUT, e.g. `SimulationOutStudy::from_bytes(cwr, compound_limits, out_limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        compound_limits: CompoundLimits,
        out_limits: SimulationOutLimits,
    ) -> Result<Self> {
        let paired = SimulationStudy::from_bytes(bytes, compound_limits)?;
        let entry = paired.carrier().entry_at_path("OUT")?;
        let report =
            SimulationOutArchive::from_bytes(paired.carrier().stream(entry.id())?, out_limits)?;
        Ok(Self { paired, report })
    }

    /// Complete actual MAS/GEN and carrier context, e.g. `study.paired().master()`.
    pub fn paired(&self) -> &SimulationStudy {
        &self.paired
    }

    /// Complete immutable report, e.g. `study.report().fields()`.
    pub fn report(&self) -> &SimulationOutArchive {
        &self.report
    }

    /// Both required report/binary comparisons, e.g. `study.paired_totals().nodes.status`.
    pub fn paired_totals(&self) -> SimulationOutPairedTotals {
        self.report.paired_totals(self.paired.image())
    }

    /// Replace one report token through a complete CWR, e.g. `study.edit_numeric_lexeme(&id, "2")?`.
    pub fn edit_numeric_lexeme(
        &self,
        id: &SimulationOutFieldId,
        lexeme: &str,
    ) -> Result<SimulationOutStudyEdit> {
        let report_edit = self.report.edit_numeric_lexeme(id, lexeme)?;
        let entry = self.paired.carrier().entry_at_path("OUT")?;
        let carrier_edit = self
            .paired
            .carrier()
            .edit_stream(entry.id(), &report_edit.archive.to_bytes())?;
        let paired = self.paired.with_carrier(carrier_edit.file.clone())?;
        verify_reopened_report(&paired, &report_edit.archive, self.report.limits)?;
        let study = Self {
            paired,
            report: report_edit.archive.clone(),
        };
        Ok(SimulationOutStudyEdit {
            totals: study.paired_totals(),
            study,
            report_edit,
            carrier_edit,
        })
    }
}

fn verify_reopened_report(
    paired: &SimulationStudy,
    expected: &SimulationOutArchive,
    limits: SimulationOutLimits,
) -> Result<()> {
    let reopened_entry = paired.carrier().entry_at_path("OUT")?;
    let reopened =
        SimulationOutArchive::from_bytes(paired.carrier().stream(reopened_entry.id())?, limits)?;
    require(
        reopened.to_bytes() == expected.to_bytes(),
        "CWR OUT replacement",
        "reopened bytes differ; expected exact complete replacement report",
    )?;
    Ok(())
}
