use super::{Result, invalid};
use std::sync::Arc;

const CARRIER_SUFFIX: [u16; 4] = [0x2e, 0x43, 0x57, 0x52];

/// NUL-free native CWR logical name with its entire-input last-dot boundary normalized once.
#[derive(Debug, Clone)]
pub struct SimulationCwrLogicalName {
    units: Arc<[u16]>,
    last_dot: usize,
}

/// Successful lexical completion does not establish native name acceptance or existence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStorageLookupCompletion {
    /// Every supplied code unit and exact component slice was checked.
    CompletedLexicalProjection,
}

/// Exact reproducible name operation, without filesystem/COM interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStorageLookupCheckMode {
    /// Last ASCII dot across the entire UTF-16 input, literal .CWR and untouched suffix.
    ExactLastDotUtf16LexicalProjection,
}

/// Owned lexical lookup metadata retaining the complete original code-unit parent.
#[derive(Debug, Clone)]
pub struct SimulationStorageLocator {
    /// Complete original normalized logical name.
    pub logical_name: SimulationCwrLogicalName,
    /// Original prefix followed by exact literal .CWR, without path normalization.
    pub storage_name: Arc<[u16]>,
    /// Exact original suffix after the final dot, including whitespace/case and empty suffixes.
    pub stream_name: Arc<[u16]>,
    /// Number of original prefix code units contributing to the storage name.
    pub storage_prefix_units: usize,
    /// Original code-unit offset of the complete stream suffix.
    pub stream_parent_offset: usize,
    /// Successful lexical completion only.
    pub completion: SimulationStorageLookupCompletion,
    /// Exact code-unit slice check mode.
    pub check_mode: SimulationStorageLookupCheckMode,
    /// Every original UTF-16 code unit was checked for the profile guards and final dot.
    pub checked_input_units: usize,
    /// Documented native storage-open mode pattern, not an executed open.
    pub storage_mode: u32,
    /// Documented native storage-format pattern.
    pub storage_format: u32,
    /// Documented native stream-open mode pattern, not an executed open.
    pub stream_mode: u32,
    /// Always false: no GEN/MAS or other stream payload is decoded by name projection.
    pub payload_decoded: bool,
    /// Always false: no existence probe, filesystem resolution or COM call executes.
    pub native_io_executed: bool,
    /// Always false: native filesystem/COM acceptance is not a lexical result.
    pub native_name_acceptance_established: bool,
    /// Achieved requested code-unit pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

impl SimulationCwrLogicalName {
    /// Normalize the reproduced NUL-free dotted profile, e.g. `SimulationCwrLogicalName::from_units(&name_units)?`.
    pub fn from_units(units: &[u16]) -> Result<Self> {
        if let Some(index) = units.iter().position(|unit| *unit == 0) {
            return Err(invalid(
                "CWR logical name",
                format!(
                    "NUL code unit at index {index} in {} units; expected a NUL-free dotted UTF-16 name",
                    units.len()
                ),
            ));
        }
        let last_dot = units.iter().rposition(|unit| *unit == 0x2e).ok_or_else(|| invalid("CWR logical name", format!("{} code units with no ASCII dot; expected a NUL-free name containing at least one dot", units.len())))?;
        Ok(Self {
            units: units.into(),
            last_dot,
        })
    }
    /// Every original raw unit, including isolated surrogates, e.g. `name.code_units()`.
    pub fn code_units(&self) -> &[u16] {
        &self.units
    }
    /// Project only exact lexical metadata, e.g. `name.project_locator()?`; directories and suffixes are not normalized.
    pub fn project_locator(&self) -> Result<SimulationStorageLocator> {
        let storage = storage_units(&self.units[..self.last_dot])?;
        Ok(locator_result(self, storage))
    }
}

fn storage_units(prefix: &[u16]) -> Result<Arc<[u16]>> {
    let length = prefix.len().checked_add(CARRIER_SUFFIX.len()).ok_or_else(|| invalid("CWR storage name", format!("{} prefix units plus four suffix units; expected representable complete extent", prefix.len())))?;
    let mut units = Vec::new();
    units.try_reserve_exact(length).map_err(|error| {
        invalid(
            "CWR storage name",
            format!(
                "{length} units, allocation {error}; expected available complete code-unit storage"
            ),
        )
    })?;
    units.extend_from_slice(prefix);
    units.extend_from_slice(&CARRIER_SUFFIX);
    Ok(units.into())
}

fn locator_result(
    logical: &SimulationCwrLogicalName,
    storage: Arc<[u16]>,
) -> SimulationStorageLocator {
    SimulationStorageLocator {
        logical_name: logical.clone(),
        storage_name: storage,
        stream_name: logical.units[logical.last_dot + 1..].into(),
        storage_prefix_units: logical.last_dot,
        stream_parent_offset: logical.last_dot + 1,
        completion: SimulationStorageLookupCompletion::CompletedLexicalProjection,
        check_mode: SimulationStorageLookupCheckMode::ExactLastDotUtf16LexicalProjection,
        checked_input_units: logical.units.len(),
        storage_mode: 0x10,
        storage_format: 0,
        stream_mode: 0x10,
        payload_decoded: false,
        native_io_executed: false,
        native_name_acceptance_established: false,
        requested_pattern_error_bits: 0,
    }
}
