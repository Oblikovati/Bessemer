//! Pin complete equation field storage and document edits before role projections (#199).
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;
use bessemer::Document;
use bessemer::model_archives::*;
use owned_resolved_document::*;

const EQUATION_LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const EQUATION_FLAGS: [u32; 8] = [4, 5, 0x80000004, 0x80000005, 0x34, 0x24, 0, 1];
const EQUATION_OPTIONS: [i32; 6] = [i32::MIN, -1, 0, 1, 2, i32::MAX];
const OWNED_EQUATIONS: [[&[u8]; 8]; 2] = [
    [
        include_bytes!("fixtures/spline-equation-state/owned-0-0.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-0-1.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-0-2.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-0-3.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-0-4.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-0-5.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-0-6.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-0-7.bin"),
    ],
    [
        include_bytes!("fixtures/spline-equation-state/owned-1-0.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-1-1.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-1-2.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-1-3.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-1-4.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-1-5.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-1-6.bin"),
        include_bytes!("fixtures/spline-equation-state/owned-1-7.bin"),
    ],
];

const EQUATION_LOCK_GOLDENS: [&[u8]; 2] = [
    include_bytes!("fixtures/spline-equation-state/owned-0-0-edited.bin"),
    include_bytes!("fixtures/spline-equation-state/owned-1-0-edited.bin"),
];

#[test]
fn complete_owned_spline_equation_fields_match_literal_saved_roles() {
    for (kind, roots) in OWNED_EQUATIONS.iter().enumerate() {
        for (style, raw) in roots.iter().enumerate() {
            let document = equation_fixture_document(raw);
            let source = read_owned_resolved_document(&document, EQUATION_LIMITS);
            assert_eq!(source.to_bytes().unwrap(), *raw);
            assert_eq!(source.object_slots(), (97, 99));
            assert_eq!(
                equation_value(&source, "flags_32c"),
                Some(&ModelFieldValue::U32(EQUATION_FLAGS[style]))
            );
            assert_eq!(
                equation_value(&source, "stored_category_mask"),
                Some(&ModelFieldValue::U16(if kind == 0 { 1 } else { 0x80 }))
            );
            assert_equation_words(&source, style);
        }
    }
}

fn equation_value<'a>(
    source: &'a PartResolvedFeaturesArchive,
    suffix: &str,
) -> Option<&'a ModelFieldValue> {
    let ending = format!(".splines[0].{suffix}");
    source
        .fields()
        .iter()
        .find(|field| field.path.ends_with(&ending))
        .map(|field| &field.value)
}

fn assert_equation_words(source: &PartResolvedFeaturesArchive, style: usize) {
    if style >= 6 {
        assert!(equation_value(source, "equation_word").is_none());
        assert!(equation_value(source, "unknown_1e0").is_none());
        assert!(equation_value(source, "equation_strings[0]").is_none());
        return;
    }
    assert_eq!(
        equation_value(source, "equation_word"),
        Some(&ModelFieldValue::I32(i32::MIN))
    );
    assert_eq!(
        equation_value(source, "unknown_1e0"),
        Some(&ModelFieldValue::I32(EQUATION_OPTIONS[style]))
    );
    assert_equation_strings(source);
}

fn assert_equation_strings(source: &PartResolvedFeaturesArchive) {
    for index in 0..6 {
        let value = equation_value(source, &format!("equation_strings[{index}]"));
        let Some(ModelFieldValue::Text(text)) = value else {
            panic!("{value:?}; expected saved equation CString")
        };
        assert_eq!(text.text(), format!("Ω equation {index}"));
    }
}

#[test]
fn existing_archive_edit_reopens_and_reverses_the_complete_owned_document() {
    for (kind, golden) in EQUATION_LOCK_GOLDENS.iter().enumerate() {
        let document = equation_fixture_document(OWNED_EQUATIONS[kind][0]);
        let source = read_owned_resolved_document(&document, EQUATION_LIMITS);
        let mut fields = source.fields().to_vec();
        let selected = fields
            .iter_mut()
            .find(|field| field.path.ends_with(".splines[0].flags_32c"))
            .unwrap();
        selected.value = ModelFieldValue::U32(4 ^ 0x10);
        let edit = source.edit_fields(fields).unwrap();
        assert_eq!(edit.completion(), ModelEditCompletion::Changed);
        assert_eq!(edit.archive().to_bytes().unwrap(), *golden);
        assert_eq!(edit.changes().len(), 1);
        assert_eq!(edit.unchanged_fields(), source.fields().len() - 1);
        assert_reopened_equation_document(&document, &source, &edit);
    }
}

fn assert_reopened_equation_document(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    edit: &PartResolvedFeaturesEdit,
) {
    let original = document.to_bytes().unwrap();
    let mut output = document.clone();
    replace_owned_resolved_stream(&mut output, edit.archive().to_bytes().unwrap());
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let current = read_owned_resolved_document(&reopened, EQUATION_LIMITS);
    assert_eq!(current.fields(), edit.archive().fields());
    unselected_document_streams::assert_unselected_document_streams(
        document,
        &reopened,
        OWNED_RESOLVED_STREAM,
    );
    let reversed = current.edit_fields(source.fields().to_vec()).unwrap();
    replace_owned_resolved_stream(&mut output, reversed.archive().to_bytes().unwrap());
    assert_eq!(output.to_bytes().unwrap(), original);
    assert_eq!(
        source.to_bytes().unwrap(),
        document.read_stream(OWNED_RESOLVED_STREAM).unwrap()
    );
}

fn equation_fixture_document(raw: &[u8]) -> Document {
    owned_saved_manager_document::prepared_saved_manager_document(raw)
}

#[path = "model_spline_equation_state/typed.rs"]
mod typed;

#[path = "support/cstring_framing_fixture.rs"]
mod cstring_framing_fixture;
#[path = "model_spline_equation_state/matrix.rs"]
mod matrix;

#[path = "model_spline_equation_state/concurrency.rs"]
mod concurrency;

#[path = "model_spline_equation_state/exports.rs"]
mod exports;

#[path = "model_spline_equation_state/aliases.rs"]
mod aliases;
