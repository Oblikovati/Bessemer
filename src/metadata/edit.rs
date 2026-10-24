use super::{XmlElementId, XmlLimits, XmlMetadata, invalid, lexical, missing};
use crate::Result;

impl XmlMetadata {
    /// Replaces leaf character content while preserving comments, PI, sibling values and physical framing.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata,id:&bessemer::metadata::XmlElementId,limits:bessemer::metadata::XmlLimits)->bessemer::Result<()> {
    /// let edited=metadata.with_leaf_text(id,"No",limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_leaf_text(
        &self,
        id: &XmlElementId,
        value: &str,
        limits: XmlLimits,
    ) -> Result<Self> {
        let element = self.element(id)?;
        if element.children().len() != 0 {
            return Err(invalid(format!(
                "XML element ordinal {} has child elements; expected a leaf for character replacement",
                id.ordinal
            )));
        }
        if element.direct_text() == value {
            return self.with_source(self.source().clone(), limits);
        }
        let escaped = lexical::escape(value, None)?;
        let record = &self.model.elements[id.ordinal];
        let mut source = self.source().clone();
        let text = source
            .document
            .as_deref()
            .ok_or_else(|| invalid("XML source absent; expected document for leaf edit"))?;
        source.document = Some(lexical::replace_leaf(text, record.span.clone(), &escaped)?);
        self.with_source(source, limits)
    }

    /// Changes an existing ordinary attribute by expanded name, retaining its quote style and spacing.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata,id:&bessemer::metadata::XmlElementId,limits:bessemer::metadata::XmlLimits)->bessemer::Result<()> {
    /// let edited=metadata.with_attribute_value(id,None,"name","Reviewed",limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_attribute_value(
        &self,
        id: &XmlElementId,
        namespace: Option<&str>,
        local: &str,
        value: &str,
        limits: XmlLimits,
    ) -> Result<Self> {
        self.element(id)?;
        let attribute=self.model.elements[id.ordinal].attributes.iter().find(|attribute|attribute.visible.name.namespace.as_deref()==namespace && attribute.visible.name.local.as_ref()==local)
            .ok_or_else(||missing(format!("XML attribute {namespace:?}:{local} on element {}; expected existing expanded attribute name",id.ordinal)))?;
        if attribute.visible.value == value {
            return self.with_source(self.source().clone(), limits);
        }
        let mut source = self.source().clone();
        let text = source
            .document
            .as_deref()
            .ok_or_else(|| invalid("XML source absent; expected document for attribute edit"))?;
        let (span, quote) = lexical::attribute_value_span(text, attribute.span.clone())?;
        source.document = Some(lexical::replace_span(
            text,
            span,
            &lexical::escape(value, Some(quote))?,
        )?);
        self.with_source(source, limits)
    }
}
