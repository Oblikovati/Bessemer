use super::{
    XmlAttribute, XmlLimits, attributes, declaration, encoding, grammar, invalid,
    namespaces::{NamePool, NamespaceScope, ParsedAttribute, qualified},
    types::{AttributeRecord, ElementRecord, XmlMetrics, XmlModel},
    unsupported,
};
use crate::Result;
use quick_xml::{
    Reader,
    events::{BytesRef, BytesStart, Event},
};
use std::{collections::BTreeSet, ops::Range, sync::Arc};

struct Frame {
    ordinal: usize,
    declared: Vec<Arc<str>>,
}
struct XmlBuilder<'a> {
    source: &'a str,
    limits: XmlLimits,
    model: XmlModel,
    pool: NamePool,
    scope: NamespaceScope,
    stack: Vec<Frame>,
}

pub(super) fn parse_document(source: &str, limits: XmlLimits) -> Result<XmlModel> {
    grammar::check_characters(source)?;
    let mut reader = Reader::from_str(source);
    reader.config_mut().enable_all_checks(true);
    let mut builder = XmlBuilder::new(source, limits)?;
    loop {
        let start = reader.buffer_position() as usize;
        let event = reader.read_event().map_err(|error| {
            invalid(format!(
                "XML at byte {start}: {error}; expected well-formed document"
            ))
        })?;
        if matches!(event, Event::Eof) {
            return builder.finish();
        }
        builder.charge()?;
        builder.consume(event, start..reader.buffer_position() as usize)?;
    }
}

impl<'a> XmlBuilder<'a> {
    fn new(source: &'a str, limits: XmlLimits) -> Result<Self> {
        encoding::require_size(1, limits.maximum_nodes as usize, "XML document frame")?;
        let model = XmlModel {
            metrics: XmlMetrics {
                nodes: 1,
                ..XmlMetrics::default()
            },
            ..XmlModel::default()
        };
        Ok(Self {
            source,
            limits,
            model,
            pool: NamePool::new(),
            scope: NamespaceScope::new(),
            stack: Vec::new(),
        })
    }

    fn charge(&mut self) -> Result<()> {
        let nodes = self
            .model
            .metrics
            .nodes
            .checked_add(1)
            .ok_or_else(|| invalid("XML event count overflow; expected u32"))?;
        encoding::require_size(
            nodes as usize,
            self.limits.maximum_nodes as usize,
            "XML lexical events",
        )?;
        self.model.metrics.nodes = nodes;
        Ok(())
    }

    fn consume(&mut self, event: Event<'_>, span: Range<usize>) -> Result<()> {
        match event {
            Event::Start(start) => self.open(start, span, false),
            Event::Empty(start) => self.open(start, span, true),
            Event::End(_) => self.close(span.end),
            Event::Text(text) => self.text(&text, false),
            Event::CData(text) => self.text(&text, true),
            Event::GeneralRef(reference) => self.reference(reference),
            Event::Decl(decl) => declaration::validate(&decl, span.start),
            Event::DocType(_) => Err(unsupported(
                "XML DTD declaration; expected metadata without DTD or external entities",
            )),
            Event::PI(pi) => grammar::processing_target(pi.target()),
            Event::Comment(_) | Event::Eof => Ok(()),
        }
    }

    fn open(&mut self, start: BytesStart<'_>, span: Range<usize>, empty: bool) -> Result<()> {
        if self.stack.is_empty() && !self.model.elements.is_empty() {
            return Err(invalid("multiple XML roots; expected one document element"));
        }
        let depth = self.stack.len() + 1;
        encoding::require_size(depth, self.limits.maximum_depth, "XML element depth")?;
        let name = qualified(&mut self.pool, start.name().as_ref())?;
        let parsed = self.read_attributes(&start)?;
        let declared = self.scope.push(&parsed, &mut self.pool)?;
        let name = self.scope.resolve(&name, true)?;
        let attributes = self.resolve_attributes(parsed)?;
        let ordinal = self.publish_element(name, attributes, span, depth);
        if empty {
            self.scope.pop(declared);
        } else {
            self.stack.push(Frame { ordinal, declared });
        }
        Ok(())
    }

    fn read_attributes(&mut self, start: &BytesStart<'_>) -> Result<Vec<ParsedAttribute>> {
        attributes::parse_attributes(
            start,
            self.source,
            &mut self.pool,
            &mut self.model.metrics.attributes,
            self.limits,
        )
    }

    fn publish_element(
        &mut self,
        name: super::ExpandedName,
        attributes: Vec<AttributeRecord>,
        span: Range<usize>,
        depth: usize,
    ) -> usize {
        let parent = self.stack.last().map(|frame| frame.ordinal);
        let ordinal = self.model.elements.len();
        self.model.elements.push(ElementRecord {
            name,
            attributes,
            parent,
            children: Vec::new(),
            direct_text: String::new(),
            span,
        });
        if let Some(parent) = parent {
            self.model.elements[parent].children.push(ordinal);
        }
        self.model.metrics.depth = self.model.metrics.depth.max(depth);
        ordinal
    }

    fn resolve_attributes(&self, parsed: Vec<ParsedAttribute>) -> Result<Vec<AttributeRecord>> {
        let mut attributes = Vec::new();
        let mut names = BTreeSet::new();
        for attribute in parsed {
            if attribute.declaration().is_some() {
                continue;
            }
            let name = self.scope.resolve(&attribute.name, false)?;
            if !names.insert((name.namespace.clone(), name.local.clone())) {
                return Err(invalid(format!(
                    "duplicate expanded XML attribute {name:?}; expected unique namespace/local pairs"
                )));
            }
            attributes.push(attribute_record(name, attribute));
        }
        Ok(attributes)
    }

    fn close(&mut self, end: usize) -> Result<()> {
        let frame = self.stack.pop().ok_or_else(|| {
            invalid("XML closing tag without open element; expected matching opening tag")
        })?;
        self.model.elements[frame.ordinal].span.end = end;
        self.scope.pop(frame.declared);
        Ok(())
    }

    fn text(&mut self, text: &str, cdata: bool) -> Result<()> {
        if !cdata && text.contains("]]>") {
            return Err(invalid(
                "literal ]]> in XML text; expected escaped closing delimiter",
            ));
        }
        if self.stack.is_empty() {
            if cdata || !text.chars().all(grammar::space) {
                return Err(invalid(format!(
                    "XML content outside root {text:?}; expected XML whitespace"
                )));
            }
            return Ok(());
        }
        let normalized = quick_xml::events::BytesText::from_escaped(text)
            .xml10_content()
            .into_owned();
        self.append(&normalized)
    }

    fn reference(&mut self, reference: BytesRef<'_>) -> Result<()> {
        let value = reference.resolve_char_ref().map_err(|error| {
            invalid(format!(
                "XML reference {error}; expected valid numeric character"
            ))
        })?;
        if let Some(character) = value {
            return self.append(&character.to_string());
        }
        self.append(predefined_reference(reference.as_ref())?)
    }

    fn append(&mut self, value: &str) -> Result<()> {
        grammar::check_characters(value)?;
        let frame = self.stack.last().ok_or_else(|| {
            invalid(format!(
                "XML character content {value:?} outside root; expected active element"
            ))
        })?;
        self.model.elements[frame.ordinal]
            .direct_text
            .push_str(value);
        Ok(())
    }

    fn finish(self) -> Result<XmlModel> {
        if !self.stack.is_empty() || self.model.elements.is_empty() {
            return Err(invalid(format!(
                "XML has {} open elements and {} total elements; expected one closed root",
                self.stack.len(),
                self.model.elements.len()
            )));
        }
        Ok(self.model)
    }
}

fn attribute_record(name: super::ExpandedName, attribute: ParsedAttribute) -> AttributeRecord {
    AttributeRecord {
        visible: XmlAttribute {
            name,
            value: attribute.value,
        },
        span: attribute.span,
    }
}

fn predefined_reference(name: &str) -> Result<&'static str> {
    match name {
        "amp" => Ok("&"),
        "lt" => Ok("<"),
        "gt" => Ok(">"),
        "apos" => Ok("'"),
        "quot" => Ok("\""),
        other => Err(invalid(format!(
            "XML entity {other:?}; expected predefined or numeric reference"
        ))),
    }
}
