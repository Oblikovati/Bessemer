use super::{Result, require};
use crate::simulation::invalid;
use std::sync::Arc;

/// Explicit allocation budgets for external mapping context and output slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElfMappingLimits {
    /// Maximum source-ID entries supplied by the caller.
    pub maximum_source_identifiers: usize,
    /// Maximum destination slots, including unassigned slots.
    pub maximum_destination_records: usize,
}

/// Immutable external mapping normalized once from one-based targets and zero skips.
#[derive(Debug, Clone)]
pub struct SimulationElfMappingContext {
    pub(super) targets: Arc<[Option<usize>]>,
    pub(super) destination_count: usize,
}

impl SimulationElfMappingContext {
    /// Validate explicit map context once, e.g. `SimulationElfMappingContext::new(&[0, 2], 3, limits)?`.
    pub fn new(
        source_to_destination: &[u32],
        destination_count: usize,
        limits: SimulationElfMappingLimits,
    ) -> Result<Self> {
        require(
            source_to_destination.len() <= limits.maximum_source_identifiers
                && destination_count <= limits.maximum_destination_records,
            "ELF mapping budgets",
            format!(
                "{} source IDs and {destination_count} destinations; expected at most {} and {}",
                source_to_destination.len(),
                limits.maximum_source_identifiers,
                limits.maximum_destination_records
            ),
        )?;
        let targets = normalized_targets(source_to_destination, destination_count)?;
        Ok(Self {
            targets: targets.into(),
            destination_count,
        })
    }

    /// Explicit complete destination extent, e.g. `mapping.destination_count()`.
    pub fn destination_count(&self) -> usize {
        self.destination_count
    }

    pub(super) fn destination(&self, identifier: i32) -> Result<Option<usize>> {
        require(
            identifier > 0,
            "ELF source identifier",
            format!("{identifier}; expected a positive identifier requiring external mapping"),
        )?;
        self.targets.get(identifier as usize - 1).copied().ok_or_else(|| invalid("ELF source-ID map", format!("identifier {identifier}; expected a supplied mapping entry among {} source IDs", self.targets.len())))
    }
}

fn normalized_targets(source: &[u32], destination_count: usize) -> Result<Vec<Option<usize>>> {
    let mut targets = Vec::new();
    targets.try_reserve_exact(source.len()).map_err(|error| {
        invalid(
            "ELF map allocation",
            format!(
                "{} source IDs: {error}; expected addressable normalized mapping storage",
                source.len()
            ),
        )
    })?;
    for (index, target) in source.iter().enumerate() {
        targets.push(normalized_target(index, *target, destination_count)?);
    }
    Ok(targets)
}

fn normalized_target(index: usize, target: u32, destination_count: usize) -> Result<Option<usize>> {
    require(
        target as usize <= destination_count,
        "ELF mapping target",
        format!(
            "source ID {} target {target}; expected zero skip or 1..={destination_count}",
            index + 1
        ),
    )?;
    if target == 0 {
        return Ok(None);
    }
    Ok(Some(target as usize - 1))
}
