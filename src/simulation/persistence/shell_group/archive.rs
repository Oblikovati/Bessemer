use super::{
    SimulationShellGroupCoverage, SimulationShellGroupField, SimulationShellGroupFieldId,
    SimulationShellGroupLimits, decode, fields,
};
use crate::{
    Result,
    archive_codec::ArchiveRevision,
    simulation::{SimulationPersistVersion, SimulationShellWireProfile},
};
use std::sync::Arc;

/// Immutable complete direct group records with all original MFC string frames retained.
#[derive(Debug, Clone)]
pub struct SimulationShellGroup {
    pub(super) parent: Arc<[u8]>,
    pub(super) revision: ArchiveRevision,
    pub(super) fields: Arc<[SimulationShellGroupField]>,
    pub(super) offsets: Arc<[(usize, usize)]>,
    pub(super) version: SimulationPersistVersion,
    pub(super) profile: SimulationShellWireProfile,
    pub(super) limits: SimulationShellGroupLimits,
    pub(super) entries: usize,
    pub(super) text_bytes: usize,
}

impl SimulationShellGroup {
    /// Read all declared direct records through mandatory EOF; e.g. `Self::from_bytes(bytes, version, profile, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        version: SimulationPersistVersion,
        profile: SimulationShellWireProfile,
        limits: SimulationShellGroupLimits,
    ) -> Result<Self> {
        let reader = decode::read(bytes, version, profile, limits)?;
        Ok(Self {
            parent: bytes.into(),
            revision: Default::default(),
            fields: reader.fields.into(),
            offsets: reader.offsets.into(),
            version,
            profile,
            limits,
            entries: reader.entries,
            text_bytes: reader.text_bytes,
        })
    }

    /// Construct the complete matching shape and revalidate every frame; e.g. `Self::from_fields(source.fields().to_vec(), version, profile, limits)?`.
    pub fn from_fields(
        fields: Vec<SimulationShellGroupField>,
        version: SimulationPersistVersion,
        profile: SimulationShellWireProfile,
        limits: SimulationShellGroupLimits,
    ) -> Result<Self> {
        let bytes = fields::encode_fields(&fields, limits)?;
        let group = Self::from_bytes(&bytes, version, profile, limits)?;
        if group.fields() != fields {
            return Err(super::super::thermal::construction_mismatch(
                &fields,
                group.fields(),
                "ShellManagerGroup construction",
            ));
        }
        Ok(group)
    }

    /// Borrow every immutable complete primitive occurrence; e.g. `group.fields()`.
    pub fn fields(&self) -> &[SimulationShellGroupField] {
        &self.fields
    }

    /// Return each body-relative stored span; e.g. `group.field_offsets()`.
    pub fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }

    /// Return the supplied normalized version; e.g. `group.version()`.
    pub fn version(&self) -> SimulationPersistVersion {
        self.version
    }

    /// Return the explicit late-field grammar without changing string encodings; e.g. `group.wire_profile()`.
    pub fn wire_profile(&self) -> SimulationShellWireProfile {
        self.profile
    }

    /// Reconstruct every retained byte without locale or floating-point normalization; e.g. `group.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.parent.to_vec()
    }

    /// Bind one physical occurrence to this immutable revision; e.g. `group.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationShellGroupFieldId> {
        super::super::load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "ShellManagerGroup",
        )
    }

    /// Resolve a field only on its owning revision; e.g. `group.field(&id)?`.
    pub fn field(&self, id: &SimulationShellGroupFieldId) -> Result<&SimulationShellGroupField> {
        super::super::load_identity::field(&self.fields, &self.revision, id, "ShellManagerGroup")
    }

    /// Return exact completion, budgets, exclusions and achieved pattern error; e.g. `group.coverage().record_count`.
    pub fn coverage(&self) -> SimulationShellGroupCoverage {
        super::types::coverage(self)
    }
}
