//! Complete version-7.9 gzip geometry streams, with named typed fields and isolated edits.
mod decode;
mod drawable;
mod encode;
mod gzip;
mod gzip_output;
mod metadata;
mod reader;
mod text;
mod types;
mod value;

use crate::archive_codec::ArchiveRevision;
use crate::{BessemerError, FailureClass, Result};
use std::sync::Arc;
pub use text::{ComposerSceneText, ComposerTextTerminator};
pub use types::*;
pub use value::ComposerSceneValue;

/// Immutable complete geometry stream, separate from native loaded-mesh/project semantics.
#[derive(Debug, Clone)]
pub struct ComposerGeometryArchive {
    fields: Arc<[ComposerSceneField]>,
    rules: Arc<[SizeRule]>,
    context: ComposerSceneContext,
    limits: ComposerSceneLimits,
    revision: ComposerSceneRevision,
    coverage: ComposerSceneCoverage,
    gzip_header: [u8; 10],
    original_gzip: Option<Arc<[u8]>>,
}

impl ComposerGeometryArchive {
    /// Read a complete gzip member, e.g. `ComposerGeometryArchive::from_gzip(bytes, context, limits)?`.
    pub fn from_gzip(
        bytes: &[u8],
        context: ComposerSceneContext,
        limits: ComposerSceneLimits,
    ) -> Result<Self> {
        let (header, decoded) = gzip::decode_gzip(bytes, limits).map_err(scene_error)?;
        let mut archive = Self::from_decoded(&decoded, context, limits)?;
        archive.gzip_header = header;
        archive.original_gzip = Some(bytes.into());
        archive.coverage.input_gzip_integrity_checked = true;
        Ok(archive)
    }

    /// Read the complete decoded stream, e.g. `ComposerGeometryArchive::from_decoded(bytes, context, limits)?`.
    pub fn from_decoded(
        bytes: &[u8],
        context: ComposerSceneContext,
        limits: ComposerSceneLimits,
    ) -> Result<Self> {
        let ledger = decode::decode_scene(bytes, context, limits).map_err(scene_error)?;
        if encode::encode_fields(&ledger.fields, limits)? != bytes {
            return Err(invalid(format!(
                "decoded reconstruction differs for {} bytes; expected exact complete stream",
                bytes.len()
            )));
        }
        Ok(publish_ledger(ledger, bytes.len(), context, limits))
    }

    /// Construct complete canonical fields, e.g. `ComposerGeometryArchive::from_fields(fields, context, limits)?`.
    pub fn from_fields(
        fields: &[ComposerSceneField],
        context: ComposerSceneContext,
        limits: ComposerSceneLimits,
    ) -> Result<Self> {
        let decoded = encode::encode_fields(fields, limits)?;
        let archive = Self::from_decoded(&decoded, context, limits)?;
        validate_fields(fields, &archive.fields)?;
        Ok(archive)
    }

    /// Ordered immutable stored fields, e.g. `archive.fields()`.
    pub fn fields(&self) -> &[ComposerSceneField] {
        &self.fields
    }

    /// Complete revision, e.g. `archive.revision().clone()`.
    pub fn revision(&self) -> &ComposerSceneRevision {
        &self.revision
    }

    /// Validated wire scope, e.g. `archive.coverage().decoded_bytes`.
    pub fn coverage(&self) -> ComposerSceneCoverage {
        self.coverage
    }

    /// Canonical typed reconstruction, e.g. `archive.to_decoded()?`.
    pub fn to_decoded(&self) -> Result<Vec<u8>> {
        encode::encode_fields(&self.fields, self.limits)
    }

    /// Exact original gzip for no-op reads, otherwise a complete new stream, e.g. `archive.to_gzip()?`.
    pub fn to_gzip(&self) -> Result<Vec<u8>> {
        if let Some(original) = &self.original_gzip {
            return Ok(original.to_vec());
        }
        gzip::encode_gzip(&self.to_decoded()?, self.gzip_header, self.limits)
    }

    /// Bind a canonical path to this revision, e.g. `archive.field_id("information.source")?`.
    pub fn field_id(&self, path: &str) -> Result<ComposerSceneFieldId> {
        let index = self
            .fields
            .iter()
            .position(|field| field.path.as_ref() == path)
            .ok_or_else(|| {
                BessemerError::new(
                    "Composer geometry stream",
                    path,
                    FailureClass::Missing,
                    format!("field path {path:?}; expected a stored field in this complete stream"),
                )
            })?;
        Ok(ComposerSceneFieldId {
            revision: self.revision.clone(),
            index,
        })
    }

    /// Read through an existing typed identity, e.g. `&archive.field(&id)?.value`.
    pub fn field(&self, id: &ComposerSceneFieldId) -> Result<&ComposerSceneField> {
        Ok(&self.fields[self.field_index(id)?])
    }

    /// Non-destructive single-field edit with derived bounded sizes, e.g. `archive.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &ComposerSceneFieldId,
        value: ComposerSceneValue,
    ) -> Result<ComposerSceneEdit> {
        let index = self.field_index(id)?;
        if self.fields[index].value == value {
            return Ok(edit_outcome(self, self.clone()));
        }
        let fields = self.edited_fields(index, value)?;
        let mut archive = Self::from_fields(&fields, self.context, self.limits)?;
        archive.gzip_header = self.gzip_header;
        // A changed result is published only after its complete encoded transport also validates.
        let encoded = archive.to_gzip()?;
        archive = Self::from_gzip(&encoded, self.context, self.limits)?;
        Ok(edit_outcome(self, archive))
    }

    fn edited_fields(
        &self,
        index: usize,
        value: ComposerSceneValue,
    ) -> Result<Vec<ComposerSceneField>> {
        if self.rules.iter().any(|rule| rule.size_field == index) {
            return Err(invalid(format!(
                "replacement {value:?} for bounded-size field {}; expected a size derived from its contents",
                self.fields[index].path
            )));
        }
        let mut fields = self.fields.to_vec();
        fields[index].value = value;
        encode::derive_sizes(&mut fields, &self.rules)?;
        Ok(fields)
    }

    fn field_index(&self, id: &ComposerSceneFieldId) -> Result<usize> {
        if id.revision == self.revision && id.index < self.fields.len() {
            return Ok(id.index);
        }
        Err(invalid(format!(
            "field index {} from another/missing revision; expected this stream's {} fields",
            id.index,
            self.fields.len()
        )))
    }
}

fn validate_fields(supplied: &[ComposerSceneField], parsed: &[ComposerSceneField]) -> Result<()> {
    if let Some((actual, expected)) = supplied.iter().zip(parsed).find(|(a, b)| a != b) {
        return Err(invalid(format!(
            "field {} ({}); expected canonical field {} ({})",
            actual.path,
            value::value_shape(&actual.value),
            expected.path,
            value::value_shape(&expected.value)
        )));
    }
    if supplied.len() != parsed.len() {
        return Err(invalid(format!(
            "supplied field count {}; expected {} complete canonical fields",
            supplied.len(),
            parsed.len()
        )));
    }
    Ok(())
}

fn publish_ledger(
    ledger: SceneLedger,
    bytes: usize,
    context: ComposerSceneContext,
    limits: ComposerSceneLimits,
) -> ComposerGeometryArchive {
    let coverage = coverage_for(&ledger, bytes);
    ComposerGeometryArchive {
        fields: ledger.fields.into(),
        rules: ledger.rules.into(),
        context,
        limits,
        revision: ComposerSceneRevision(ArchiveRevision::default()),
        coverage,
        gzip_header: gzip::DEFAULT_HEADER,
        original_gzip: None,
    }
}

fn coverage_for(ledger: &SceneLedger, decoded_bytes: usize) -> ComposerSceneCoverage {
    ComposerSceneCoverage {
        mode: ComposerSceneCheckMode::ExactSceneWire,
        decoded_bytes,
        fields: ledger.fields.len(),
        geodes: ledger.geodes,
        drawables: ledger.drawables,
        sized_extension_bytes: ledger.extension_bytes,
        input_gzip_integrity_checked: false,
    }
}

fn edit_outcome(
    parent: &ComposerGeometryArchive,
    archive: ComposerGeometryArchive,
) -> ComposerSceneEdit {
    let unchanged = parent.revision == archive.revision;
    let history = ComposerSceneHistory {
        parent: parent.revision.clone(),
        result: archive.revision.clone(),
    };
    ComposerSceneEdit {
        coverage: archive.coverage,
        archive,
        history,
        completion: edit_completion(unchanged),
        fidelity: ComposerSceneEditFidelity::ExactTypedFields,
        diagnostics: SCENE_DIAGNOSTICS,
    }
}

fn edit_completion(unchanged: bool) -> crate::display_lists::DisplayEditCompletion {
    if unchanged {
        return crate::display_lists::DisplayEditCompletion::Unchanged;
    }
    crate::display_lists::DisplayEditCompletion::Changed
}

const SCENE_DIAGNOSTICS: [ComposerSceneDiagnostic; 3] = [
    ComposerSceneDiagnostic::ProjectSemanticsNotChecked,
    ComposerSceneDiagnostic::GeometryNotChecked,
    ComposerSceneDiagnostic::NativeApplicationNotChecked,
];

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "Composer geometry stream",
        "complete wire",
        FailureClass::Invalid,
        cause,
    )
}

fn unsupported(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "Composer geometry stream",
        "version/layout",
        FailureClass::Unsupported,
        cause,
    )
}

fn budget(label: &str, actual: usize, maximum: usize) -> Result<()> {
    if actual <= maximum {
        return Ok(());
    }
    Err(invalid(format!(
        "{label} {actual}; expected at most {maximum}"
    )))
}

fn scene_error(mut error: BessemerError) -> BessemerError {
    error.operation = "Composer geometry stream";
    error
}
