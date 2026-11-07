use super::{SimulationOutArchive, SimulationOutDecimal, SimulationOutFieldId};
use crate::simulation::SimulationGenArchive;
use std::sync::Arc;

/// Report-local comparison against the actual paired binary mesh counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationOutTotalStatus {
    /// Exactly one printed occurrence has the exact expected nonnegative integer.
    Matches,
    /// Exactly one printed nonnegative integer differs from the binary count.
    Mismatch,
    /// The required symbol is absent.
    Missing,
    /// Duplicate occurrences or a non-count numeric value prevent a unique comparison.
    Inconclusive,
}

/// Every candidate for one printed mesh total, without collapsing duplicate symbols.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutTotalComparison {
    /// Actual count from the complete MAS-selected GEN framing.
    pub expected: u32,
    /// All printed occurrence identities in stored order.
    pub candidates: Arc<[SimulationOutFieldId]>,
    /// Exact checked count, when one occurrence supplies it.
    pub printed: Option<u32>,
    /// Complete, mismatched, missing or inconclusive comparison.
    pub status: SimulationOutTotalStatus,
}

/// Independent printed node/element totals, without geometry acceptance claims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutPairedTotals {
    /// NUMNP against the paired GEN node count.
    pub nodes: SimulationOutTotalComparison,
    /// NUME against the paired GEN element count.
    pub elements: SimulationOutTotalComparison,
}

impl SimulationOutArchive {
    /// Compare required printed totals using real paired GEN context, e.g. `report.paired_totals(study.image())`.
    pub fn paired_totals(&self, image: &SimulationGenArchive) -> SimulationOutPairedTotals {
        SimulationOutPairedTotals {
            nodes: comparison(self, "NUMNP", image.layout().counts[3]),
            elements: comparison(self, "NUME", image.layout().counts[4]),
        }
    }
}

fn comparison(
    report: &SimulationOutArchive,
    symbol: &str,
    expected: u32,
) -> SimulationOutTotalComparison {
    let fields: Vec<_> = report
        .fields
        .iter()
        .filter(|field| field.symbol.as_ref() == symbol)
        .collect();
    let printed = if fields.len() == 1 {
        exact_count(&fields[0].value)
    } else {
        None
    };
    let status = comparison_status(fields.len(), printed, expected);
    SimulationOutTotalComparison {
        expected,
        candidates: fields.iter().map(|field| field.id.clone()).collect(),
        printed,
        status,
    }
}

fn exact_count(decimal: &SimulationOutDecimal) -> Option<u32> {
    if decimal.digits.iter().all(|digit| *digit == 0) {
        return Some(0);
    }
    if decimal.negative {
        return None;
    }
    let digits = integral_digits(decimal)?;
    let value = digits.iter().try_fold(0_u32, |value, digit| {
        value.checked_mul(10)?.checked_add(u32::from(*digit))
    })?;
    scale_count(value, decimal.exponent)
}

fn scale_count(value: u32, exponent: i64) -> Option<u32> {
    if exponent <= 0 {
        return Some(value);
    }
    if exponent > 10 {
        return None;
    }
    (0..exponent).try_fold(value, |value, _| value.checked_mul(10))
}

fn comparison_status(
    count: usize,
    printed: Option<u32>,
    expected: u32,
) -> SimulationOutTotalStatus {
    match (count, printed) {
        (0, _) => SimulationOutTotalStatus::Missing,
        (1, Some(value)) if value == expected => SimulationOutTotalStatus::Matches,
        (1, Some(_)) => SimulationOutTotalStatus::Mismatch,
        _ => SimulationOutTotalStatus::Inconclusive,
    }
}

fn integral_digits(decimal: &SimulationOutDecimal) -> Option<&[u8]> {
    let digits = &decimal.digits;
    let integral_end = digits.len() as i128 + i128::from(decimal.exponent.min(0));
    if integral_end <= 0 || integral_end > digits.len() as i128 {
        return None;
    }
    let end = integral_end as usize;
    if digits[end..].iter().any(|digit| *digit != 0) {
        return None;
    }
    Some(&digits[..end])
}
