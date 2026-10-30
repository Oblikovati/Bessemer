use super::{ComposerReferenceSplit, Result, invalid};
use std::sync::Arc;

/// NUL-free UTF-16 units; unpaired surrogates and newlines remain exact.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComposerReferenceText(pub(super) Arc<[u16]>);

impl ComposerReferenceText {
    /// Validate raw units, e.g. `ComposerReferenceText::from_units(&[0xd800])?`.
    pub fn from_units(units: &[u16]) -> Result<Self> {
        if units.len() > i32::MAX as usize || units.contains(&0) {
            return Err(invalid(format!(
                "reference length {}, NUL {}; expected NUL-free UTF-16 of at most {} units",
                units.len(),
                units.contains(&0),
                i32::MAX
            )));
        }
        Ok(Self(units.into()))
    }

    /// Encode Unicode without normalization, e.g. `ComposerReferenceText::from_text("guid")?`.
    pub fn from_text(value: &str) -> Result<Self> {
        Self::from_units(&value.encode_utf16().collect::<Vec<_>>())
    }

    /// Inspect exact units, e.g. `reference.code_units()`.
    pub fn code_units(&self) -> &[u16] {
        &self.0
    }

    /// Apply the native absolute-eight split, e.g. `reference.split()`.
    pub fn split(&self) -> ComposerReferenceSplit {
        let marker = self
            .0
            .windows(8)
            .position(|window| window == [82, 101, 102, 58, 69, 120, 116, 58]);
        let separator = marker.and_then(|_| self.0.iter().position(|unit| *unit == 35));
        let file = marker.map(|_| self.slice(8, separator.unwrap_or(self.0.len()).max(8)));
        let guid = self.remaining_guid(marker, separator);
        ComposerReferenceSplit {
            original: self.clone(),
            marker,
            separator,
            file,
            guid,
        }
    }

    fn remaining_guid(&self, marker: Option<usize>, separator: Option<usize>) -> Self {
        match (marker, separator) {
            (_, Some(at)) => self.slice(at + 1, self.0.len()),
            (Some(_), None) => self.slice(0, 0),
            (None, None) => self.clone(),
        }
    }

    pub(super) fn slice(&self, start: usize, end: usize) -> Self {
        Self(self.0[start.min(self.0.len())..end.min(self.0.len())].into())
    }

    pub(super) fn appended(&self, suffix: &str) -> Self {
        let mut units = self.0.to_vec();
        units.extend(suffix.encode_utf16());
        Self(units.into())
    }

    pub(super) fn last(&self, unit: u16) -> Option<usize> {
        self.0.iter().rposition(|candidate| *candidate == unit)
    }

    pub(super) fn ascii(&self) -> Result<String> {
        if let Some(unit) = self.0.iter().find(|unit| **unit > 127) {
            return Err(invalid(format!(
                "key unit {unit:#06x}; expected NUL-free ASCII under the C-locale profile"
            )));
        }
        Ok(self.0.iter().map(|unit| char::from(*unit as u8)).collect())
    }
}
