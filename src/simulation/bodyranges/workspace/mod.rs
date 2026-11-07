mod allocation;
mod composed;
mod normalization;
mod outcome;
use super::*;
pub use allocation::*;
pub use composed::*;
pub use normalization::*;
pub use outcome::*;

/// Identity of an immutable 288-byte memory image, never a portable pointer owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationAuxiliaryWorkspaceRevision(ArchiveRevision);

/// Exact normalized/supplied memory image; slot bits are never dereferenced.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryWorkspace {
    bytes: Arc<[u8]>,
    revision: SimulationAuxiliaryWorkspaceRevision,
    gen_parent: Option<(SimulationGenRevision, usize)>,
    trial_source_offset: Option<usize>,
}

impl SimulationAuxiliaryWorkspace {
    /// Own an exact supplied image, e.g. `SimulationAuxiliaryWorkspace::from_bytes(initial)?`; slot bits are not usable addresses.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        require(
            bytes.len() == 288,
            "auxiliary workspace",
            format!(
                "{} bytes; expected exactly 288 supplied memory-image bytes",
                bytes.len()
            ),
        )?;
        Ok(Self {
            bytes: bytes.into(),
            revision: SimulationAuxiliaryWorkspaceRevision(ArchiveRevision::default()),
            gen_parent: None,
            trial_source_offset: None,
        })
    }
    /// Exact memory-image bytes, e.g. `workspace.to_bytes()`; not a GEN file write.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Immutable memory-image identity, e.g. `workspace.revision()`.
    pub fn revision(&self) -> &SimulationAuxiliaryWorkspaceRevision {
        &self.revision
    }
    /// Original GEN/logical-row copy parent, e.g. `workspace.gen_parent()`; not a portable stored pointer.
    pub fn gen_parent(&self) -> Option<(&SimulationGenRevision, usize)> {
        self.gen_parent
            .as_ref()
            .map(|(revision, index)| (revision, *index))
    }
    /// Original full-GEN trial read offset, if known, e.g. `workspace.trial_source_offset()`.
    pub fn trial_source_offset(&self) -> Option<usize> {
        self.trial_source_offset
    }
    /// Exact numeric slot pattern, e.g. `workspace.pointer_slot_bits()`; never interpreted as a dereferenceable pointer.
    pub fn pointer_slot_bits(&self) -> u64 {
        u64::from_le_bytes(std::array::from_fn(|i| self.bytes[8 + i]))
    }
}

impl SimulationGenArchive {
    /// Capture the original 288-byte trial slice, e.g. `image.auxiliary_trial_workspace(0)?`; legacy stored offsets are not substituted.
    pub fn auxiliary_trial_workspace(&self, index: usize) -> Result<SimulationAuxiliaryWorkspace> {
        let (bytes, offset) = self.auxiliary_trial_slice(index)?;
        let mut workspace = SimulationAuxiliaryWorkspace::from_bytes(bytes)?;
        workspace.gen_parent = Some((self.revision().clone(), index));
        workspace.trial_source_offset = Some(offset);
        Ok(workspace)
    }
}
