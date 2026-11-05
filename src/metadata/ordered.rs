use super::{Utf8XmlDocument, XmlContent, types::XmlOrderedContent};

pub(crate) struct OrderedXmlView<'a> {
    xml: &'a Utf8XmlDocument,
    ordered: &'a XmlOrderedContent,
}

impl<'a> OrderedXmlView<'a> {
    pub(super) fn new(xml: &'a Utf8XmlDocument, ordered: &'a XmlOrderedContent) -> Self {
        Self { xml, ordered }
    }

    pub(crate) fn qualified_name(&self, ordinal: usize) -> &str {
        &self.ordered.qualified_names[ordinal]
    }

    pub(crate) fn content(&self, ordinal: usize) -> &[XmlContent] {
        &self.ordered.content[ordinal]
    }

    pub(crate) fn children(&self, ordinal: usize) -> &[usize] {
        self.xml.children(ordinal)
    }

    pub(crate) fn descendant_text(&self, ordinal: usize) -> String {
        let mut pending: Vec<_> = self.content(ordinal).iter().rev().collect();
        let mut text = String::new();
        while let Some(content) = pending.pop() {
            match content {
                XmlContent::Text(value) | XmlContent::CData(value) => text.push_str(value),
                XmlContent::Element(child) => pending.extend(self.content(*child).iter().rev()),
                XmlContent::Comment | XmlContent::ProcessingInstruction => {}
            }
        }
        text
    }
}
