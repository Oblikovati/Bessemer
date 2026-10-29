//! Complete SMG-prefixed ZIP32 containers; member payload schemas remain separate.
pub mod arrays;
mod decode;
mod encode;
pub mod geometry;
pub mod links;
mod names;
mod payload;
mod types;

use crate::archive_codec::ArchiveRevision;
use crate::{BessemerError, FailureClass, Result};
use std::sync::Arc;
pub use types::{
    ComposerCheckMode, ComposerCompression, ComposerCoverage, ComposerDiagnostic, ComposerEdit,
    ComposerEditCompletion, ComposerEditFidelity, ComposerLimits, ComposerMember, ComposerMemberId,
    ComposerMemberInput,
};

/// Immutable, completely framed and CRC-checked Composer container.
#[derive(Debug, Clone)]
pub struct ComposerArchive {
    original: Arc<[u8]>,
    directory: Arc<decode::ComposerDirectory>,
    expanded: Arc<[Arc<[u8]>]>,
    revision: ArchiveRevision,
    limits: ComposerLimits,
}

impl ComposerArchive {
    /// Validate every member before publication, e.g. `ComposerArchive::from_bytes(bytes, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: ComposerLimits) -> Result<Self> {
        let directory = decode::decode_composer_directory(bytes, limits).map_err(composer_error)?;
        let expanded = payload::expand_members(bytes, &directory.members, limits)?;
        let archive = Self {
            original: Arc::from(bytes),
            directory: Arc::new(directory),
            expanded: expanded.into(),
            revision: ArchiveRevision::default(),
            limits,
        };
        if encode::serialize_composer(&archive, None)? != bytes {
            return Err(failure(
                FailureClass::Invalid,
                "construction",
                "reconstructed bytes differ; expected exact complete ZIP32 reconstruction",
            ));
        }
        Ok(archive)
    }

    /// Create a new carrier, e.g. `ComposerArchive::from_members(members, b"comment", limits)?`.
    pub fn from_members(
        members: Vec<ComposerMemberInput>,
        comment: &[u8],
        limits: ComposerLimits,
    ) -> Result<Self> {
        let bytes = encode::construct_composer(members, comment, limits)?;
        Self::from_bytes(&bytes, limits)
    }

    /// Enumerate central-directory order, including duplicates, e.g. `archive.members()`.
    pub fn members(&self) -> impl ExactSizeIterator<Item = ComposerMember> + '_ {
        self.directory
            .members
            .iter()
            .enumerate()
            .map(|(index, member)| ComposerMember {
                id: self.member_id(index),
                name: member.name.clone(),
                compression: member.method,
                expanded_bytes: member.expanded,
                stored_bytes: member.stored.len() as u32,
                crc32: member.crc,
            })
    }

    /// Resolve exactly one name, e.g. `let id = archive.member_named("product.smgXml")?;`.
    pub fn member_named(&self, name: &str) -> Result<ComposerMemberId> {
        let mut matches = self.members().filter(|member| member.name.as_ref() == name);
        let first = matches.next().ok_or_else(|| {
            failure(
                FailureClass::Missing,
                name,
                "0 matches; expected exactly one member",
            )
        })?;
        if matches.next().is_some() {
            return Err(failure(
                FailureClass::Invalid,
                name,
                "multiple matches; expected an identity for a duplicate name",
            ));
        }
        Ok(first.id)
    }

    /// Borrow validated payload bytes through an owned identity, e.g. `archive.read_member(&id)?`.
    pub fn read_member(&self, id: &ComposerMemberId) -> Result<&[u8]> {
        Ok(&self.expanded[self.member_index(id)?])
    }

    /// Borrow the raw archive comment, e.g. `archive.comment()`.
    pub fn comment(&self) -> &[u8] {
        &self.original[self.directory.end + 22..]
    }

    /// Preserve all unchanged encoded bytes, e.g. `let bytes = archive.to_bytes();`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.original.to_vec()
    }

    /// Replace one payload on a copy and return checked coverage/history, e.g. `archive.edit_member(&id, replacement)?`.
    pub fn edit_member(&self, id: &ComposerMemberId, replacement: &[u8]) -> Result<ComposerEdit> {
        let index = self.member_index(id)?;
        if self.expanded[index].as_ref() == replacement {
            return Ok(ComposerEdit::unchanged(self.clone()));
        }
        let bytes = encode::replace_composer_member(self, index, replacement)?;
        let archive = Self::from_bytes(&bytes, self.limits)?;
        let result = archive.member_id(index);
        Ok(ComposerEdit::changed(archive, id.clone(), result))
    }

    /// Report complete container checks without payload-schema acceptance, e.g. `archive.coverage()`.
    pub fn coverage(&self) -> ComposerCoverage {
        ComposerCoverage {
            mode: ComposerCheckMode::ExactContainer,
            wire_bytes: self.original.len(),
            checked_members: self.expanded.len(),
            expanded_bytes: self.expanded.iter().map(|bytes| bytes.len()).sum(),
        }
    }

    /// Borrow explicit acceptance exclusions, e.g. `archive.diagnostics()`.
    pub fn diagnostics(&self) -> &[ComposerDiagnostic] {
        &[
            ComposerDiagnostic::MemberSchemasOutsideScope,
            ComposerDiagnostic::NativeApplicationNotChecked,
        ]
    }

    fn member_id(&self, index: usize) -> ComposerMemberId {
        ComposerMemberId {
            revision: self.revision.clone(),
            index,
        }
    }

    fn member_index(&self, id: &ComposerMemberId) -> Result<usize> {
        if id.revision != self.revision || id.index >= self.directory.members.len() {
            return Err(failure(
                FailureClass::Invalid,
                "member identity",
                format!(
                    "index {} in another or missing revision; expected this archive's {} members",
                    id.index,
                    self.directory.members.len()
                ),
            ));
        }
        Ok(id.index)
    }
}

fn failure(class: FailureClass, entity: &str, cause: impl Into<String>) -> BessemerError {
    BessemerError::new("composer archive", entity, class, cause)
}

fn composer_error(mut error: BessemerError) -> BessemerError {
    error.operation = "composer archive";
    error
}
