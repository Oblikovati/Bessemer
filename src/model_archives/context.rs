use super::{CompressedKernelContext, ConfigurationId, ModelArchiveLimits};
use crate::header::{ArchiveVersion, DocumentHeader};
use crate::{BessemerError, Document, Result};
use std::sync::Arc;

/// Declared ordinary external-XML part context using the current 19000 runtime layout.
/// Model wire versions remain independent; legacy runtime overrides are outside this dialect.
/// Ordinary loading declares helper +834 clear; a nonzero override is another dialect.
#[derive(Debug, Clone)]
pub struct PartConfigurationContext {
    pub(super) configuration: ConfigurationId,
    pub(super) version: ArchiveVersion,
    pub(super) header: Arc<DocumentHeader>,
    pub(super) kernel: Option<CompressedKernelContext>,
}

impl PartConfigurationContext {
    /// Declares the supported dialect and checks that the header and model outer versions match.
    ///
    /// ```no_run
    /// # fn example(header: std::sync::Arc<bessemer::header::DocumentHeader>) -> bessemer::Result<()> {
    /// use bessemer::{model_archives::{ConfigurationId, PartConfigurationContext}, header::ArchiveVersion};
    /// let context = PartConfigurationContext::ordinary_external_xml(ConfigurationId::new(0), ArchiveVersion::new(19000), header, None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn ordinary_external_xml(
        configuration: ConfigurationId,
        version: ArchiveVersion,
        header: Arc<DocumentHeader>,
        kernel: Option<CompressedKernelContext>,
    ) -> Result<Self> {
        super::header_imports::require_import_context(&header, version)
            .map_err(|error| selected_error(error, configuration))?;
        Ok(Self {
            configuration,
            version,
            header,
            kernel,
        })
    }

    /// Reads the selected configuration's actual header with explicit byte and object budgets.
    /// No model version is inferred from unrelated document directories.
    ///
    /// ```no_run
    /// # fn example(document: &bessemer::Document, limits: bessemer::model_archives::ModelArchiveLimits) -> bessemer::Result<()> {
    /// use bessemer::{model_archives::{ConfigurationId, PartConfigurationContext}, header::ArchiveVersion};
    /// let context = PartConfigurationContext::from_document(document, ConfigurationId::new(0), ArchiveVersion::new(19000), None, limits)?;
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
        let bytes = document
            .read_stream_limited(&configuration.header_name(), limits.maximum_bytes)
            .map_err(|error| selected_error(error, configuration))?;
        let header = DocumentHeader::from_bytes(&bytes, version, limits.maximum_objects)
            .map_err(|error| selected_error(error, configuration))?;
        Self::ordinary_external_xml(configuration, version, Arc::new(header), kernel)
    }

    /// Returns the normalized selected configuration.
    ///
    /// ```no_run
    /// # fn example(context: &bessemer::model_archives::PartConfigurationContext) {
    /// let id = context.configuration();
    /// # }
    /// ```
    pub fn configuration(&self) -> ConfigurationId {
        self.configuration
    }

    /// Returns the explicitly supplied model wire version.
    ///
    /// ```no_run
    /// # fn example(context: &bessemer::model_archives::PartConfigurationContext) {
    /// let version = context.version();
    /// # }
    /// ```
    pub fn version(&self) -> ArchiveVersion {
        self.version
    }

    /// Borrows the immutable header that supplies imported object identities.
    ///
    /// ```no_run
    /// # fn example(context: &bessemer::model_archives::PartConfigurationContext) {
    /// let header = context.header();
    /// # }
    /// ```
    pub fn header(&self) -> &DocumentHeader {
        &self.header
    }
}

pub(super) fn selected_error(
    mut error: BessemerError,
    configuration: ConfigurationId,
) -> BessemerError {
    let location = std::mem::replace(&mut error.entity, configuration.stream_name());
    error.operation = "part configuration";
    error.cause = format!("{location}: {}", error.cause);
    error
}
