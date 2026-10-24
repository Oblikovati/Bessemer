use crate::archive_codec::ArchiveString;

/// A stored path and the writer's associated drive observations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentPath {
    /// Raw CString, possibly empty or the repeated-string marker `*`.
    pub path: ArchiveString,
    /// Raw `GetDriveTypeW` result; unrecognized values remain unchanged.
    pub drive_type: i32,
    /// Stored bytes available to the writer's user, without re-querying the drive.
    pub available_disk_bytes: u64,
}

/// Direct `moStartEndRecord_c` body with six fixed, ordered path groups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentSnapshot {
    /// FILETIME ticks: 100 ns intervals since 1601-01-01 UTC, retained without conversion.
    pub filetime: u64,
    /// Low 32 bits of the writer's available commit-memory observation.
    pub available_page_file_low32: u32,
    /// System, Windows, current, TEMP, TMP, and document paths, in that order.
    pub paths: [EnvironmentPath; 6],
}

/// Direct biography record with raw fields and two independent environment snapshots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiographyRecord {
    /// Stored product file version, independent of the enclosing archive directory.
    pub file_version: u32,
    /// Product build date code, not a save timestamp.
    pub date_code: i32,
    /// Uninterpreted native member +0x10.
    pub unknown_10: i32,
    /// Stored processor architecture code.
    pub processor_architecture: u32,
    /// Stored processor count, including any default or sentinel value.
    pub processor_count: u32,
    /// Platform ID reported to the writer by `GetVersionExW`.
    pub os_platform_id: u32,
    /// Reported OS major version.
    pub os_major: u32,
    /// Reported OS minor version.
    pub os_minor: u32,
    /// Reported OS build.
    pub os_build: u32,
    /// Raw CSD string, supporting the repeated-string marker `*`.
    pub os_csd_version: ArchiveString,
    /// Stored language identifier, without normalization.
    pub language_id: u16,
    /// Load and save snapshots; corresponding fields inherit only across records.
    pub snapshots: [EnvironmentSnapshot; 2],
}

impl BiographyRecord {
    pub(super) fn strings(&self) -> impl Iterator<Item = &ArchiveString> {
        std::iter::once(&self.os_csd_version).chain(
            self.snapshots
                .iter()
                .flat_map(|snapshot| snapshot.paths.iter().map(|group| &group.path)),
        )
    }

    pub(super) fn strings_mut(&mut self) -> impl Iterator<Item = &mut ArchiveString> {
        std::iter::once(&mut self.os_csd_version).chain(
            self.snapshots
                .iter_mut()
                .flat_map(|snapshot| snapshot.paths.iter_mut().map(|group| &mut group.path)),
        )
    }
}
