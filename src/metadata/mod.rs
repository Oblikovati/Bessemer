//! Lossless, namespace-aware SolidWorks XML metadata with immutable revision-scoped edits.
mod attributes;
mod composer;
mod declaration;
mod edit;
mod encoding;
mod grammar;
mod inspection;
mod lexical;
mod model;
mod namespaces;
mod properties;
mod reader;
mod stream;
mod types;
mod views;

use crate::{BessemerError, FailureClass, Result, archive_codec::PreservedContents};
pub(crate) use composer::ComposerXmlDocument;
pub(crate) use inspection::validate as validate_inspection_xml;
pub use properties::{MetadataProperty, PropertySection, PropertyValue, PropertyValueKind};
use std::sync::Arc;
pub use stream::{MetadataFamily, MetadataStream};
use types::XmlModel;
pub use types::{
    ExpandedName, XmlAttribute, XmlElementId, XmlEncoding, XmlFormat, XmlLimits, XmlSource,
};
pub use views::XmlElementRef;

/// Immutable source plus a complete flat element/attribute model; reads take no lock.
#[derive(Debug, Clone)]
pub struct XmlMetadata {
    stream: MetadataStream,
    source: PreservedContents<XmlSource>,
    model: Arc<XmlModel>,
    revision: Arc<()>,
}

impl XmlMetadata {
    /// Reads a complete known metadata stream, including empty and keyword-framed forms.
    ///
    /// ```no_run
    /// # fn example(stream:bessemer::metadata::MetadataStream,bytes:&[u8],limits:bessemer::metadata::XmlLimits)->bessemer::Result<()> {
    /// let metadata=bessemer::metadata::XmlMetadata::from_bytes(stream,bytes,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(stream: MetadataStream, bytes: &[u8], limits: XmlLimits) -> Result<Self> {
        let source = encoding::decode(&stream, bytes, limits)?;
        let model = Arc::new(model::parse(&stream, &source, limits)?);
        Ok(Self {
            stream,
            source: PreservedContents::decoded(bytes, source),
            model,
            revision: Arc::new(()),
        })
    }

    /// Constructs a complete document from its exact lexical source and physical framing.
    ///
    /// ```no_run
    /// # fn example(stream:bessemer::metadata::MetadataStream,source:bessemer::metadata::XmlSource,limits:bessemer::metadata::XmlLimits)->bessemer::Result<()> {
    /// let metadata=bessemer::metadata::XmlMetadata::new(stream,source,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(stream: MetadataStream, source: XmlSource, limits: XmlLimits) -> Result<Self> {
        let model = Arc::new(model::parse(&stream, &source, limits)?);
        Ok(Self {
            stream,
            source: PreservedContents::constructed(source),
            model,
            revision: Arc::new(()),
        })
    }

    /// Borrows the validated stream identity and normalized family.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata){
    /// let family=metadata.stream().family();
    /// # }
    /// ```
    pub fn stream(&self) -> &MetadataStream {
        &self.stream
    }

    /// Borrows exact source, encoding, BOM and opaque keyword prefix.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata){
    /// let source=metadata.source();
    /// # }
    /// ```
    pub fn source(&self) -> &XmlSource {
        self.source.current()
    }

    /// Returns the document element, or None for a completed empty stream.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata){
    /// let root=metadata.root();
    /// # }
    /// ```
    pub fn root(&self) -> Option<XmlElementRef<'_>> {
        (!self.model.elements.is_empty()).then_some(XmlElementRef {
            owner: self,
            ordinal: 0,
        })
    }

    /// Iterates every element in stable document preorder.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata){
    /// for element in metadata.elements(){let name=element.name();}
    /// # }
    /// ```
    pub fn elements(&self) -> impl ExactSizeIterator<Item = XmlElementRef<'_>> {
        (0..self.model.elements.len()).map(|ordinal| XmlElementRef {
            owner: self,
            ordinal,
        })
    }

    /// Resolves a preorder ordinal into a scoped element view.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata)->bessemer::Result<()> {
    /// let root=metadata.element_at(0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn element_at(&self, ordinal: usize) -> Result<XmlElementRef<'_>> {
        if ordinal < self.model.elements.len() {
            return Ok(XmlElementRef {
                owner: self,
                ordinal,
            });
        }
        Err(missing(format!(
            "XML element ordinal {ordinal}; expected below {}",
            self.model.elements.len()
        )))
    }

    /// Resolves an identity only in the unchanged snapshot that issued it.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata,id:&bessemer::metadata::XmlElementId)->bessemer::Result<()> {
    /// let element=metadata.element(id)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn element(&self, id: &XmlElementId) -> Result<XmlElementRef<'_>> {
        if !Arc::ptr_eq(&self.revision, &id.revision) {
            return Err(invalid(format!(
                "XML element ordinal {} from another revision; expected identity issued by this snapshot",
                id.ordinal
            )));
        }
        self.element_at(id.ordinal)
    }

    /// Validates a complete source edit before publishing a new immutable revision.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata,limits:bessemer::metadata::XmlLimits)->bessemer::Result<()> {
    /// let equivalent=metadata.with_source(metadata.source().clone(),limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_source(&self, source: XmlSource, limits: XmlLimits) -> Result<Self> {
        if &source == self.source() {
            model::require_limits(self.model.metrics, limits)?;
            return Ok(self.clone());
        }
        let model = Arc::new(model::parse(&self.stream, &source, limits)?);
        Ok(Self {
            stream: self.stream.clone(),
            source: self.source.edited(source),
            model,
            revision: Arc::new(()),
        })
    }

    /// Writes exact stored framing and source, preserving equivalent edits and authentic reverts.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata)->bessemer::Result<()> {
    /// let bytes=metadata.to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.source.original_bytes() {
            return Ok(bytes.to_vec());
        }
        Ok(encoding::encode(self.source()))
    }
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "XML metadata",
        "metadata document",
        FailureClass::Invalid,
        cause,
    )
}
fn unsupported(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "XML metadata",
        "metadata document",
        FailureClass::Unsupported,
        cause,
    )
}
fn missing(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "XML metadata",
        "metadata document",
        FailureClass::Missing,
        cause,
    )
}
