//! Complete direct-boundary operations against independent Source goldens (#205).
#[path = "support/boundary_direct_cases.rs"]
mod boundary_direct_cases;
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;

use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;
use boundary_direct_cases::*;
use owned_resolved_document::*;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn boundary_document(raw: &[u8]) -> Document {
    owned_saved_manager_document::prepared_saved_manager_document(raw)
}

fn boundary_archive(
    document: &Document,
    limits: ModelArchiveLimits,
) -> bessemer::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        limits,
    )
}

#[test]
fn all_owned_boundary_graphs_force_typed_construction_without_new_object_slots() {
    let mut checked = 0;
    for case in boundary_direct_cases()
        .iter()
        .filter(|case| case.context == 1)
    {
        assert_eq!(case.version, 19000);
        let document = boundary_document(case.raw);
        let source = read_owned_resolved_document(&document, LIMITS);
        assert_eq!(source.object_slots(), (97, 99));
        assert_eq!(source.to_bytes().unwrap(), case.raw);
        let constructed = source.edit_fields(source.fields().to_vec()).unwrap();
        assert_eq!(constructed.completion(), ModelEditCompletion::Unchanged);
        assert_eq!(constructed.archive().fields(), source.fields());
        assert_eq!(constructed.archive().to_bytes().unwrap(), case.raw);
        checked += 1;
    }
    assert_eq!(checked, 17);
}

fn boundary_edit_suffix(style: usize) -> &'static str {
    if style == 3 {
        return ".temporary_records[0].time_0";
    }
    ".direct_model_data.unknown_38"
}

fn edit_boundary_value(
    source: &PartResolvedFeaturesArchive,
    style: usize,
    edited: bool,
) -> PartResolvedFeaturesEdit {
    let mut fields = source.fields().to_vec();
    let selected = fields
        .iter_mut()
        .find(|field| field.path.ends_with(boundary_edit_suffix(style)))
        .unwrap();
    selected.value = if style == 3 {
        ModelFieldValue::U32(if edited { 0x11223344 } else { u32::MAX })
    } else {
        ModelFieldValue::I32(if edited { 42 } else { -17 })
    };
    source.edit_fields(fields).unwrap()
}

#[test]
fn all_seven_expected_edits_reopen_and_freshly_reverse_complete_documents() {
    let cases = boundary_direct_cases();
    for case in cases
        .iter()
        .filter(|case| case.context == 1 && !case.edited && case.style >= 3)
    {
        let expected = cases
            .iter()
            .find(|other| other.context == 1 && other.edited && other.style == case.style)
            .unwrap();
        assert_boundary_document_edit(case, expected);
    }
}

fn assert_boundary_document_edit(case: &BoundaryDirectCase<'_>, expected: &BoundaryDirectCase<'_>) {
    let document = boundary_document(case.raw);
    let source = boundary_archive(&document, LIMITS).unwrap();
    let changed = edit_boundary_value(&source, case.style, true);
    assert_eq!(changed.completion(), ModelEditCompletion::Changed);
    assert_eq!(changed.changes().len(), 1);
    assert_eq!(changed.archive().to_bytes().unwrap(), expected.raw);
    reverse_boundary_document(&document, changed.archive(), case.style);
    assert_eq!(source.to_bytes().unwrap(), case.raw);
}

fn reverse_boundary_document(
    document: &Document,
    changed: &PartResolvedFeaturesArchive,
    style: usize,
) {
    let mut output = document.clone();
    replace_owned_resolved_stream(&mut output, changed.to_bytes().unwrap());
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let current = boundary_archive(&reopened, LIMITS).unwrap();
    assert_eq!(current.fields(), changed.fields());
    unselected_document_streams::assert_unselected_document_streams(
        document,
        &reopened,
        OWNED_RESOLVED_STREAM,
    );
    let reverse = edit_boundary_value(&current, style, false);
    replace_owned_resolved_stream(&mut output, reverse.archive().to_bytes().unwrap());
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}

#[test]
fn incomplete_or_oversized_direct_bodies_never_publish_and_keep_inputs_unchanged() {
    for case in boundary_direct_cases()
        .iter()
        .filter(|case| case.context == 1 && !case.edited)
    {
        let source = boundary_archive(&boundary_document(case.raw), LIMITS).unwrap();
        for raw in [&case.raw[..case.raw.len() - 1], &[case.raw, &[0]].concat()] {
            assert!(boundary_archive(&boundary_document(raw), LIMITS).is_err());
        }
        for suffix in [
            ".temporary_count",
            ".direct_model_data.pair_count",
            ".direct_model_data.value_count",
        ] {
            assert_oversized_boundary_count(&source, suffix);
        }
        assert_eq!(source.to_bytes().unwrap(), case.raw);
    }
}

fn assert_oversized_boundary_count(source: &PartResolvedFeaturesArchive, suffix: &str) {
    let mut fields = source.fields().to_vec();
    let Some(selected) = fields.iter_mut().find(|field| field.path.ends_with(suffix)) else {
        return;
    };
    selected.value = ModelFieldValue::I32(i32::MAX);
    assert!(source.edit_fields(fields).is_err());
}
