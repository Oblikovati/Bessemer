//! Explicit version-selected LBC Property and independently recovered fixed writer framing.
mod edit;
mod layout;
mod types;
use super::{Result, invalid, require};
use crate::archive_codec::ArchiveRevision;
pub use edit::*;
pub use layout::{SimulationLbcPropertyProfile, SimulationLbcRuntimeMember};
use std::sync::Arc;
pub use types::*;

/// Complete immutable binary LBCManager/Property image with explicitly selected context.
#[derive(Debug, Clone)]
pub struct SimulationLbcProperty {
    pub(super) bytes: Arc<[u8]>,
    pub(super) fields: Arc<[SimulationLbcPropertyField]>,
    pub(super) profile: SimulationLbcPropertyProfile,
    pub(super) revision: SimulationLbcPropertyRevision,
}

impl SimulationLbcProperty {
    /// Require the exact supplied profile and EOF, e.g. `SimulationLbcProperty::from_bytes(bytes, profile)?`.
    pub fn from_bytes(bytes: &[u8], profile: SimulationLbcPropertyProfile) -> Result<Self> {
        let members = layout::member_order(profile);
        require(
            bytes.len() == members.len() * 4,
            "LBC Property profile extent",
            format!(
                "{} bytes for {profile:?}; expected exactly {} bytes",
                bytes.len(),
                members.len() * 4
            ),
        )?;
        let revision = SimulationLbcPropertyRevision(ArchiveRevision::default());
        let fields = field_directory(bytes, &members, &revision);
        Ok(Self {
            bytes: bytes.into(),
            fields,
            profile,
            revision,
        })
    }

    /// Construct all ordered words under an explicit profile, e.g. `SimulationLbcProperty::from_values(&values, profile)?`; no native writer executes.
    pub fn from_values(values: &[i32], profile: SimulationLbcPropertyProfile) -> Result<Self> {
        let expected = layout::member_order(profile).len();
        require(
            values.len() == expected,
            "LBC Property field count",
            format!(
                "{} values for {profile:?}; expected exactly {expected}",
                values.len()
            ),
        )?;
        let bytes: Vec<u8> = values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect();
        Self::from_bytes(&bytes, profile)
    }

    /// Exact complete bare carrier bytes, e.g. `property.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Original explicit supplied context, e.g. `property.profile()`.
    pub fn profile(&self) -> SimulationLbcPropertyProfile {
        self.profile
    }
    /// Immutable image/profile identity, e.g. `property.revision()`.
    pub fn revision(&self) -> &SimulationLbcPropertyRevision {
        &self.revision
    }
    /// All stored words in original wire order, e.g. `property.fields()`.
    pub fn fields(&self) -> &[SimulationLbcPropertyField] {
        &self.fields
    }
    /// Normalize an actual zero-based stored field, e.g. `property.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationLbcPropertyFieldId> {
        self.fields
            .get(ordinal)
            .map(|field| field.id.clone())
            .ok_or_else(|| {
                invalid(
                    "LBC Property field ordinal",
                    format!("{ordinal}; expected below {}", self.fields.len()),
                )
            })
    }
    /// Resolve a normalized ID on its original image, e.g. `property.field(&id)?`.
    pub fn field(&self, id: &SimulationLbcPropertyFieldId) -> Result<&SimulationLbcPropertyField> {
        require(
            id.revision == self.revision,
            "LBC Property field revision",
            format!(
                "foreign revision at field {}; expected this exact image/profile revision",
                id.ordinal
            ),
        )?;
        self.fields.get(id.ordinal).ok_or_else(|| {
            invalid(
                "LBC Property field ordinal",
                format!("{}; expected below {}", id.ordinal, self.fields.len()),
            )
        })
    }
    /// Exact complete selected-profile scope and explicit exclusions, e.g. `property.coverage()`.
    pub fn coverage(&self) -> SimulationLbcPropertyCoverage {
        SimulationLbcPropertyCoverage {
            profile: self.profile,
            check_mode: SimulationLbcPropertyCheckMode::ExactSelectedProfile,
            checked_bytes: self.bytes.len(),
            checked_fields: self.fields.len(),
            completion: SimulationLbcReadCompletion::CompletedFraming,
            diagnostics: [
                SimulationLbcPropertyDiagnostic::NativeApplicationNotExecuted,
                SimulationLbcPropertyDiagnostic::FieldMeaningsUnassigned,
                SimulationLbcPropertyDiagnostic::RuntimeVersionNotInferred,
            ],
            requested_pattern_error_bits: 0,
        }
    }
}

fn field_directory(
    bytes: &[u8],
    members: &[SimulationLbcRuntimeMember],
    revision: &SimulationLbcPropertyRevision,
) -> Arc<[SimulationLbcPropertyField]> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .zip(members.iter())
        .enumerate()
        .map(|(ordinal, (word, member))| SimulationLbcPropertyField {
            id: SimulationLbcPropertyFieldId {
                revision: revision.clone(),
                ordinal,
            },
            stream_offset: ordinal * 4,
            member: *member,
            value: i32::from_le_bytes(*word),
        })
        .collect()
}
