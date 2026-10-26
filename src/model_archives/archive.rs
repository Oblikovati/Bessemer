use super::{
    CompressedKernelContext, ConfigurationId, ModelArchiveLimits, ModelField, ModelObjectId,
    PartConfigurationContext, context::selected_error, part_archive,
};
use crate::archive_codec::ArchiveRevision;
use crate::header::{ArchiveVersion, HeaderReference};
use crate::{Document, Result};
use std::{fmt, sync::Arc};

/// Immutable complete ordinary modern part configuration, including both stored sections.
#[derive(Clone)]
pub struct PartConfigurationArchive {
    pub(super) context: PartConfigurationContext,
    pub(super) parsed: Arc<part_archive::ParsedPartArchive>,
    pub(super) revision: ArchiveRevision,
}

impl fmt::Debug for PartConfigurationArchive {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PartConfigurationArchive")
            .field("context", &self.context)
            .field("typed_fields", &self.fields().len())
            .finish()
    }
}

impl PartConfigurationArchive {
    /// Reads both complete sections and checks every shared-map reference through EOF.
    ///
    /// ```no_run
    /// # fn example(bytes: &[u8], context: bessemer::model_archives::PartConfigurationContext, limits: bessemer::model_archives::ModelArchiveLimits) -> bessemer::Result<()> {
    /// let archive = bessemer::model_archives::PartConfigurationArchive::from_bytes(bytes, context, limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(
        bytes: &[u8],
        context: PartConfigurationContext,
        limits: ModelArchiveLimits,
    ) -> Result<Self> {
        let parsed = part_archive::read_with_kernel(
            bytes,
            context.header.clone(),
            context.version,
            limits,
            context.kernel.clone(),
        )
        .map_err(|error| selected_error(error, context.configuration))?;
        Ok(Self {
            context,
            parsed: Arc::new(parsed),
            revision: ArchiveRevision::default(),
        })
    }

    /// Resolves the selected header once and reads its matching complete configuration stream.
    ///
    /// ```no_run
    /// # fn example(document: &bessemer::Document, limits: bessemer::model_archives::ModelArchiveLimits) -> bessemer::Result<()> {
    /// use bessemer::{header::ArchiveVersion, model_archives::{ConfigurationId, PartConfigurationArchive}};
    /// let archive = PartConfigurationArchive::from_document(document, ConfigurationId::new(0), ArchiveVersion::new(19000), None, limits)?;
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
        let context = PartConfigurationContext::from_document(
            document,
            configuration,
            version,
            kernel,
            limits,
        )?;
        let bytes = document
            .read_stream_limited(&configuration.stream_name(), limits.maximum_bytes)
            .map_err(|error| selected_error(error, configuration))?;
        Self::from_bytes(&bytes, context, limits)
    }

    /// Constructs from owned fields, deriving section lengths and checking an exact typed reparse.
    ///
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartConfigurationArchive, limits: bessemer::model_archives::ModelArchiveLimits) -> bessemer::Result<()> {
    /// let rebuilt = bessemer::model_archives::PartConfigurationArchive::from_fields(source.fields().to_vec(), source.context().clone(), limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_fields(
        fields: Vec<ModelField>,
        context: PartConfigurationContext,
        limits: ModelArchiveLimits,
    ) -> Result<Self> {
        let parsed = part_archive::construct(
            fields,
            context.header.clone(),
            context.version,
            limits,
            context.kernel.clone(),
        )
        .map_err(|error| selected_error(error, context.configuration))?;
        Ok(Self {
            context,
            parsed: Arc::new(parsed),
            revision: ArchiveRevision::default(),
        })
    }

    /// Borrows the immutable schema-checked fields in physical serialization order.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartConfigurationArchive) {
    /// for field in archive.fields() { let path = &field.path; }
    /// # }
    /// ```
    pub fn fields(&self) -> &[ModelField] {
        self.parsed.fields()
    }

    /// Borrows the declared configuration, header and model-version context.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartConfigurationArchive) {
    /// let configuration = archive.context().configuration();
    /// # }
    /// ```
    pub fn context(&self) -> &PartConfigurationContext {
        &self.context
    }

    /// Resolves an imported model-map slot to its header identity and owning immutable header.
    /// Imported null slots retain their explicit null reference.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartConfigurationArchive) -> bessemer::Result<()> {
    /// let origin = archive.header_origin(bessemer::model_archives::ModelObjectId::new(3)?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn header_origin(
        &self,
        identity: ModelObjectId,
    ) -> Option<(HeaderReference, &crate::header::DocumentHeader)> {
        let imports = &self.parsed.lightweight().imports;
        Some((imports.origin(identity)?, imports.header()))
    }

    /// Reconstructs all typed fields, retaining unchanged kernel compression encodings.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartConfigurationArchive) -> bessemer::Result<()> {
    /// let bytes = archive.to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.parsed
            .to_bytes()
            .map_err(|error| selected_error(error, self.context.configuration))
    }
}
