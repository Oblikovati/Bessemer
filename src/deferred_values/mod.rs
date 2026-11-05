//! Conditional installed deferred-value MsgPack grammar (9.2 / C-3590).
//! Profiles declare control state; this module does not assign a format to arbitrary database values.
mod decode;
mod encode;
mod types;
mod wire;
use crate::{BessemerError, FailureClass, Result};
pub use types::*;

/// Immutable complete selected writer-form blob and its owned ordered value tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeferredValueDocument {
    profile: DeferredValueProfile,
    value: DeferredValue,
    bytes: Box<[u8]>,
    coverage: DeferredValueCoverage,
}

impl DeferredValueDocument {
    /// Reads through EOF and verifies the installed writer form; e.g. `DeferredValueDocument::from_bytes(&[0xc0], profile, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        profile: DeferredValueProfile,
        limits: DeferredValueLimits,
    ) -> Result<Self> {
        let (value, coverage) = decode::document(bytes, profile, limits)?;
        let canonical = encode::document(&value, profile, limits)?;
        require_writer_form(bytes, &canonical)?;
        Ok(Self {
            profile,
            value,
            bytes: bytes.into(),
            coverage,
        })
    }
    /// Constructs isolated bytes from a borrowed value; e.g. `DeferredValueDocument::from_value(&DeferredValue::Null, profile, limits)?`.
    pub fn from_value(
        value: &DeferredValue,
        profile: DeferredValueProfile,
        limits: DeferredValueLimits,
    ) -> Result<Self> {
        let bytes = encode::document(value, profile, limits)?;
        Self::from_bytes(&bytes, profile, limits)
    }
    /// Borrows the immutable ordered tree; e.g. `let value = document.value()`.
    pub fn value(&self) -> &DeferredValue {
        &self.value
    }
    /// Returns the explicit byte-order profile; e.g. `let selected = document.profile()`.
    pub fn profile(&self) -> DeferredValueProfile {
        self.profile
    }
    /// Reports complete syntax coverage; e.g. `let count = document.coverage().values`.
    pub fn coverage(&self) -> DeferredValueCoverage {
        self.coverage
    }
    /// Reconstructs all accepted bytes; e.g. `assert_eq!(document.to_bytes(), original)`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}

fn require_writer_form(actual: &[u8], expected: &[u8]) -> Result<()> {
    if actual == expected {
        return Ok(());
    }
    let offset = actual
        .iter()
        .zip(expected)
        .position(|(a, b)| a != b)
        .unwrap_or(actual.len().min(expected.len()));
    Err(unsupported(format!(
        "encoding at {offset}: actual {:?}, expected {:?}; expected selected installed minimal writer form",
        actual.get(offset),
        expected.get(offset)
    )))
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "deferred value",
        "selected blob",
        FailureClass::Invalid,
        cause,
    )
}

fn unsupported(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "deferred value",
        "selected writer profile",
        FailureClass::Unsupported,
        cause,
    )
}
