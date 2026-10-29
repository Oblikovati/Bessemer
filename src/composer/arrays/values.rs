use super::*;
use crate::archive_codec::{ArchiveRevision, StoredFloat};
use std::sync::Arc;

/// Owned immutable row-interleaved binary32 patterns under an explicit version-7.9 profile.
#[derive(Debug, Clone)]
pub struct ComposerVectorValues {
    patterns: Arc<[StoredFloat]>,
    layout: ComposerVectorLayout,
    report: ComposerVectorReconstructionReport,
}

impl ComposerVectorValues {
    /// Exact component patterns, e.g. `values.patterns()[0].bits()`.
    pub fn patterns(&self) -> &[StoredFloat] {
        &self.patterns
    }

    /// Original row/dimension shape, e.g. `values.layout().count`.
    pub fn layout(&self) -> ComposerVectorLayout {
        self.layout
    }

    /// Complete operation report, e.g. `values.report().coverage.components`.
    pub fn report(&self) -> &ComposerVectorReconstructionReport {
        &self.report
    }
}

pub(super) fn reconstruct_vectors(
    block: &ComposerVectorBlock,
    profile: ComposerVectorProfile,
    maximum_components: usize,
) -> Result<ComposerVectorValues> {
    let layout = block.layout();
    let count = component_count(layout, maximum_components)?;
    require_finite_endpoints(block)?;
    let patterns = expand_components(block, count)?;
    let report = reconstruction_report(block, profile, count);
    Ok(ComposerVectorValues {
        patterns,
        layout,
        report,
    })
}

fn component_count(layout: ComposerVectorLayout, maximum: usize) -> Result<usize> {
    (layout.count as usize)
        .checked_mul(layout.dimensions.lanes())
        .filter(|count| *count <= maximum)
        .ok_or_else(|| {
            BessemerError::new(
                "reconstruct Composer vectors",
                "component output",
                FailureClass::Invalid,
                format!(
                    "rows {} * lanes {}; expected at most {maximum} components",
                    layout.count,
                    layout.dimensions.lanes()
                ),
            )
        })
}

fn expand_components(block: &ComposerVectorBlock, count: usize) -> Result<Arc<[StoredFloat]>> {
    let mut patterns = Vec::with_capacity(count);
    let mut codes = block.codes();
    for _ in 0..block.layout().count {
        for axis in block.axes() {
            patterns.push(reconstruct_lane(*axis, &mut codes, block.packing())?);
        }
    }
    Ok(patterns.into())
}

fn require_finite_endpoints(block: &ComposerVectorBlock) -> Result<()> {
    if block.layout().count == 0 {
        return Ok(());
    }
    for axis in block.axes() {
        if let ComposerVectorAxis::Quantized { first, second } = axis {
            require_finite_word(*first)?;
            require_finite_word(*second)?;
        }
    }
    Ok(())
}

fn require_finite_word(word: u32) -> Result<()> {
    if word & 0x7f800000 != 0x7f800000 {
        return Ok(());
    }
    Err(BessemerError::new(
        "reconstruct Composer vectors",
        "quantized endpoint",
        FailureClass::Unsupported,
        format!(
            "pattern 0x{word:08x}; expected finite binary32 for nearest-even-gradual arithmetic"
        ),
    ))
}

fn reconstruct_lane(
    axis: ComposerVectorAxis,
    codes: &mut impl Iterator<Item = u32>,
    packing: ComposerVectorPacking,
) -> Result<StoredFloat> {
    if let ComposerVectorAxis::Constant(word) = axis {
        return Ok(StoredFloat::from_bits(word));
    }
    let code = codes
        .next()
        .ok_or_else(|| invalid("missing packed scalar; expected one per varying lane"))?;
    let word = match axis {
        ComposerVectorAxis::Quantized { first, second } => {
            binary32::reconstruct_code(code, packing.width(), first, second)
        }
        _ => code,
    };
    Ok(StoredFloat::from_bits(word))
}

fn reconstruction_report(
    block: &ComposerVectorBlock,
    profile: ComposerVectorProfile,
    components: usize,
) -> ComposerVectorReconstructionReport {
    ComposerVectorReconstructionReport {
        profile,
        completion: ComposerVectorValueCompletion::Completed,
        fidelity: ComposerVectorValueFidelity::ExactProfilePatterns,
        coverage: reconstruction_coverage(block, components),
        history: ComposerArrayHistory::Derived {
            parent: block.revision().clone(),
            result: ComposerArrayRevision(ArchiveRevision::default()),
        },
        diagnostics: VALUE_DIAGNOSTICS,
    }
}

const VALUE_DIAGNOSTICS: [ComposerVectorValueDiagnostic; 4] = [
    ComposerVectorValueDiagnostic::NativeControlStateNotChecked,
    ComposerVectorValueDiagnostic::GeometryNotChecked,
    ComposerVectorValueDiagnostic::QuantizationErrorNotEstablished,
    ComposerVectorValueDiagnostic::NativeApplicationNotChecked,
];

fn reconstruction_coverage(
    block: &ComposerVectorBlock,
    components: usize,
) -> ComposerVectorValueCoverage {
    let quantized = block
        .axes()
        .iter()
        .any(|axis| matches!(axis, ComposerVectorAxis::Quantized { .. }));
    ComposerVectorValueCoverage {
        mode: ComposerVectorValueCheckMode::ExactDeclaredProfile,
        source: block.coverage(),
        components,
        raw_copy_only: !quantized,
    }
}
