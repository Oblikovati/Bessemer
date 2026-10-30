use super::*;

/// Full normalization/binding history with every original input snapshot retained.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryBoundProjection {
    /// Complete original conversion and copy parents.
    pub normalization: SimulationAuxiliaryNormalization,
    /// Exact ID-gated binding and supplied result bits.
    pub binding: SimulationAuxiliaryPointerBinding,
    /// Final byte parents, with selected slot spans derived from supplied result bits.
    pub final_spans: Arc<[SimulationAuxiliarySpanCopy]>,
}

impl SimulationAuxiliaryBodyRow {
    /// Compose both supplied-context stages, e.g. `row.normalize_and_bind(Some(&trial), index, Some(0))?`; no native allocation/read/write occurs.
    pub fn normalize_and_bind(
        &self,
        initial: Option<&SimulationAuxiliaryWorkspace>,
        index: SimulationAuxiliaryLogicalIndex,
        result: Option<u64>,
    ) -> Result<SimulationAuxiliaryBoundProjection> {
        let normalization = self.normalize_workspace(initial)?;
        let binding = normalization
            .workspace
            .project_pointer_binding(index, result)?;
        let final_spans = compose_spans(&normalization.copied_spans, binding.plan.selected);
        Ok(SimulationAuxiliaryBoundProjection {
            normalization,
            binding,
            final_spans,
        })
    }
}

fn compose_spans(
    spans: &[SimulationAuxiliarySpanCopy],
    selected: bool,
) -> Arc<[SimulationAuxiliarySpanCopy]> {
    if !selected {
        return spans.into();
    }
    let mut result = Vec::new();
    for original in spans {
        append_retained(&mut result, *original);
    }
    result.push(SimulationAuxiliarySpanCopy {
        source: SimulationAuxiliaryCopySource::SuppliedAllocationResult,
        source_offset: 0,
        destination_offset: 8,
        length: 8,
    });
    result.sort_by_key(|span| span.destination_offset);
    result.into()
}

fn append_retained(
    output: &mut Vec<SimulationAuxiliarySpanCopy>,
    original: SimulationAuxiliarySpanCopy,
) {
    let start = original.destination_offset;
    let end = start + original.length;
    if end <= 8 || start >= 16 {
        output.push(original);
        return;
    }
    if start < 8 {
        output.push(SimulationAuxiliarySpanCopy {
            length: 8 - start,
            ..original
        });
    }
    if end > 16 {
        output.push(SimulationAuxiliarySpanCopy {
            source: original.source,
            source_offset: original.source_offset + (16 - start),
            destination_offset: 16,
            length: end - 16,
        });
    }
}
