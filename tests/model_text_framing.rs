//! Exact CString prefix-only field edits and history, independent of semantic text equality (#200).
#[path = "support/cstring_framing_fixture.rs"]
mod cstring_framing_fixture;
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
use bessemer::Document;
use bessemer::model_archives::*;
use cstring_framing_fixture::cstring_equation_wire;
use owned_resolved_document::*;
use owned_saved_manager_document::prepared_saved_manager_document;

const FRAMING_LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
fn framing_archive(raw: &[u8]) -> PartResolvedFeaturesArchive {
    read_owned_resolved_document(&prepared_saved_manager_document(raw), FRAMING_LIMITS)
}

fn framing_field(source: &PartResolvedFeaturesArchive) -> usize {
    source
        .fields()
        .iter()
        .position(|field| field.path.ends_with(".splines[0].equation_strings[0]"))
        .unwrap()
}

#[test]
fn semantic_cstring_comparison_stays_equal_while_physical_field_framing_differs() {
    for wide in [false, true] {
        let short = framing_archive(&cstring_equation_wire(wide, 0));
        let extended = framing_archive(&cstring_equation_wire(wide, 1));
        let index = framing_field(&short);
        assert_eq!(short.fields()[index].value, extended.fields()[index].value);
        assert_ne!(short.fields()[index], extended.fields()[index]);
    }
}

#[test]
fn all_prefix_width_transitions_preserve_requested_bytes_completion_and_history() {
    for wide in [false, true] {
        for before in 0..3 {
            for after in 0..3 {
                check_framing_transition(wide, before, after);
            }
        }
    }
}

fn check_framing_transition(wide: bool, before: usize, after: usize) {
    let original = cstring_equation_wire(wide, before);
    let requested = cstring_equation_wire(wide, after);
    let source = framing_archive(&original);
    let target = framing_archive(&requested);
    let index = framing_field(&source);
    let mut fields = source.fields().to_vec();
    fields[index].value = target.fields()[index].value.clone();
    let edit = source.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), requested);
    assert_framing_contract(&source, &edit, index, before == after);
    assert_framing_reopen(&original, &source, &edit);
    assert_eq!(source.to_bytes().unwrap(), original);
}

fn assert_framing_contract(
    source: &PartResolvedFeaturesArchive,
    edit: &PartResolvedFeaturesEdit,
    index: usize,
    unchanged: bool,
) {
    assert_eq!(
        edit.completion(),
        if unchanged {
            ModelEditCompletion::Unchanged
        } else {
            ModelEditCompletion::Changed
        }
    );
    assert_eq!(edit.changes().len(), usize::from(!unchanged));
    assert_eq!(
        edit.unchanged_fields(),
        source.fields().len() - usize::from(!unchanged)
    );
    assert_framing_history(source, edit, index, unchanged);
}

fn assert_framing_reopen(
    original: &[u8],
    source: &PartResolvedFeaturesArchive,
    edit: &PartResolvedFeaturesEdit,
) {
    let mut document = prepared_saved_manager_document(original);
    let before = document.to_bytes().unwrap();
    replace_owned_resolved_stream(&mut document, edit.archive().to_bytes().unwrap());
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    let current = read_owned_resolved_document(&reopened, FRAMING_LIMITS);
    assert_eq!(current.fields(), edit.archive().fields());
    let reversed = current.edit_fields(source.fields().to_vec()).unwrap();
    replace_owned_resolved_stream(&mut document, reversed.archive().to_bytes().unwrap());
    assert_eq!(document.to_bytes().unwrap(), before);
}

fn assert_framing_history(
    source: &PartResolvedFeaturesArchive,
    edit: &PartResolvedFeaturesEdit,
    index: usize,
    unchanged: bool,
) {
    if let Some(change) = edit.changes().first() {
        assert_eq!(change.relation(), ModelFieldRelation::Derived);
        assert_eq!(
            change.parents(),
            std::slice::from_ref(&source.field_handle(index).unwrap())
        );
        assert_eq!(
            change.result(),
            Some(&edit.archive().field_handle(index).unwrap())
        );
    }
    assert_eq!(
        edit.archive()
            .resolve_field(&source.field_handle(index).unwrap())
            .is_ok(),
        unchanged
    );
}
