use super::mode3_failure;
use crate::{FailureClass, Result};
use std::sync::Arc;

/// Immutable copied destination observations with an explicit resource budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationMode3RgbTriples {
    entries: Arc<[[u32; 3]]>,
    maximum_triples: usize,
}
impl SimulationMode3RgbTriples {
    /// Copy known triples within an explicit budget; e.g. `SimulationMode3RgbTriples::from_slice(&[[0; 3]; 6], 10)?`.
    pub fn from_slice(entries: &[[u32; 3]], maximum_triples: usize) -> Result<Self> {
        if entries.len() > maximum_triples {
            return Err(mode3_failure(
                "triple budget",
                FailureClass::Invalid,
                format!(
                    "{} supplied triples; expected at most {maximum_triples}",
                    entries.len()
                ),
            ));
        }
        Ok(Self {
            entries: Arc::from(entries),
            maximum_triples,
        })
    }
    /// Read immutable complete triples without acquiring a lock; e.g. `snapshot.triples()`.
    pub fn triples(&self) -> &[[u32; 3]] {
        &self.entries
    }
    /// Inspect the constructor budget retained by derived snapshots; e.g. `snapshot.maximum_triples()`.
    pub fn maximum_triples(&self) -> usize {
        self.maximum_triples
    }
    pub(super) fn patched(&self, rgb: [u32; 3]) -> Result<(Self, [u32; 3])> {
        let mut copied = self.entries.to_vec();
        let observed = copied.len();
        let slot = copied.get_mut(5).ok_or_else(|| {
            mode3_failure(
                "destination prefix",
                FailureClass::Invalid,
                format!("{observed} complete triples; expected at least6 for fixed index5"),
            )
        })?;
        let before = *slot;
        *slot = rgb;
        Ok((
            Self {
                entries: Arc::from(copied),
                maximum_triples: self.maximum_triples,
            },
            before,
        ))
    }
}
