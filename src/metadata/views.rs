use super::{ExpandedName, XmlAttribute, XmlElementId, XmlMetadata};

/// Read-only element view; child ordinals and identities belong to its immutable snapshot.
#[derive(Debug, Clone, Copy)]
pub struct XmlElementRef<'a> {
    pub(super) owner: &'a XmlMetadata,
    pub(super) ordinal: usize,
}

impl<'a> XmlElementRef<'a> {
    /// Returns a revision-scoped identity for checked immutable edits.
    ///
    /// ```no_run
    /// # fn example(element:bessemer::metadata::XmlElementRef<'_>){
    /// let identity=element.id();
    /// # }
    /// ```
    pub fn id(self) -> XmlElementId {
        XmlElementId {
            ordinal: self.ordinal,
            revision: self.owner.revision.clone(),
        }
    }

    /// Returns its stable preorder ordinal in this snapshot.
    ///
    /// ```no_run
    /// # fn example(element:bessemer::metadata::XmlElementRef<'_>){
    /// let index=element.ordinal();
    /// # }
    /// ```
    pub fn ordinal(self) -> usize {
        self.ordinal
    }

    /// Borrows the namespace-expanded name, independently of source prefix spelling.
    ///
    /// ```no_run
    /// # fn example(element:bessemer::metadata::XmlElementRef<'_>){
    /// let name=element.name();
    /// # }
    /// ```
    pub fn name(self) -> &'a ExpandedName {
        &self.owner.model.elements[self.ordinal].name
    }

    /// Iterates ordinary attributes in source order; namespace declarations remain in raw source.
    ///
    /// ```no_run
    /// # fn example(element:bessemer::metadata::XmlElementRef<'_>){
    /// for attribute in element.attributes(){let value=&attribute.value;}
    /// # }
    /// ```
    pub fn attributes(self) -> impl ExactSizeIterator<Item = &'a XmlAttribute> {
        self.owner.model.elements[self.ordinal]
            .attributes
            .iter()
            .map(|attribute| &attribute.visible)
    }

    /// Looks up an attribute by expanded identity; unqualified attributes use None.
    ///
    /// ```no_run
    /// # fn example(element:bessemer::metadata::XmlElementRef<'_>){
    /// let pid=element.attribute(None,"pid");
    /// # }
    /// ```
    pub fn attribute(self, namespace: Option<&str>, local: &str) -> Option<&'a str> {
        self.attributes()
            .find(|attribute| {
                attribute.name.namespace.as_deref() == namespace
                    && attribute.name.local.as_ref() == local
            })
            .map(|attribute| attribute.value.as_str())
    }

    /// Borrows concatenated direct character content, without interpreting units or value types.
    ///
    /// ```no_run
    /// # fn example(element:bessemer::metadata::XmlElementRef<'_>){
    /// let stored_text=element.direct_text();
    /// # }
    /// ```
    pub fn direct_text(self) -> &'a str {
        &self.owner.model.elements[self.ordinal].direct_text
    }

    /// Iterates direct child elements without dropping unknown names or extra value children.
    ///
    /// ```no_run
    /// # fn example(element:bessemer::metadata::XmlElementRef<'_>){
    /// for child in element.children(){let name=child.name();}
    /// # }
    /// ```
    pub fn children(self) -> impl ExactSizeIterator<Item = XmlElementRef<'a>> {
        self.owner.model.elements[self.ordinal]
            .children
            .iter()
            .map(move |ordinal| XmlElementRef {
                owner: self.owner,
                ordinal: *ordinal,
            })
    }

    /// Returns the parent element, or None for the document element.
    ///
    /// ```no_run
    /// # fn example(element:bessemer::metadata::XmlElementRef<'_>){
    /// let parent=element.parent();
    /// # }
    /// ```
    pub fn parent(self) -> Option<XmlElementRef<'a>> {
        self.owner.model.elements[self.ordinal]
            .parent
            .map(|ordinal| XmlElementRef {
                owner: self.owner,
                ordinal,
            })
    }

    /// Borrows this element's complete original lexical markup from the decoded source.
    ///
    /// ```no_run
    /// # fn example(element:bessemer::metadata::XmlElementRef<'_>){
    /// let markup=element.raw_source();
    /// # }
    /// ```
    pub fn raw_source(self) -> &'a str {
        let source = self.owner.source().document.as_deref().unwrap_or_default();
        &source[self.owner.model.elements[self.ordinal].span.clone()]
    }

    pub(super) fn named(self, namespace: Option<&str>, local: &str) -> bool {
        self.name().namespace.as_deref() == namespace && self.name().local.as_ref() == local
    }
}
