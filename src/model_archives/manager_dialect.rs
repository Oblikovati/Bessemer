use crate::header::ArchiveVersion;
use crate::{BessemerError, FailureClass, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ManagerDialect {
    Current19000,
    Owned18000,
}

impl ManagerDialect {
    /// Normalize only the independently reviewed configuration-manager versions.
    /// Example: `ManagerDialect::from_version(ArchiveVersion::new(18000))?`.
    pub(super) fn from_version(version: ArchiveVersion) -> Result<Self> {
        match version.value() {
            19000 => Ok(Self::Current19000),
            18000 => Ok(Self::Owned18000),
            value => Err(BessemerError::new(
                "configuration context",
                "Contents/CMgr",
                FailureClass::Unsupported,
                format!(
                    "configuration version {value}; expected explicitly scoped version 18000 or 19000"
                ),
            )),
        }
    }

    /// Return the exact archive version required by this selected dialect.
    /// Example: `ManagerDialect::Owned18000.version()` returns 18000.
    pub(super) fn version(self) -> u32 {
        match self {
            Self::Current19000 => 19000,
            Self::Owned18000 => 18000,
        }
    }
}
