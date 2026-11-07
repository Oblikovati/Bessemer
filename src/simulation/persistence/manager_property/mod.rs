use super::{Result, SimulationPersistVersion, invalid, require};
use crate::records::dword;
use std::sync::Arc;
mod edit;
mod profile;
mod types;
pub use profile::*;
pub use types::*;

/// Complete direct Beam/Mass manager header with immutable bytes and explicit context.
#[derive(Debug, Clone)]
pub struct SimulationManagerProperty {
    bytes: Arc<[u8]>,
    fields: Arc<[SimulationManagerPropertyField]>,
    revision: SimulationManagerPropertyRevision,
    profile: SimulationManagerPropertyProfile,
}

impl SimulationManagerProperty {
    /// Require the exact selected header, e.g. `SimulationManagerProperty::from_bytes(bytes, profile)?`.
    pub fn from_bytes(bytes: &[u8], profile: SimulationManagerPropertyProfile) -> Result<Self> {
        let members = profile.members();
        require_extent(bytes.len(), profile)?;
        let revision = SimulationManagerPropertyRevision(Default::default());
        let fields = members
            .iter()
            .enumerate()
            .map(|(ordinal, member)| decode_field(bytes, &revision, ordinal, *member))
            .collect::<Result<Arc<[_]>>>()?;
        Ok(Self {
            bytes: bytes.into(),
            fields,
            revision,
            profile,
        })
    }
    /// Construct all selected signed words together, e.g. `SimulationManagerProperty::from_values(&[-1], profile)?`.
    pub fn from_values(values: &[i32], profile: SimulationManagerPropertyProfile) -> Result<Self> {
        require(
            values.len() == profile.members().len(),
            "manager Property values",
            format!(
                "{} words for {profile:?}; expected exactly {} words",
                values.len(),
                profile.members().len()
            ),
        )?;
        let bytes: Vec<u8> = values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect();
        Self::from_bytes(&bytes, profile)
    }
    /// Entire exact bare header, e.g. `header.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Original supplied kind and optional version, e.g. `header.profile()`.
    pub fn profile(&self) -> SimulationManagerPropertyProfile {
        self.profile
    }
    /// Immutable complete header identity, e.g. `header.revision()`.
    pub fn revision(&self) -> &SimulationManagerPropertyRevision {
        &self.revision
    }
    /// Every stored word in wire order, e.g. `header.fields()`.
    pub fn fields(&self) -> &[SimulationManagerPropertyField] {
        &self.fields
    }
    /// Normalize an existing physical position, e.g. `header.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationManagerPropertyFieldId> {
        self.fields
            .get(ordinal)
            .map(|field| field.id.clone())
            .ok_or_else(|| {
                invalid(
                    "manager Property field ordinal",
                    format!("{ordinal}; expected 0..{}", self.fields.len()),
                )
            })
    }
    /// Resolve only an original normalized field, e.g. `header.field(&id)?`.
    pub fn field(
        &self,
        id: &SimulationManagerPropertyFieldId,
    ) -> Result<&SimulationManagerPropertyField> {
        require(
            id.revision == self.revision,
            "manager Property field revision",
            format!(
                "foreign revision at ordinal {}; expected this exact header revision",
                id.ordinal
            ),
        )?;
        self.fields.get(id.ordinal).ok_or_else(|| {
            invalid(
                "manager Property field ordinal",
                format!("{}; expected 0..{}", id.ordinal, self.fields.len()),
            )
        })
    }
    /// Complete exact header scope and exclusions, e.g. `header.coverage()`.
    pub fn coverage(&self) -> SimulationManagerPropertyCoverage {
        SimulationManagerPropertyCoverage {
            profile: self.profile,
            completion: SimulationManagerPropertyCompletion::CompletedFraming,
            check_mode: SimulationManagerPropertyCheckMode::ExactProfile,
            checked_bytes: self.bytes.len(),
            checked_fields: self.fields.len(),
            diagnostics: [
                SimulationManagerPropertyDiagnostic::NativeApplicationNotExecuted,
                SimulationManagerPropertyDiagnostic::RuntimeDispatchNotInferred,
                SimulationManagerPropertyDiagnostic::FieldMeaningsUnassigned,
                SimulationManagerPropertyDiagnostic::NestedObjectsNotDecoded,
            ],
            requested_pattern_error_bits: 0,
        }
    }
}

fn require_extent(length: usize, profile: SimulationManagerPropertyProfile) -> Result<()> {
    require(
        length == profile.members().len() * 4,
        "manager Property extent",
        format!(
            "{length} bytes for {profile:?}; expected exactly {} bytes",
            profile.members().len() * 4
        ),
    )
}

fn decode_field(
    bytes: &[u8],
    revision: &SimulationManagerPropertyRevision,
    ordinal: usize,
    member: SimulationManagerPropertyMember,
) -> Result<SimulationManagerPropertyField> {
    Ok(SimulationManagerPropertyField {
        id: SimulationManagerPropertyFieldId {
            revision: revision.clone(),
            ordinal,
        },
        stream_offset: ordinal * 4,
        member,
        value: dword(bytes, ordinal * 4)? as i32,
    })
}
