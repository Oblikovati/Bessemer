use super::{XmlElementRef, XmlMetadata};

const VALUE_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";

/// One section's properties and aliases; property IDs are local to this section.
#[derive(Debug, Clone, Copy)]
pub struct PropertySection<'a> {
    element: XmlElementRef<'a>,
}

/// One complete property, without reducing its values or interpreting its stored text.
#[derive(Debug, Clone, Copy)]
pub struct MetadataProperty<'a> {
    element: XmlElementRef<'a>,
}

/// A stored typed value, FPVals container, or unknown value element.
#[derive(Debug, Clone, Copy)]
pub struct PropertyValue<'a> {
    element: XmlElementRef<'a>,
}

/// Observed value-element identity, without inventing value or enum constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyValueKind {
    /// Stored vt:i2 text.
    I2,
    /// Stored vt:i4 text.
    I4,
    /// Stored vt:r8 text.
    R8,
    /// Stored vt:lpstr text.
    LpStr,
    /// Stored vt:bool text, including observed spelling No.
    Bool,
    /// Stored vt:empty element.
    Empty,
    /// Stored vt:blob text, with uninterpreted blob encoding.
    Blob,
    /// Unqualified FPVals container with all nested value children.
    FpVals,
    /// Additional value whose expanded name remains available on its element.
    Other,
}

impl XmlMetadata {
    /// Iterates unqualified sections only in the three documented property-section families.
    ///
    /// ```no_run
    /// # fn example(metadata:&bessemer::metadata::XmlMetadata){
    /// for section in metadata.property_sections(){let name=section.element().attribute(None,"name");}
    /// # }
    /// ```
    pub fn property_sections(&self) -> impl Iterator<Item = PropertySection<'_>> {
        self.root()
            .filter(|_| self.stream().family().property_sections())
            .into_iter()
            .flat_map(|root| root.children())
            .filter(|element| element.named(None, "propertySection"))
            .map(|element| PropertySection { element })
    }
}

impl<'a> PropertySection<'a> {
    /// Borrows the complete section, including raw name/fmtid and unknown attributes.
    ///
    /// ```no_run
    /// # fn example(section:bessemer::metadata::PropertySection<'_>){
    /// let fmtid=section.element().attribute(None,"fmtid");
    /// # }
    /// ```
    pub fn element(self) -> XmlElementRef<'a> {
        self.element
    }

    /// Iterates every property in source order, without document-wide ID deduplication.
    ///
    /// ```no_run
    /// # fn example(section:bessemer::metadata::PropertySection<'_>){
    /// for property in section.properties(){let pid=property.element().attribute(None,"pid");}
    /// # }
    /// ```
    pub fn properties(self) -> impl Iterator<Item = MetadataProperty<'a>> {
        self.element
            .children()
            .filter(|element| element.named(None, "property"))
            .map(|element| MetadataProperty { element })
    }

    /// Retains all dictionary entries, including empty names and aliases sharing a property ID.
    ///
    /// ```no_run
    /// # fn example(section:bessemer::metadata::PropertySection<'_>){
    /// for alias in section.dictionary_entries(){let name=alias.attribute(None,"name");}
    /// # }
    /// ```
    pub fn dictionary_entries(self) -> impl Iterator<Item = XmlElementRef<'a>> {
        self.element
            .children()
            .filter(|element| element.named(None, "propertyNameDictionaryElement"))
    }
}

impl<'a> MetadataProperty<'a> {
    /// Borrows every property attribute, including raw TypeID/IsEquation spellings.
    ///
    /// ```no_run
    /// # fn example(property:bessemer::metadata::MetadataProperty<'_>){
    /// let name=property.element().attribute(None,"name");
    /// # }
    /// ```
    pub fn element(self) -> XmlElementRef<'a> {
        self.element
    }

    /// Iterates all value children, including FPVals and additional unknown values.
    ///
    /// ```no_run
    /// # fn example(property:bessemer::metadata::MetadataProperty<'_>){
    /// for value in property.values(){let kind=value.kind();}
    /// # }
    /// ```
    pub fn values(self) -> impl ExactSizeIterator<Item = PropertyValue<'a>> {
        self.element
            .children()
            .map(|element| PropertyValue { element })
    }
}

impl<'a> PropertyValue<'a> {
    /// Borrows exact stored text and identity for a scoped immutable value edit.
    ///
    /// ```no_run
    /// # fn example(value:bessemer::metadata::PropertyValue<'_>){
    /// let stored=value.element().direct_text();
    /// # }
    /// ```
    pub fn element(self) -> XmlElementRef<'a> {
        self.element
    }

    /// Identifies the observed XML value type without parsing its lexical value.
    ///
    /// ```no_run
    /// # fn example(value:bessemer::metadata::PropertyValue<'_>){
    /// let stored_kind=value.kind();
    /// # }
    /// ```
    pub fn kind(self) -> PropertyValueKind {
        use PropertyValueKind::*;
        if self.element.named(None, "FPVals") {
            return FpVals;
        }
        if self.element.name().namespace.as_deref() != Some(VALUE_NAMESPACE) {
            return Other;
        }
        match self.element.name().local.as_ref() {
            "i2" => I2,
            "i4" => I4,
            "r8" => R8,
            "lpstr" => LpStr,
            "bool" => Bool,
            "empty" => Empty,
            "blob" => Blob,
            _ => Other,
        }
    }

    /// Retains nested value elements rather than flattening FPVals to its first child.
    ///
    /// ```no_run
    /// # fn example(value:bessemer::metadata::PropertyValue<'_>){
    /// for nested in value.children(){let stored=nested.element().direct_text();}
    /// # }
    /// ```
    pub fn children(self) -> impl ExactSizeIterator<Item = PropertyValue<'a>> {
        self.element
            .children()
            .map(|element| PropertyValue { element })
    }
}
