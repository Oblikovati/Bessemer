mod bindings;
mod types;
use super::{ElectricalFileLink, ElectricalRecord};
use crate::{BessemerError, FailureClass, Result};
use std::sync::Arc;
pub use types::*;

impl ElectricalFileLink {
    /// Maps all 38 documented direct reads without provider conversion; e.g. `let mapped = link.file_read_map()?`.
    pub fn file_read_map(&self) -> Result<ElectricalFileReadMap> {
        let reads = bindings::FILE_READ_BINDINGS
            .iter()
            .map(|(column, reader, destination)| {
                let value = self
                    .record
                    .value(column)
                    .cloned()
                    .ok_or_else(|| missing_file_column(column))?;
                Ok(ElectricalFileRead {
                    column,
                    reader: *reader,
                    destination: *destination,
                    value,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(ElectricalFileReadMap {
            record: self.record.clone(),
            reads: reads.into(),
        })
    }
}

impl ElectricalFileReadMap {
    /// Borrows every direct read in native call order; e.g. `let reads = mapped.reads()`.
    pub fn reads(&self) -> &[ElectricalFileRead] {
        &self.reads
    }
    /// Borrows all original names and values, including unmatched columns; e.g. `let record = mapped.record()`.
    pub fn record(&self) -> &ElectricalRecord {
        &self.record
    }
    /// Enumerates columns outside the 38 direct reads without classifying them as ignored; e.g. `let extra = mapped.unmapped_columns().collect::<Vec<_>>()`.
    pub fn unmapped_columns(
        &self,
    ) -> impl Iterator<Item = (&str, &crate::sqlite_image::SqliteValue)> + '_ {
        self.record
            .columns()
            .iter()
            .zip(self.record.values())
            .filter(|(name, _)| {
                !bindings::FILE_READ_BINDINGS
                    .iter()
                    .any(|(column, _, _)| name.as_str() == *column)
            })
            .map(|(name, value)| (name.as_str(), value))
    }
    /// Reports the exact declared transport-map scope; e.g. `let checked = mapped.coverage()`.
    pub fn coverage(&self) -> ElectricalFileReadCoverage {
        ElectricalFileReadCoverage {
            mode: ElectricalFileReadCheckMode::DeclaredNativeTransport,
            mapped_reads: self.reads.len(),
            preserved_columns: self.record.columns().len(),
        }
    }
    /// Borrows explicit runtime/provider exclusions; e.g. `let excluded = mapped.diagnostics()`.
    pub fn diagnostics(&self) -> &[ElectricalFileReadDiagnostic] {
        &[
            ElectricalFileReadDiagnostic::ProviderConversionsNotApplied,
            ElectricalFileReadDiagnostic::RuntimeOffsetsAreNotFileOffsets,
            ElectricalFileReadDiagnostic::TemporaryReuseHasNoIndependentDefaults,
            ElectricalFileReadDiagnostic::UnmappedColumnsUnclassified,
            ElectricalFileReadDiagnostic::VirtualIdentityDestinationUnresolved,
            ElectricalFileReadDiagnostic::NativeLoaderNotExecuted,
        ]
    }
}

fn missing_file_column(column: &str) -> BessemerError {
    BessemerError::new(
        "Electrical file read map",
        column,
        FailureClass::Missing,
        format!(
            "missing column {column:?}; expected all 38 documented direct-read columns, retaining NULL values"
        ),
    )
}
