use super::{AssemblyConfigurationContext, ModelArchiveLimits, ModelField, ModelObjectId};
use super::{ModelArchiveCoverage, ModelArchiveDiagnostic, ModelFieldHandle, assembly_decoder};
use crate::Result;
use crate::archive_codec::ArchiveRevision;
use crate::header::HeaderReference;
use std::{fmt, sync::Arc};

/// Immutable complete ordinary assembly configuration with an explicit root kind.
#[derive(Clone)]
pub struct AssemblyConfigurationArchive {
    pub(super) context: AssemblyConfigurationContext,
    pub(super) parsed: Arc<assembly_decoder::ParsedAssemblyArchive>,
    pub(super) limits: ModelArchiveLimits,
    pub(super) revision: ArchiveRevision,
}

impl fmt::Debug for AssemblyConfigurationArchive {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AssemblyConfigurationArchive")
            .field("context", &self.context)
            .field("typed_fields", &self.fields().len())
            .field("map_slots", &self.parsed.object_map.slot_count())
            .finish()
    }
}

impl AssemblyConfigurationArchive {
    /// Check both sections and physical EOF, e.g. `AssemblyConfigurationArchive::from_bytes(bytes, context, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        context: AssemblyConfigurationContext,
        limits: ModelArchiveLimits,
    ) -> Result<Self> {
        let parsed = assembly_decoder::read(bytes, &context, limits).map_err(|error| {
            super::assembly_context::selected_error(error, context.configuration)
        })?;
        Ok(Self {
            context,
            parsed: Arc::new(parsed),
            limits,
            revision: ArchiveRevision::default(),
        })
    }

    /// Load the matching header and stream, e.g. `AssemblyConfigurationArchive::from_document(doc, id, kind, version, None, limits)?`.
    pub fn from_document(
        document: &crate::Document,
        configuration: super::ConfigurationId,
        kind: super::AssemblyRootKind,
        version: crate::header::ArchiveVersion,
        kernel: Option<super::CompressedKernelContext>,
        limits: ModelArchiveLimits,
    ) -> Result<Self> {
        let context = AssemblyConfigurationContext::from_document(
            document,
            configuration,
            kind,
            version,
            kernel,
            limits,
        )?;
        let bytes = document
            .read_stream_limited(&configuration.stream_name(), limits.maximum_bytes)
            .map_err(|error| super::assembly_context::selected_error(error, configuration))?;
        Self::from_bytes(&bytes, context, limits)
    }

    /// Force schema construction and derive section lengths, e.g. `AssemblyConfigurationArchive::from_fields(fields, context, limits)?`.
    pub fn from_fields(
        fields: Vec<ModelField>,
        context: AssemblyConfigurationContext,
        limits: ModelArchiveLimits,
    ) -> Result<Self> {
        let parsed = assembly_decoder::construct(fields, &context, limits).map_err(|error| {
            super::assembly_context::selected_error(error, context.configuration)
        })?;
        Ok(Self {
            context,
            parsed: Arc::new(parsed),
            limits,
            revision: ArchiveRevision::default(),
        })
    }

    /// Borrow stored fields in wire order, e.g. `archive.fields()`.
    pub fn fields(&self) -> &[ModelField] {
        &self.parsed.fields
    }

    /// Borrow the root/header selection, e.g. `archive.context()`.
    pub fn context(&self) -> &AssemblyConfigurationContext {
        &self.context
    }

    /// Reconstruct all stored bytes, e.g. `archive.to_bytes()?`.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        super::encode_fields(self.fields(), self.limits.maximum_bytes).map_err(|error| {
            super::assembly_context::selected_error(error, self.context.configuration)
        })
    }

    /// Resolve an imported header identity, e.g. `archive.header_origin(id)`.
    pub fn header_origin(
        &self,
        identity: ModelObjectId,
    ) -> Option<(HeaderReference, &crate::header::DocumentHeader)> {
        Some((
            self.parsed.imports.origin(identity)?,
            self.parsed.imports.header(),
        ))
    }

    /// Return exact stored-schema coverage, e.g. `archive.coverage().wire_bytes`.
    pub fn coverage(&self) -> ModelArchiveCoverage {
        super::validation::field_coverage(
            self.fields(),
            self.parsed.wire_bytes,
            self.parsed.imports.slots().len(),
        )
    }

    /// Borrow the geometry and external-document exclusions, e.g. `archive.diagnostics()`.
    pub fn diagnostics(&self) -> &[ModelArchiveDiagnostic] {
        super::validation::DIAGNOSTICS
    }

    /// Bind a field to this immutable revision, e.g. `archive.field_handle(0)?`.
    pub fn field_handle(&self, index: usize) -> Result<ModelFieldHandle> {
        super::edit_history::checked_handle(
            self.fields(),
            &self.revision,
            self.context.configuration,
            index,
        )
    }

    /// Reject handles from other revisions, e.g. `archive.resolve_field(&handle)?`.
    pub fn resolve_field(&self, handle: &ModelFieldHandle) -> Result<&ModelField> {
        super::edit_history::resolve_snapshot(
            self.fields(),
            &self.revision,
            self.context.configuration,
            handle,
        )
    }
}

pub use super::assembly_edit::AssemblyConfigurationEdit;
