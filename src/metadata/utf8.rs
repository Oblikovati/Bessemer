//! UTF-8 facade over the shared metadata grammar, namespace and lexical edit engine.
use super::{
    ExpandedName, XmlAttribute, XmlLimits, encoding, invalid, model, reader, types::XmlModel,
};
use crate::Result;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct Utf8XmlDocument {
    original: Arc<[u8]>,
    text: Arc<str>,
    model: Arc<XmlModel>,
    siblings: Arc<[usize]>,
    bom: bool,
    subject: &'static str,
}

impl Utf8XmlDocument {
    pub(crate) fn from_bytes(bytes: &[u8], limits: XmlLimits) -> Result<Self> {
        Self::from_bytes_for(bytes, limits, "Composer XML")
    }

    pub(crate) fn from_bytes_for(
        bytes: &[u8],
        limits: XmlLimits,
        subject: &'static str,
    ) -> Result<Self> {
        let (text, bom) = decode_utf8(bytes, limits, subject)?;
        let parsed = parse_model(text, bytes.len(), limits)?;
        let siblings = sibling_ordinals(&parsed);
        Ok(Self {
            original: bytes.into(),
            text: text.into(),
            model: Arc::new(parsed),
            siblings: siblings.into(),
            bom,
            subject,
        })
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.original
    }

    pub(crate) fn element_count(&self) -> usize {
        self.model.elements.len()
    }

    pub(crate) fn name(&self, ordinal: usize) -> &ExpandedName {
        &self.model.elements[ordinal].name
    }

    pub(crate) fn attributes(&self, ordinal: usize) -> impl Iterator<Item = &XmlAttribute> {
        self.model.elements[ordinal]
            .attributes
            .iter()
            .map(|attribute| &attribute.visible)
    }

    pub(crate) fn attribute(&self, ordinal: usize, local: &str) -> Option<&str> {
        self.attributes(ordinal)
            .find(|attribute| {
                attribute.name.namespace.is_none() && attribute.name.local.as_ref() == local
            })
            .map(|attribute| attribute.value.as_str())
    }

    pub(crate) fn element_path(&self, mut ordinal: usize) -> Vec<usize> {
        let mut path = Vec::new();
        while let Some(parent) = self.model.elements[ordinal].parent {
            path.push(self.siblings[ordinal]);
            ordinal = parent;
        }
        path.reverse();
        path
    }

    pub(crate) fn with_ident_geom(
        &self,
        ordinal: usize,
        value: &str,
        limits: XmlLimits,
    ) -> Result<Self> {
        self.with_attribute(ordinal, "IdentGeom", value, limits)
    }

    pub(crate) fn with_attribute(
        &self,
        ordinal: usize,
        local: &str,
        value: &str,
        limits: XmlLimits,
    ) -> Result<Self> {
        let text = self.replaced_text(ordinal, local, value)?;
        let source = utf8_source(text, self.bom);
        encoding::require_size(
            encoding::encoded_length(&source)?,
            limits.maximum_bytes,
            &format!("edited {} bytes", self.subject),
        )?;
        Self::from_bytes_for(&encoding::encode(&source), limits, self.subject)
    }

    fn replaced_text(&self, ordinal: usize, local: &str, value: &str) -> Result<String> {
        super::edit::replace_attribute(
            &self.text,
            &self.model.elements[ordinal],
            ordinal,
            None,
            local,
            value,
        )
    }

    pub(crate) fn parent(&self, ordinal: usize) -> Option<usize> {
        self.model.elements[ordinal].parent
    }

    pub(crate) fn children(&self, ordinal: usize) -> &[usize] {
        &self.model.elements[ordinal].children
    }
}

fn decode_utf8<'a>(bytes: &'a [u8], limits: XmlLimits, subject: &str) -> Result<(&'a str, bool)> {
    encoding::require_size(
        bytes.len(),
        limits.maximum_bytes,
        &format!("{subject} bytes"),
    )?;
    let bom = bytes.starts_with(&[0xef, 0xbb, 0xbf]);
    let payload = if bom { &bytes[3..] } else { bytes };
    let text = std::str::from_utf8(payload).map_err(|error| {
        invalid(format!(
            "{subject} encoding {error}; expected UTF-8 with optional BOM"
        ))
    })?;
    reject_declarations(text, subject)?;
    Ok((text, bom))
}

fn parse_model(text: &str, physical: usize, limits: XmlLimits) -> Result<XmlModel> {
    let mut parsed = reader::parse_document(text, limits)?;
    parsed.metrics.source_bytes = text.len();
    parsed.metrics.physical_bytes = physical;
    model::require_limits(parsed.metrics, limits)?;
    Ok(parsed)
}

fn utf8_source(text: String, bom: bool) -> super::XmlSource {
    super::XmlSource {
        document: Some(text),
        format: super::XmlFormat {
            encoding: super::XmlEncoding::Utf8,
            bom,
            keyword_prefix: None,
        },
    }
}

fn reject_declarations(text: &str, subject: &str) -> Result<()> {
    if text.contains("<!DOCTYPE") || text.contains("<!ENTITY") {
        return Err(super::unsupported(format!(
            "{subject} contains a DTD/entity declaration marker; expected no declarations or external resources"
        )));
    }
    Ok(())
}

fn sibling_ordinals(model: &XmlModel) -> Vec<usize> {
    let mut ordinals = vec![0; model.elements.len()];
    for element in &model.elements {
        for (index, child) in element.children.iter().enumerate() {
            ordinals[*child] = index;
        }
    }
    ordinals
}
