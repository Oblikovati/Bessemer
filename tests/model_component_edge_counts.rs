//! Complete signed component-edge count interoperability and regression oracles (#204).
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;

use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;
use owned_resolved_document::*;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

struct EdgeCountCase<'a> {
    kind: u32,
    flags: i32,
    count: i32,
    raw: &'a [u8],
}

fn edge_count_word(cursor: &mut &[u8]) -> u32 {
    let (value, tail) = cursor.split_at(4);
    *cursor = tail;
    u32::from_le_bytes(value.try_into().unwrap())
}

fn edge_count_cases() -> Vec<EdgeCountCase<'static>> {
    let mut cursor = include_bytes!("fixtures/component-edge-counts/cases.bin").as_slice();
    assert_eq!(&cursor[..4], b"BCE1");
    cursor = &cursor[4..];
    assert_eq!(edge_count_word(&mut cursor), 40);
    let cases = (0..40).map(|_| edge_count_case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    cases
}

fn edge_count_case<'a>(cursor: &mut &'a [u8]) -> EdgeCountCase<'a> {
    let kind = edge_count_word(cursor);
    let flags = edge_count_word(cursor) as i32;
    let count = edge_count_word(cursor) as i32;
    let length = edge_count_word(cursor) as usize;
    let (raw, tail) = cursor.split_at(length);
    *cursor = tail;
    EdgeCountCase {
        kind,
        flags,
        count,
        raw,
    }
}

fn edge_count_document(raw: &[u8]) -> Document {
    owned_saved_manager_document::prepared_saved_manager_document(raw)
}

fn edge_count_archive(document: &Document) -> bessemer::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
}

#[test]
fn every_owned_signed_count_preserves_optional_pairs_fields_and_complete_document_bytes() {
    for case in edge_count_cases() {
        let document = edge_count_document(case.raw);
        let original = document.to_bytes().unwrap();
        let source = read_owned_resolved_document(&document, LIMITS);
        assert_eq!(source.to_bytes().unwrap(), case.raw);
        assert_eq!(source.object_slots(), (97, 99));
        assert_edge_count_fields(&source, &case);
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

fn assert_edge_count_fields(source: &PartResolvedFeaturesArchive, case: &EdgeCountCase<'_>) {
    for (suffix, expected) in [
        (".reference_count", ModelFieldValue::I32(case.count)),
        (".unknown_188", ModelFieldValue::I32(case.flags)),
        (".unknown_18c", ModelFieldValue::U32(0xfedcba98)),
    ] {
        let field = source
            .fields()
            .iter()
            .find(|field| field.path.ends_with(suffix))
            .unwrap();
        assert_eq!(field.value, expected);
    }
    let references = edge_count_references(source, ".references[");
    assert_eq!(references.len(), case.count.max(0) as usize);
    let pair = edge_count_references(source, ".composite_references[");
    if case.flags & 6 == 6 || case.flags & 8 == 0 {
        assert!(pair.is_empty());
        return;
    }
    assert_edge_pair(&pair, case.kind);
}

fn edge_count_references<'a>(
    source: &'a PartResolvedFeaturesArchive,
    part: &str,
) -> Vec<&'a ModelObjectRecord> {
    source
        .fields()
        .iter()
        .filter_map(|field| {
            if !field.path.contains(part) {
                return None;
            }
            let ModelFieldValue::ObjectTag(tag) = &field.value else {
                panic!("expected a complete reference tag");
            };
            Some(&tag.record)
        })
        .collect()
}

fn assert_edge_pair(pair: &[&ModelObjectRecord], kind: u32) {
    assert_eq!(pair.len(), 2);
    let ModelObjectRecord::Reference { index, class } = pair[0] else {
        panic!("expected mapped self alias before null pair member");
    };
    assert_eq!(index.value(), 98);
    assert_eq!(
        class.name,
        if kind == 0 {
            "moCompEdge_c"
        } else {
            "moCompSilhEdge_c"
        }
    );
    assert!(matches!(pair[1], ModelObjectRecord::Null));
}

fn edit_edge_count(source: &PartResolvedFeaturesArchive, count: i32) -> PartResolvedFeaturesEdit {
    let mut fields = source.fields().to_vec();
    let selected = fields
        .iter_mut()
        .find(|field| field.path.ends_with(".reference_count"))
        .unwrap();
    selected.value = ModelFieldValue::I32(count);
    source.edit_fields(fields).unwrap()
}

#[test]
fn every_negative_count_edit_reopens_and_freshly_reverses_the_complete_owned_document() {
    let cases = edge_count_cases();
    for case in cases.iter().filter(|case| case.count < 0) {
        let replacement = if case.count == -1 { i32::MIN } else { -1 };
        let expected = cases
            .iter()
            .find(|other| {
                other.kind == case.kind && other.flags == case.flags && other.count == replacement
            })
            .unwrap();
        assert_negative_edge_edit(case, expected);
    }
}

fn assert_negative_edge_edit(case: &EdgeCountCase<'_>, expected: &EdgeCountCase<'_>) {
    let document = edge_count_document(case.raw);
    let source = edge_count_archive(&document).unwrap();
    let changed = edit_edge_count(&source, expected.count);
    assert_eq!(changed.completion(), ModelEditCompletion::Changed);
    assert_eq!(changed.changes().len(), 1);
    assert_eq!(changed.archive().to_bytes().unwrap(), expected.raw);
    reverse_edge_document(&document, changed.archive(), case.count);
    assert_eq!(source.to_bytes().unwrap(), case.raw);
}

fn reverse_edge_document(
    document: &Document,
    changed: &PartResolvedFeaturesArchive,
    original_count: i32,
) {
    let mut output = document.clone();
    replace_owned_resolved_stream(&mut output, changed.to_bytes().unwrap());
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let current = edge_count_archive(&reopened).unwrap();
    assert_eq!(current.fields(), changed.fields());
    unselected_document_streams::assert_unselected_document_streams(
        document,
        &reopened,
        OWNED_RESOLVED_STREAM,
    );
    let reverse = edit_edge_count(&current, original_count);
    replace_owned_resolved_stream(&mut output, reverse.archive().to_bytes().unwrap());
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}

#[test]
fn impossible_positive_counts_and_incomplete_optional_pairs_fail_without_mutation() {
    for case in edge_count_cases().iter().filter(|case| case.count < 0) {
        let document = edge_count_document(case.raw);
        let source = edge_count_archive(&document).unwrap();
        let mut fields = source.fields().to_vec();
        let selected = fields
            .iter_mut()
            .find(|field| field.path.ends_with(".reference_count"))
            .unwrap();
        selected.value = ModelFieldValue::I32(i32::MAX);
        assert!(source.edit_fields(fields).is_err());
        for raw in [&case.raw[..case.raw.len() - 1], &[case.raw, &[0]].concat()] {
            assert!(edge_count_archive(&edge_count_document(raw)).is_err());
        }
        assert_eq!(source.to_bytes().unwrap(), case.raw);
    }
}
