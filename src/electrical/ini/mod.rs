use super::*;
mod decode;
mod edit;
mod types;
pub use types::*;

/// Owned lossless ASCII INI records with no duplicate lookup or value normalization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalIniDocument {
    bytes: Box<[u8]>,
    limits: ElectricalIniLimits,
    records: Vec<ElectricalIniRecord>,
}

impl ElectricalIniDocument {
    /// Returns an ASCII view or opaque `None`; e.g. `ElectricalIniDocument::from_bytes(&raw, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: ElectricalIniLimits) -> Result<Option<Self>> {
        decode::document(bytes, limits)
    }
    /// Borrows all immutable original bytes; e.g. `let bytes = document.bytes()`.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Reconstructs all raw bytes; e.g. `assert_eq!(document.to_bytes(), original)`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Borrows physical lexical order, including duplicates; e.g. `let records = document.records()`.
    pub fn records(&self) -> &[ElectricalIniRecord] {
        &self.records
    }
    /// Reports complete lexical syntax coverage; e.g. `let checked = document.coverage()`.
    pub fn coverage(&self) -> ElectricalIniCoverage {
        ElectricalIniCoverage {
            mode: ElectricalIniCheckMode::ExactAsciiLexicalRecords,
            buffer_bytes: self.bytes.len(),
            records: self.records.len(),
            fields: self
                .records
                .iter()
                .filter(|record| record.kind == ElectricalIniKind::Field)
                .count(),
            achieved_byte_error: 0,
        }
    }
    /// Borrows field/native-load exclusions; e.g. `let exclusions = document.diagnostics()`.
    pub fn diagnostics(&self) -> &[ElectricalIniDiagnostic] {
        &[
            ElectricalIniDiagnostic::StoredValuesNotApplicationValidated,
            ElectricalIniDiagnostic::NativeLoadNotChecked,
        ]
    }
}

fn ini_error(entity: impl ToString, cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "electrical INI",
        &entity.to_string(),
        FailureClass::Invalid,
        cause,
    )
}
