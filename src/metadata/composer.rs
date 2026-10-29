//! Composer's UTF-8 facade reuses the metadata grammar, namespace and lexical edit engine.
use super::{
    ExpandedName, XmlAttribute, XmlLimits, encoding, invalid, model, reader, types::XmlModel,
};
use crate::Result;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct ComposerXmlDocument {
    original: Arc<[u8]>,
    text: Arc<str>,
    model: Arc<XmlModel>,
    siblings: Arc<[usize]>,
    bom: bool,
}

impl ComposerXmlDocument {
    pub(crate) fn from_bytes(bytes: &[u8], limits: XmlLimits) -> Result<Self> {
        let (text, bom) = decode_utf8(bytes, limits)?;
        let parsed = parse_model(text, bytes.len(), limits)?;
        let siblings = sibling_ordinals(&parsed);
        Ok(Self {
            original: bytes.into(),
            text: text.into(),
            model: Arc::new(parsed),
            siblings: siblings.into(),
            bom,
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
        let text = self.replaced_text(ordinal, value)?;
        let source = utf8_source(text, self.bom);
        encoding::require_size(
            encoding::encoded_length(&source)?,
            limits.maximum_bytes,
            "edited Composer XML bytes",
        )?;
        Self::from_bytes(&encoding::encode(&source), limits)
    }

    fn replaced_text(&self, ordinal: usize, value: &str) -> Result<String> {
        super::edit::replace_attribute(
            &self.text,
            &self.model.elements[ordinal],
            ordinal,
            None,
            "IdentGeom",
            value,
        )
    }
}

fn decode_utf8(bytes: &[u8], limits: XmlLimits) -> Result<(&str, bool)> {
    encoding::require_size(bytes.len(), limits.maximum_bytes, "Composer XML bytes")?;
    let bom = bytes.starts_with(&[0xef, 0xbb, 0xbf]);
    let payload = if bom { &bytes[3..] } else { bytes };
    let text = std::str::from_utf8(payload).map_err(|error| {
        invalid(format!(
            "Composer XML encoding {error}; expected UTF-8 with optional BOM"
        ))
    })?;
    reject_declarations(text)?;
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

fn reject_declarations(text: &str) -> Result<()> {
    if text.contains("<!DOCTYPE") || text.contains("<!ENTITY") {
        return Err(super::unsupported(
            "Composer XML contains a DTD/entity declaration marker; expected no declarations or external resources",
        ));
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
