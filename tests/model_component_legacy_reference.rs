//! Ordinary direct legacy-reference operations against independent Source goldens (#206).
#[path = "support/component_legacy_cases.rs"]
mod component_legacy_cases;
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;

use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;
use component_legacy_cases::*;
use owned_resolved_document::*;

const TIMES: [u32; 3] = [0, 0x80000000, u32::MAX];
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn legacy_document(raw: &[u8]) -> Document {
    owned_saved_manager_document::prepared_saved_manager_document(raw)
}

fn legacy_archive(document: &Document) -> PartResolvedFeaturesArchive {
    read_owned_resolved_document(document, LIMITS)
}

#[test]
fn all_owned_legacy_bodies_keep_raw_values_targets_map_slots_and_complete_construction() {
    let mut checked = 0;
    for case in component_legacy_cases()
        .iter()
        .filter(|case| case.context == 1)
    {
        assert_eq!(case.version, 19000);
        let source = legacy_archive(&legacy_document(case.raw));
        assert_eq!(source.to_bytes().unwrap(), case.raw);
        assert_legacy_values(&source, case);
        let constructed = source.edit_fields(source.fields().to_vec()).unwrap();
        assert_eq!(constructed.completion(), ModelEditCompletion::Unchanged);
        assert_eq!(constructed.archive().to_bytes().unwrap(), case.raw);
        checked += 1;
    }
    assert_eq!(checked, 27);
}

fn assert_legacy_values(source: &PartResolvedFeaturesArchive, case: &ComponentLegacyCase<'_>) {
    assert_legacy_map_slots(source, case);
    let presence = source
        .fields()
        .iter()
        .find(|field| field.path.ends_with(".unknown_84"))
        .unwrap();
    assert_eq!(presence.value, ModelFieldValue::I32(case.presence));
    let time = source
        .fields()
        .iter()
        .find(|field| field.path.ends_with(".legacy_reference.time_word"));
    if case.presence == 0 {
        assert!(time.is_none());
        return;
    }
    let expected = TIMES[case.kind] ^ u32::from(case.edited);
    assert_eq!(time.unwrap().value, ModelFieldValue::U32(expected));
}

fn assert_legacy_map_slots(source: &PartResolvedFeaturesArchive, case: &ComponentLegacyCase<'_>) {
    let expected_slots = if case.presence != 0 && case.kind == 2 {
        101
    } else {
        99
    };
    assert_eq!(source.object_slots(), (97, expected_slots));
    let count = if case.presence != 0 && case.kind > 0 {
        2
    } else {
        1
    };
    assert_eq!(source.root_graph().unwrap().roots().len(), count);
}

fn legacy_time_edit(source: &PartResolvedFeaturesArchive, value: u32) -> PartResolvedFeaturesEdit {
    let mut fields = source.fields().to_vec();
    let time = fields
        .iter_mut()
        .find(|field| field.path.ends_with(".legacy_reference.time_word"))
        .unwrap();
    time.value = ModelFieldValue::U32(value);
    source.edit_fields(fields).unwrap()
}

#[test]
fn twelve_expected_time_edits_reopen_and_freshly_reverse_complete_documents() {
    let cases = component_legacy_cases();
    for case in cases
        .iter()
        .filter(|case| case.context == 1 && case.presence != 0 && !case.edited)
    {
        let expected = cases
            .iter()
            .find(|other| {
                other.context == 1
                    && other.presence == case.presence
                    && other.kind == case.kind
                    && other.edited
            })
            .unwrap();
        assert_legacy_document_edit(case, expected);
    }
}

fn assert_legacy_document_edit(case: &ComponentLegacyCase<'_>, expected: &ComponentLegacyCase<'_>) {
    let document = legacy_document(case.raw);
    let source = legacy_archive(&document);
    let changed = legacy_time_edit(&source, TIMES[case.kind] ^ 1);
    assert_eq!(changed.completion(), ModelEditCompletion::Changed);
    assert_eq!(changed.changes().len(), 1);
    assert_eq!(changed.archive().to_bytes().unwrap(), expected.raw);
    reverse_legacy_document(&document, changed.archive(), TIMES[case.kind]);
    assert_eq!(source.to_bytes().unwrap(), case.raw);
}

fn reverse_legacy_document(
    document: &Document,
    changed: &PartResolvedFeaturesArchive,
    original_time: u32,
) {
    let mut output = document.clone();
    replace_owned_resolved_stream(&mut output, changed.to_bytes().unwrap());
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let current = legacy_archive(&reopened);
    assert_eq!(current.fields(), changed.fields());
    unselected_document_streams::assert_unselected_document_streams(
        document,
        &reopened,
        OWNED_RESOLVED_STREAM,
    );
    let reversed = legacy_time_edit(&current, original_time);
    replace_owned_resolved_stream(&mut output, reversed.archive().to_bytes().unwrap());
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}

#[test]
fn incomplete_legacy_streams_fail_without_changing_valid_inputs() {
    for case in component_legacy_cases()
        .iter()
        .filter(|case| case.context == 1 && !case.edited)
    {
        let source = legacy_archive(&legacy_document(case.raw));
        for raw in [&case.raw[..case.raw.len() - 1], &[case.raw, &[0]].concat()] {
            let result = PartResolvedFeaturesArchive::from_document(
                &legacy_document(raw),
                ConfigurationId::new(0),
                ArchiveVersion::new(19000),
                None,
                LIMITS,
            );
            assert!(result.is_err());
        }
        assert_eq!(source.to_bytes().unwrap(), case.raw);
    }
}
