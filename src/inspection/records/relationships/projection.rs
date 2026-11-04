use super::*;
use crate::archive_codec::ArchiveRevision;
use crate::metadata::{OrderedXmlView, Utf8XmlDocument, XmlContent};
use std::collections::BTreeSet;

pub(super) fn attempts(
    xml: &Utf8XmlDocument,
    revision: &ArchiveRevision,
    sheet: usize,
    policy: InspectionRelationshipPolicy,
) -> Result<Vec<InspectionRelationshipAttempt>> {
    let ordered = xml.ordered_view().ok_or_else(|| {
        record_error(
            FailureClass::Missing,
            "ordered XML capture absent; expected complete relationship traversal".to_owned(),
        )
    })?;
    Ok(collect_attempts(&ordered, revision, sheet, policy))
}

fn collect_attempts(
    xml: &OrderedXmlView<'_>,
    revision: &ArchiveRevision,
    sheet: usize,
    policy: InspectionRelationshipPolicy,
) -> Vec<InspectionRelationshipAttempt> {
    let dictionary = first_child(xml, sheet, "PackageOptions")
        .and_then(|package| first_child(xml, package, "RelationshipsDictionary"));
    let Some(container) = dictionary else {
        return Vec::new();
    };
    xml.content(container)
        .iter()
        .enumerate()
        .filter(|(_, content)| {
            policy == InspectionRelationshipPolicy::RetainedNodes
                || matches!(content, XmlContent::Element(_))
        })
        .map(|(content_index, content)| {
            make_attempt(xml, revision, container, content_index, content)
        })
        .collect()
}

fn make_attempt(
    xml: &OrderedXmlView<'_>,
    revision: &ArchiveRevision,
    container: usize,
    content_index: usize,
    content: &XmlContent,
) -> InspectionRelationshipAttempt {
    let mut attempt = InspectionRelationshipAttempt {
        id: InspectionRelationshipNodeId {
            revision: revision.clone(),
            container,
            content_index,
        },
        kind: node_kind(content),
        element_name: None,
        key: Arc::from(""),
        values: Arc::from([]),
    };
    if let XmlContent::Element(element) = content {
        attempt.element_name = Some(xml.qualified_name(*element).into());
        let (key, values) = entry_fields(xml, *element);
        attempt.key = key;
        attempt.values = values.into();
    }
    attempt
}

fn node_kind(content: &XmlContent) -> InspectionRelationshipNodeKind {
    use InspectionRelationshipNodeKind::*;
    match content {
        XmlContent::Element(_) => Element,
        XmlContent::Text(_) => Text,
        XmlContent::CData(_) => CData,
        XmlContent::Comment => Comment,
        XmlContent::ProcessingInstruction => ProcessingInstruction,
    }
}

fn entry_fields(xml: &OrderedXmlView<'_>, element: usize) -> (Arc<str>, Vec<Arc<str>>) {
    let mut key = Arc::from("");
    let mut values = Vec::new();
    for child in xml.children(element) {
        match qt_trim(xml.qualified_name(*child)) {
            "key" => {
                key = first_child(xml, *child, "string")
                    .map(|value| Arc::from(qt_trim(&xml.descendant_text(value))))
                    .unwrap_or_else(|| Arc::from(""))
            }
            "value" => append_values(xml, *child, &mut values),
            _ => {}
        }
    }
    (key, values)
}

fn append_values(xml: &OrderedXmlView<'_>, field: usize, values: &mut Vec<Arc<str>>) {
    if let Some(array) = first_child(xml, field, "ArrayOfString") {
        values.extend(
            xml.children(array)
                .iter()
                .map(|child| Arc::from(qt_trim(&xml.descendant_text(*child)))),
        );
    }
}

fn first_child(xml: &OrderedXmlView<'_>, parent: usize, name: &str) -> Option<usize> {
    xml.children(parent)
        .iter()
        .copied()
        .find(|child| xml.qualified_name(*child) == name)
}

pub(super) fn first_insertions(
    attempts: &[InspectionRelationshipAttempt],
) -> Vec<InspectionRelationshipAttempt> {
    let mut seen = BTreeSet::new();
    attempts
        .iter()
        .filter(|attempt| seen.insert(attempt.key.encode_utf16().collect::<Vec<_>>()))
        .cloned()
        .collect()
}

fn qt_trim(value: &str) -> &str {
    value.trim_matches(|character| matches!(character, '\u{0009}'..='\u{000d}' | ' ' | '\u{0085}' | '\u{00a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}'))
}
