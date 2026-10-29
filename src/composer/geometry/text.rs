use super::{Result, invalid};
use std::sync::Arc;

/// The two documented terminated UTF-16 text endings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerTextTerminator {
    /// UTF-16 NUL.
    Nul,
    /// UTF-16 line feed.
    Newline,
}

impl ComposerTextTerminator {
    pub(super) fn word(self) -> u16 {
        match self {
            Self::Nul => 0,
            Self::Newline => 10,
        }
    }
}

/// Immutable raw UTF-16 code units, including unpaired surrogates, and the original ending.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerSceneText {
    pub(super) units: Arc<[u16]>,
    pub(super) terminator: ComposerTextTerminator,
}

impl ComposerSceneText {
    /// Create exact raw text, e.g. `ComposerSceneText::from_units(&[0xd800], ComposerTextTerminator::Nul)?`.
    pub fn from_units(units: &[u16], terminator: ComposerTextTerminator) -> Result<Self> {
        if let Some(unit) = units.iter().find(|unit| **unit == 0 || **unit == 10) {
            return Err(invalid(format!(
                "embedded text unit {unit}; expected content without NUL/line-feed terminators"
            )));
        }
        Ok(Self {
            units: units.into(),
            terminator,
        })
    }

    /// Encode Unicode content, e.g. `ComposerSceneText::from_text("name Ω", ComposerTextTerminator::Newline)?`.
    pub fn from_text(value: &str, terminator: ComposerTextTerminator) -> Result<Self> {
        Self::from_units(&value.encode_utf16().collect::<Vec<_>>(), terminator)
    }

    /// Exact stored content units, e.g. `text.code_units()`.
    pub fn code_units(&self) -> &[u16] {
        &self.units
    }

    /// Original ending, e.g. `text.terminator()`.
    pub fn terminator(&self) -> ComposerTextTerminator {
        self.terminator
    }

    /// Decode well-formed content without replacing invalid units, e.g. `text.to_text()?`.
    pub fn to_text(&self) -> Result<String> {
        String::from_utf16(&self.units).map_err(|error| {
            invalid(format!(
                "text units {:?}: {error}; expected well-formed UTF-16 for a Unicode string",
                self.units
            ))
        })
    }
}
