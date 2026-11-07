use super::*;

/// Completed memory conversion distinguished from file serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryNormalizationCompletion {
    /// Exact modern stored image copied without using prior workspace bytes.
    CopiedModernImage,
    /// Only the documented legacy spans were copied into the supplied prior image.
    ConvertedLegacySpans,
}

/// Immutable conversion with both real source parents and exact span history retained.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryNormalization {
    /// Complete original row, including any legacy word not copied.
    pub source: SimulationAuxiliaryBodyRow,
    /// Exact prior legacy workspace; modern conversion ignores it.
    pub initial_workspace: Option<SimulationAuxiliaryWorkspace>,
    /// Complete normalized memory image with retained GEN origin when known.
    pub workspace: SimulationAuxiliaryWorkspace,
    /// Actual conversion kind, not a native allocator/storage success status.
    pub completion: SimulationAuxiliaryNormalizationCompletion,
    /// Four legacy spans or one modern span, covering all destination bytes.
    pub copied_spans: Arc<[SimulationAuxiliarySpanCopy]>,
    /// Original legacy word omitted by conversion, retained verbatim.
    pub legacy_word_discarded: Option<[u8; 4]>,
    /// Exact byte scope and memory-only limitation.
    pub coverage: SimulationAuxiliaryNormalizationCoverage,
    /// Achieved copied-pattern error, zero.
    pub copied_pattern_error_bits: u32,
    /// Nonportable slot and unexecuted runtime responsibilities.
    pub diagnostics: [SimulationAuxiliaryRuntimeDiagnostic; 4],
}

impl SimulationAuxiliaryBodyRow {
    /// Normalize against real supplied prior bytes, e.g. `row.normalize_workspace(Some(&trial))?`; legacy slot bytes are never invented.
    pub fn normalize_workspace(
        &self,
        initial: Option<&SimulationAuxiliaryWorkspace>,
    ) -> Result<SimulationAuxiliaryNormalization> {
        let legacy = self.profile == SimulationAuxiliaryBodyProfile::Stored284;
        let prior = required_prior(initial, legacy)?;
        let spans = normalization_spans(legacy);
        let mut bytes = prior.map_or([0u8; 288], |prior| std::array::from_fn(|i| prior.bytes[i]));
        copy_spans(self, prior, &spans, &mut bytes)?;
        let mut workspace = SimulationAuxiliaryWorkspace::from_bytes(&bytes)?;
        workspace.gen_parent = self.gen_parent.clone();
        Ok(normalization_result(self, prior, workspace, spans, legacy))
    }
}

fn required_prior(
    initial: Option<&SimulationAuxiliaryWorkspace>,
    legacy: bool,
) -> Result<Option<&SimulationAuxiliaryWorkspace>> {
    if !legacy {
        return Ok(None);
    }
    initial.map(Some).ok_or_else(||invalid("legacy auxiliary workspace", "stored profile 284 with no prior image; expected an explicitly supplied complete 288-byte initial workspace"))
}

fn normalization_spans(legacy: bool) -> Arc<[SimulationAuxiliarySpanCopy]> {
    use SimulationAuxiliaryCopySource::{InitialWorkspace, StoredRow};
    let spans = if legacy {
        vec![
            span(StoredRow, 0, 0, 8),
            span(InitialWorkspace, 8, 8, 8),
            span(StoredRow, 12, 16, 8),
            span(StoredRow, 20, 24, 264),
        ]
    } else {
        vec![span(StoredRow, 0, 0, 288)]
    };
    spans.into()
}

fn span(
    source: SimulationAuxiliaryCopySource,
    source_offset: usize,
    destination_offset: usize,
    length: usize,
) -> SimulationAuxiliarySpanCopy {
    SimulationAuxiliarySpanCopy {
        source,
        source_offset,
        destination_offset,
        length,
    }
}

fn copy_spans(
    row: &SimulationAuxiliaryBodyRow,
    prior: Option<&SimulationAuxiliaryWorkspace>,
    spans: &[SimulationAuxiliarySpanCopy],
    output: &mut [u8; 288],
) -> Result<()> {
    for span in spans {
        let source = span_source(row, prior, span.source)?;
        output[span.destination_offset..span.destination_offset + span.length]
            .copy_from_slice(&source[span.source_offset..span.source_offset + span.length]);
    }
    Ok(())
}

fn normalization_result(
    row: &SimulationAuxiliaryBodyRow,
    prior: Option<&SimulationAuxiliaryWorkspace>,
    workspace: SimulationAuxiliaryWorkspace,
    copied_spans: Arc<[SimulationAuxiliarySpanCopy]>,
    legacy: bool,
) -> SimulationAuxiliaryNormalization {
    SimulationAuxiliaryNormalization {
        source: row.clone(),
        initial_workspace: prior.cloned(),
        workspace,
        copied_spans,
        completion: if legacy {
            SimulationAuxiliaryNormalizationCompletion::ConvertedLegacySpans
        } else {
            SimulationAuxiliaryNormalizationCompletion::CopiedModernImage
        },
        legacy_word_discarded: legacy.then(|| std::array::from_fn(|i| row.bytes[8 + i])),
        coverage: normalization_coverage(legacy),
        copied_pattern_error_bits: 0,
        diagnostics: outcome::runtime_diagnostics(),
    }
}

fn span_source<'a>(
    row: &'a SimulationAuxiliaryBodyRow,
    prior: Option<&'a SimulationAuxiliaryWorkspace>,
    source: SimulationAuxiliaryCopySource,
) -> Result<&'a [u8]> {
    match source {
        SimulationAuxiliaryCopySource::StoredRow => Ok(&row.bytes),
        SimulationAuxiliaryCopySource::InitialWorkspace => prior
            .map(|workspace| workspace.bytes.as_ref())
            .ok_or_else(|| {
                invalid(
                    "auxiliary span parent",
                    "initial workspace None; expected an explicit 288-byte retained-slot source",
                )
            }),
        SimulationAuxiliaryCopySource::SuppliedAllocationResult => Err(invalid(
            "auxiliary normalization span",
            "allocation-result parent before binding; expected stored-row or prior-workspace bytes",
        )),
    }
}

fn normalization_coverage(legacy: bool) -> SimulationAuxiliaryNormalizationCoverage {
    SimulationAuxiliaryNormalizationCoverage {
        mode: SimulationAuxiliaryNormalizationCheckMode::ExactStoredSpansAndSuppliedSlot,
        destination_bytes: 288,
        prior_workspace_bytes: if legacy { 8 } else { 0 },
        uncopied_stored_bytes: if legacy { 4 } else { 0 },
        memory_only: true,
    }
}
