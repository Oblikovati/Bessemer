//! Electrical project ZIP32 carriers and lossless ASCII Archive.ini lexical records.
mod carrier;
mod edit;
mod file_fields;
mod ini;
mod projects;
mod types;
mod userdata;
mod userdata_connection;
mod userdata_state;
mod userdata_write;
use crate::composer::{ComposerArchive, ComposerCompression, ComposerMember};
use crate::{BessemerError, FailureClass, Result};
pub use file_fields::*;
pub use ini::*;
pub use projects::*;
pub use types::*;
pub use userdata::*;
pub use userdata_state::*;
pub use userdata_write::*;

/// Immutable complete Electrical ZIP carrier; member payload schemas remain separate.
#[derive(Clone, Debug)]
pub struct ElectricalArchive {
    backend: ComposerArchive,
    limits: ElectricalLimits,
}

impl ElectricalArchive {
    /// Checks every member before publication; e.g. `ElectricalArchive::from_bytes(&raw, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: ElectricalLimits) -> Result<Self> {
        carrier::read(bytes, limits)
    }
    /// Enumerates directory order including duplicates; e.g. `let members = archive.members().collect::<Vec<_>>()`.
    pub fn members(&self) -> impl ExactSizeIterator<Item = ElectricalMember> + '_ {
        self.backend.members().map(electrical_member)
    }
    /// Resolves exactly one occurrence; e.g. `let id = archive.member_named("Archive.ini")?`.
    pub fn member_named(&self, name: &str) -> Result<ElectricalMemberId> {
        self.backend
            .member_named(name)
            .map(ElectricalMemberId)
            .map_err(electrical_error)
    }
    /// Borrows complete checked payload bytes; e.g. `let raw = archive.read_member(&id)?`.
    pub fn read_member(&self, id: &ElectricalMemberId) -> Result<&[u8]> {
        self.backend.read_member(&id.0).map_err(electrical_error)
    }
    /// Borrows retained ZIP flags/offset/extra/comment metadata; e.g. `let metadata = archive.member_metadata(&id)?`.
    pub fn member_metadata(&self, id: &ElectricalMemberId) -> Result<ElectricalMemberMetadata<'_>> {
        let metadata = self.backend.member_wire(&id.0).map_err(electrical_error)?;
        Ok(ElectricalMemberMetadata {
            flags: metadata.flags,
            header_offset: metadata.local_offset,
            extra: metadata.extra,
            comment: metadata.comment,
        })
    }
    /// Borrows the exact archive comment; e.g. `let comment = archive.comment()`.
    pub fn comment(&self) -> &[u8] {
        self.backend.comment()
    }
    /// Reconstructs only the original plain ZIP bytes; e.g. `assert_eq!(archive.to_bytes(), raw)`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.backend.to_bytes()[3..].to_vec()
    }
    /// Reports complete carrier integrity checks; e.g. `let checked = archive.coverage()`.
    pub fn coverage(&self) -> ElectricalCoverage {
        let checked = self.backend.coverage();
        ElectricalCoverage {
            mode: ElectricalCheckMode::ExactZipCarrier,
            wire_bytes: checked.wire_bytes - 3,
            checked_members: checked.checked_members,
            expanded_bytes: checked.expanded_bytes,
            achieved_byte_error: 0,
        }
    }
    /// Borrows explicit payload/application exclusions; e.g. `let exclusions = archive.diagnostics()`.
    pub fn diagnostics(&self) -> &[ElectricalDiagnostic] {
        &[
            ElectricalDiagnostic::MemberSchemasOutsideScope,
            ElectricalDiagnostic::NativeApplicationNotChecked,
        ]
    }
    /// Reads one exact Archive.ini occurrence; non-ASCII data stays opaque; e.g. `let view = archive.archive_ini(&id)?`.
    pub fn archive_ini(&self, id: &ElectricalMemberId) -> Result<Option<ElectricalIniDocument>> {
        let raw = self.read_member(id)?;
        require_ini_name(self, id)?;
        ElectricalIniDocument::from_bytes(
            raw,
            ElectricalIniLimits {
                maximum_bytes: self.limits.maximum_member_bytes,
                maximum_records: self.limits.maximum_ini_records,
            },
        )
    }
    /// Checks one complete SQLite member in memory; e.g. `let database = archive.sqlite_member(&id, limits)?`.
    pub fn sqlite_member(
        &self,
        id: &ElectricalMemberId,
        limits: crate::sqlite_image::SqliteLimits,
    ) -> Result<crate::sqlite_image::SqliteImage> {
        crate::sqlite_image::SqliteImage::from_bytes(self.read_member(id)?, limits)
    }
}

fn electrical_member(member: ComposerMember) -> ElectricalMember {
    let compression = match member.compression {
        ComposerCompression::Stored => ElectricalCompression::Stored,
        ComposerCompression::Deflated => ElectricalCompression::Deflated,
    };
    ElectricalMember {
        id: ElectricalMemberId(member.id),
        name: member.name,
        compression,
        expanded_bytes: member.expanded_bytes,
        stored_bytes: member.stored_bytes,
        crc32: member.crc32,
    }
}

fn require_ini_name(archive: &ElectricalArchive, id: &ElectricalMemberId) -> Result<()> {
    let member = archive
        .members()
        .find(|member| member.id == *id)
        .ok_or_else(|| {
            failure(
                FailureClass::Missing,
                "INI occurrence",
                "identity absent; expected current member",
            )
        })?;
    check_ini_spelling(&member.name)
}

fn check_ini_spelling(name: &str) -> Result<()> {
    if name != "Archive.ini" {
        return Err(failure(
            FailureClass::Unsupported,
            name,
            format!("member {:?}; expected exact Archive.ini spelling", name),
        ));
    }
    Ok(())
}

fn failure(class: FailureClass, entity: &str, cause: impl Into<String>) -> BessemerError {
    BessemerError::new("electrical archive", entity, class, cause)
}

fn electrical_error(mut error: BessemerError) -> BessemerError {
    error.operation = "electrical archive";
    error
}
