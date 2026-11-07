use super::{CompressedKernelContext, ConfigurationId, ModelArchiveLimits};
use crate::header::{ArchiveVersion, DocumentHeader};
use crate::{BessemerError, Document, Result};
use std::sync::Arc;

/// Exact ordinary root class; routing trailers are outside the resolved section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssemblyRootKind {
    /// `moAssembly_c` configuration.
    Assembly,
    /// `moRouteAssembly_c` configuration and its routing trailer.
    RoutingAssembly,
}

impl AssemblyRootKind {
    pub(super) fn classes(self) -> &'static [&'static str] {
        match self {
            Self::Assembly => &["moAssembly_c"],
            Self::RoutingAssembly => &["moRouteAssembly_c"],
        }
    }
}

/// Explicit ordinary assembly configuration, real header and optional kernel context.
#[derive(Debug, Clone)]
pub struct AssemblyConfigurationContext {
    pub(super) configuration: ConfigurationId,
    pub(super) kind: AssemblyRootKind,
    pub(super) version: ArchiveVersion,
    pub(super) header: Arc<DocumentHeader>,
    pub(super) kernel: Option<CompressedKernelContext>,
}

impl AssemblyConfigurationContext {
    /// Select a root kind and matching header/version, e.g. `ordinary_external_xml(id, kind, version, header, None)?`.
    pub fn ordinary_external_xml(
        configuration: ConfigurationId,
        kind: AssemblyRootKind,
        version: ArchiveVersion,
        header: Arc<DocumentHeader>,
        kernel: Option<CompressedKernelContext>,
    ) -> Result<Self> {
        require_version(version).map_err(|error| selected_error(error, configuration))?;
        super::header_imports::require_import_context(&header, version)
            .map_err(|error| selected_error(error, configuration))?;
        Ok(Self {
            configuration,
            kind,
            version,
            header,
            kernel,
        })
    }

    /// Read the selected real model header, e.g. `from_document(doc, id, kind, version, None, limits)?`.
    pub fn from_document(
        document: &Document,
        configuration: ConfigurationId,
        kind: AssemblyRootKind,
        version: ArchiveVersion,
        kernel: Option<CompressedKernelContext>,
        limits: ModelArchiveLimits,
    ) -> Result<Self> {
        require_version(version).map_err(|error| selected_error(error, configuration))?;
        let bytes = document
            .read_stream_limited(&configuration.header_name(), limits.maximum_bytes)
            .map_err(|error| selected_error(error, configuration))?;
        let header = DocumentHeader::from_bytes(&bytes, version, limits.maximum_objects)
            .map_err(|error| selected_error(error, configuration))?;
        Self::ordinary_external_xml(configuration, kind, version, Arc::new(header), kernel)
    }

    /// Return the normalized configuration, e.g. `context.configuration()`.
    pub fn configuration(&self) -> ConfigurationId {
        self.configuration
    }

    /// Return the declared root class selection, e.g. `context.root_kind()`.
    pub fn root_kind(&self) -> AssemblyRootKind {
        self.kind
    }

    /// Return the declared outer wire version, e.g. `context.version()`.
    pub fn version(&self) -> ArchiveVersion {
        self.version
    }

    /// Borrow the immutable imported header, e.g. `context.header()`.
    pub fn header(&self) -> &DocumentHeader {
        &self.header
    }
}

fn require_version(version: ArchiveVersion) -> Result<()> {
    if version.value() > 0x27cb {
        return Ok(());
    }
    Err(BessemerError::new(
        "assembly configuration",
        "archive version",
        crate::FailureClass::Unsupported,
        format!(
            "assembly version {}; expected > 0x27CB for the ordinary modern layout",
            version.value()
        ),
    ))
}

pub(super) fn selected_error(
    error: BessemerError,
    configuration: ConfigurationId,
) -> BessemerError {
    let mut error = super::context::selected_error(error, configuration);
    error.operation = "assembly configuration";
    error
}
