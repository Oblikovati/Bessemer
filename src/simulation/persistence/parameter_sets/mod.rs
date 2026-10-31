use super::{Result, SimulationPersistText, SimulationPersistVersion, invalid, require};
use std::sync::Arc;
mod construction;
mod decode;
mod edit;
mod encode;
mod layout;
mod parts;
mod types;
pub use parts::*;
pub use types::*;

/// Complete bounded ParameterSets archive with immutable owned parts and primitive identities.
#[derive(Debug, Clone)]
pub struct SimulationParameterSets {
    bytes: Arc<[u8]>,
    parts: Arc<SimulationParameterParts>,
    fields: Arc<[SimulationParameterField]>,
    revision: SimulationParameterRevision,
    version: SimulationPersistVersion,
    limits: SimulationParameterLimits,
}

impl SimulationParameterSets {
    /// Decode the complete explicit profile, e.g. `SimulationParameterSets::from_bytes(bytes, version, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        limits: SimulationParameterLimits,
    ) -> Result<Self> {
        let gates = layout::ParameterGates::new(version);
        let scan = decode::scan(bytes, gates, limits)?;
        let parent: Arc<[u8]> = bytes.into();
        let parts = scan.materialize(parent.clone(), limits)?;
        let revision = SimulationParameterRevision(Default::default());
        let encoded = encode::encode(&parts, gates, limits, revision.clone())?;
        verify_reconstruction(&encoded.bytes, &parent)?;
        Ok(Self {
            bytes: parent,
            parts: Arc::new(parts),
            fields: encoded.fields.into(),
            revision,
            version,
            limits,
        })
    }
    /// Construct complete matching tables/records, e.g. `SimulationParameterSets::from_parts(&parts, version, limits)?`.
    pub fn from_parts(
        parts: &SimulationParameterParts,
        version: SimulationPersistVersion,
        limits: SimulationParameterLimits,
    ) -> Result<Self> {
        let revision = SimulationParameterRevision(Default::default());
        let encoded = encode::encode(
            parts,
            layout::ParameterGates::new(version),
            limits,
            revision,
        )?;
        Self::from_bytes(&encoded.bytes, version, limits)
    }
    /// Complete exact bare-stream bytes, e.g. `archive.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Immutable complete construction parts, e.g. `archive.parts()`.
    pub fn parts(&self) -> &SimulationParameterParts {
        &self.parts
    }
    /// Explicit normalized version, e.g. `archive.version()`.
    pub fn version(&self) -> SimulationPersistVersion {
        self.version
    }
    /// Original explicit budgets, e.g. `archive.limits()`.
    pub fn limits(&self) -> SimulationParameterLimits {
        self.limits
    }
    /// Complete immutable archive identity, e.g. `archive.revision()`.
    pub fn revision(&self) -> &SimulationParameterRevision {
        &self.revision
    }
    /// Every primitive and complete text in total wire order, e.g. `archive.fields()`.
    pub fn fields(&self) -> &[SimulationParameterField] {
        &self.fields
    }
    /// Normalize an actual physical field, e.g. `archive.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationParameterFieldId> {
        self.fields
            .get(ordinal)
            .map(|field| field.id.clone())
            .ok_or_else(|| {
                invalid(
                    "ParameterSets field ordinal",
                    format!("{ordinal}; expected 0..{}", self.fields.len()),
                )
            })
    }
    /// Resolve an original normalized field, e.g. `archive.field(&id)?`.
    pub fn field(&self, id: &SimulationParameterFieldId) -> Result<&SimulationParameterField> {
        require(
            id.revision == self.revision,
            "ParameterSets field revision",
            format!(
                "foreign revision at ordinal {}; expected this exact ParameterSets revision",
                id.ordinal
            ),
        )?;
        self.fields.get(id.ordinal).ok_or_else(|| {
            invalid(
                "ParameterSets field ordinal",
                format!("{}; expected 0..{}", id.ordinal, self.fields.len()),
            )
        })
    }
    /// Complete exact byte coverage and exclusions, e.g. `archive.coverage()`.
    pub fn coverage(&self) -> SimulationParameterCoverage {
        parameter_coverage(self)
    }
}

fn verify_reconstruction(encoded: &[u8], source: &[u8]) -> Result<()> {
    require(
        encoded == source,
        "ParameterSets reconstruction",
        format!(
            "{} reconstructed bytes, {} original bytes; expected an identical complete byte image",
            encoded.len(),
            source.len()
        ),
    )
}

fn parameter_coverage(archive: &SimulationParameterSets) -> SimulationParameterCoverage {
    SimulationParameterCoverage {
        version: archive.version,
        limits: archive.limits,
        completion: read_completion(archive.parts.records.len()),
        check_mode: SimulationParameterCheckMode::ExactSelectedProfile,
        checked_bytes: archive.bytes.len(),
        checked_fields: archive.fields.len(),
        table_count: archive.parts.table.len(),
        record_count: archive.parts.records.len(),
        checked_text_frames: archive.parts.records.len(),
        diagnostics: parameter_diagnostics(),
        requested_pattern_error_bits: 0,
    }
}

fn read_completion(records: usize) -> SimulationParameterCompletion {
    if records == 0 {
        SimulationParameterCompletion::CompletedEmptyRecords
    } else {
        SimulationParameterCompletion::CompletedRecords
    }
}

fn parameter_diagnostics() -> [SimulationParameterDiagnostic; 5] {
    use SimulationParameterDiagnostic::*;
    [
        NativeApplicationNotExecuted,
        RuntimeVersionNotInferred,
        LegacyFallbackNotUsed,
        NativeRetentionNotApplied,
        FieldMeaningsUnassigned,
    ]
}
