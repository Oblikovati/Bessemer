use super::{
    CompressedKernelContext, ConfigurationId, ModelArchiveCoverage, ModelArchiveDiagnostic,
    ModelArchiveLimits, ModelEditCheckMode, ModelField, PartConfigurationArchive,
    configuration_manager, context::selected_error, resolved_features,
};
use crate::header::ArchiveVersion;
use crate::{Document, Result};
use std::sync::Arc;

/// Immutable complete ordinary part feature stream with its inherited model context.
#[derive(Debug, Clone)]
pub struct PartResolvedFeaturesArchive {
    state: Arc<ResolvedArchiveState>,
    pub(super) revision: crate::archive_codec::ArchiveRevision,
}

#[derive(Debug)]
struct ResolvedArchiveState {
    source: PartConfigurationArchive,
    manager: configuration_manager::ParsedConfigurationManager,
    parsed: resolved_features::ParsedResolvedFeatures,
    limits: ModelArchiveLimits,
    wire_bytes: usize,
}

impl PartResolvedFeaturesArchive {
    /// Reads a complete feature stream against an owned model snapshot and complete CMgr.
    ///
    /// ```no_run
    /// # fn example(features: &[u8], source: bessemer::model_archives::PartConfigurationArchive, manager: &[u8], limits: bessemer::model_archives::ModelArchiveLimits) -> bessemer::Result<()> {
    /// let archive = bessemer::model_archives::PartResolvedFeaturesArchive::from_bytes(features, source, manager, limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(
        bytes: &[u8],
        source: PartConfigurationArchive,
        manager_bytes: &[u8],
        limits: ModelArchiveLimits,
    ) -> Result<Self> {
        let configuration = source.context.configuration;
        let result = Self::read(bytes, source, manager_bytes, limits);
        result.map_err(|error| selected_error(error, configuration))
    }

    fn read(
        bytes: &[u8],
        source: PartConfigurationArchive,
        manager_bytes: &[u8],
        limits: ModelArchiveLimits,
    ) -> Result<Self> {
        let manager = configuration_manager::parse(manager_bytes, source.context.version, limits)?;
        if super::encode_fields(&manager.fields, limits.maximum_bytes)? != manager_bytes {
            return Err(super::invalid(
                "CMgr reconstruction differed from input; expected exact stored framing",
            ));
        }
        let parsed = resolved_features::parse(bytes, &source, &manager, limits)?;
        Ok(Self {
            revision: crate::archive_codec::ArchiveRevision::default(),
            state: Arc::new(ResolvedArchiveState {
                source,
                manager,
                parsed,
                limits,
                wire_bytes: bytes.len(),
            }),
        })
    }

    pub(super) fn configuration(&self) -> ConfigurationId {
        self.state.source.context.configuration
    }

    pub(super) fn with_fields(&self, fields: Vec<ModelField>) -> Result<Self> {
        let parsed = resolved_features::construct(
            fields,
            &self.state.source,
            &self.state.manager,
            self.state.limits,
        )
        .map_err(|error| selected_error(error, self.configuration()))?;
        if parsed.fields == self.fields() {
            return Ok(self.clone());
        }
        let wire_bytes =
            super::encode_fields(&parsed.fields, self.state.limits.maximum_bytes)?.len();
        Ok(Self {
            revision: crate::archive_codec::ArchiveRevision::default(),
            state: Arc::new(ResolvedArchiveState {
                source: self.state.source.clone(),
                manager: self.state.manager.clone(),
                parsed,
                limits: self.state.limits,
                wire_bytes,
            }),
        })
    }

    /// Resolves the selected model once, then reads its CMgr and matching feature stream.
    ///
    /// ```no_run
    /// # fn example(document: &bessemer::Document, limits: bessemer::model_archives::ModelArchiveLimits) -> bessemer::Result<()> {
    /// use bessemer::{header::ArchiveVersion, model_archives::{ConfigurationId, PartResolvedFeaturesArchive}};
    /// let archive = PartResolvedFeaturesArchive::from_document(document, ConfigurationId::new(0), ArchiveVersion::new(19000), None, limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_document(
        document: &Document,
        configuration: ConfigurationId,
        version: ArchiveVersion,
        kernel: Option<CompressedKernelContext>,
        limits: ModelArchiveLimits,
    ) -> Result<Self> {
        let source = PartConfigurationArchive::from_document(
            document,
            configuration,
            version,
            kernel,
            limits,
        )?;
        let manager = document
            .read_stream_limited("Contents/CMgr", limits.maximum_bytes)
            .map_err(|error| selected_error(error, configuration))?;
        let name = format!("{}-ResolvedFeatures", configuration.stream_name());
        let features = document
            .read_stream_limited(&name, limits.maximum_bytes)
            .map_err(|error| selected_error(error, configuration))?;
        Self::from_bytes(&features, source, &manager, limits)
    }

    /// Borrows the complete immutable fields in serialization order.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartResolvedFeaturesArchive) {
    /// let fields = archive.fields();
    /// # }
    /// ```
    pub fn fields(&self) -> &[ModelField] {
        &self.state.parsed.fields
    }

    /// Returns the serialized root count, including null and repeated roots.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartResolvedFeaturesArchive) {
    /// let roots = archive.root_count();
    /// # }
    /// ```
    pub fn root_count(&self) -> usize {
        self.state.parsed.roots
    }

    /// Returns inherited and final map slot counts, including reserved null slots.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartResolvedFeaturesArchive) {
    /// let (inherited, final_slots) = archive.object_slots();
    /// # }
    /// ```
    pub fn object_slots(&self) -> (usize, usize) {
        (
            self.state.source.parsed.object_map.slot_count(),
            self.state.parsed.final_map.slot_count(),
        )
    }

    /// Returns exact stored-schema coverage; imported slots refer to the source header.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartResolvedFeaturesArchive) {
    /// let bytes_checked = archive.coverage().wire_bytes;
    /// # }
    /// ```
    pub fn coverage(&self) -> ModelArchiveCoverage {
        let mut coverage = ModelArchiveCoverage {
            mode: ModelEditCheckMode::ExactStoredArchive,
            wire_bytes: self.state.wire_bytes,
            typed_fields: self.fields().len(),
            new_objects: 0,
            object_aliases: 0,
            imported_slots: self.state.source.coverage().imported_slots,
        };
        for field in self.fields() {
            super::validation::charge_object(&mut coverage, &field.value);
        }
        coverage
    }

    /// Borrows exclusions that prevent a wire-schema pass being mistaken for geometry validation.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartResolvedFeaturesArchive) {
    /// let exclusions = archive.diagnostics();
    /// # }
    /// ```
    pub fn diagnostics(&self) -> &[ModelArchiveDiagnostic] {
        super::validation::DIAGNOSTICS
    }

    /// Reconstructs and reparses the complete fields against their retained inherited context.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartResolvedFeaturesArchive) -> bessemer::Result<()> {
    /// let bytes = archive.to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let result = resolved_features::construct(
            self.fields().to_vec(),
            &self.state.source,
            &self.state.manager,
            self.state.limits,
        )
        .and_then(|parsed| super::encode_fields(&parsed.fields, self.state.limits.maximum_bytes));
        result.map_err(|error| selected_error(error, self.state.source.context.configuration))
    }
}
