use super::{LIMITS, ResolvedCorpusCase, configuration_for_case, next_case};
use crate::archive_codec::ArchiveCursor;
use crate::model_archives::{
    CompressedKernelContext, ModelEditCompletion, ModelFieldChange, ModelFieldHandle,
    ModelFieldRelation, ModelFieldValue, ModelSketchReferenceStoredLookupEdit as LookupEdit,
    ModelSketchReferenceStoredLookupState as LookupState, PartResolvedFeaturesArchive,
    PartResolvedFeaturesEdit,
};

type NativeLookupRow = [i64; 7];
type NativeLookupCheck =
    fn(&ResolvedCorpusCase<'_>, &[&NativeLookupRow], Option<CompressedKernelContext>, bool);

#[path = "native_documents.rs"]
mod documents;

fn native_lookup_rows() -> Vec<NativeLookupRow> {
    include_str!("../fixtures/sketch-reference-configuration-keys/native-spans.tsv")
        .lines()
        .map(|line| {
            let words: Vec<i64> = line.split('\t').map(|word| word.parse().unwrap()).collect();
            words.try_into().unwrap()
        })
        .collect()
}

fn native_lookup_archive(
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
) -> PartResolvedFeaturesArchive {
    let configuration = configuration_for_case(case, kernel).unwrap();
    let source =
        PartResolvedFeaturesArchive::from_bytes(case.features, configuration, case.manager, LIMITS)
            .unwrap();
    assert_eq!(source.to_bytes().unwrap(), case.features);
    source
}

fn native_lookup_words(row: &NativeLookupRow) -> [i32; 2] {
    [
        i32::try_from(row[5]).unwrap(),
        i32::try_from(row[6]).unwrap(),
    ]
}

fn verify_native_lookup_projection(
    source: &PartResolvedFeaturesArchive,
    state: &LookupState,
    row: &NativeLookupRow,
    raw: &[u8],
) {
    let expected = native_lookup_words(row);
    let keys = state.keys();
    assert_eq!(
        [keys.configuration_object_key, keys.configuration_record_key],
        expected
    );
    for (role, handle) in state.source_fields().iter().enumerate() {
        let offset = usize::try_from(row[3 + role]).unwrap();
        assert_eq!(&raw[offset..offset + 4], &expected[role].to_le_bytes());
        assert_eq!(
            source.resolve_field(handle).unwrap().value,
            ModelFieldValue::I32(expected[role])
        );
    }
    let rebound = source
        .sketch_reference_lookup_state(&state.source_fields()[0])
        .unwrap();
    assert_eq!(rebound, *state);
}

fn native_lookup_edit(row: &NativeLookupRow, kind: usize) -> (LookupEdit, [i32; 2]) {
    let [object, record] = native_lookup_words(row);
    match kind {
        0 => (
            LookupEdit::ConfigurationObjectKey(!object),
            [!object, record],
        ),
        1 => (
            LookupEdit::ConfigurationRecordKey(!record),
            [object, !record],
        ),
        2 => (
            LookupEdit::BothKeys {
                configuration_object_key: !object,
                configuration_record_key: !record,
            },
            [!object, !record],
        ),
        _ => panic!("native lookup request kind {kind}; expected 0..3"),
    }
}

fn verify_native_lookup_history(
    source: &PartResolvedFeaturesArchive,
    state: &LookupState,
    result: &PartResolvedFeaturesEdit,
    expected: [i32; 2],
) {
    let changed: Vec<_> = state
        .source_fields()
        .iter()
        .zip(expected)
        .filter(|(handle, value)| {
            source.resolve_field(handle).unwrap().value != ModelFieldValue::I32(*value)
        })
        .collect();
    assert_eq!(result.changes().len(), changed.len());
    for (relation, (parent, value)) in result.changes().iter().zip(changed) {
        verify_native_lookup_relation(source, result, relation, parent, value);
    }
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(result.diagnostics(), source.diagnostics());
    assert_eq!(
        result.coverage().wire_bytes,
        source.to_bytes().unwrap().len()
    );
    assert_eq!(result.coverage().typed_fields, source.fields().len());
}

fn verify_native_lookup_edit(
    source: &PartResolvedFeaturesArchive,
    state: &LookupState,
    row: &NativeLookupRow,
    raw: &[u8],
    kind: usize,
) -> PartResolvedFeaturesEdit {
    let (request, expected) = native_lookup_edit(row, kind);
    let result = source
        .edit_sketch_reference_lookup_keys(state, request)
        .unwrap();
    verify_native_lookup_payload(source, state, row, raw, &result, expected);
    verify_native_lookup_history(source, state, &result, expected);
    verify_native_lookup_stale(&result, state, request);
    assert_eq!(source.to_bytes().unwrap(), raw);
    result
}

fn verify_native_lookup_case(
    case: &ResolvedCorpusCase<'_>,
    rows: &[&NativeLookupRow],
    kernel: Option<CompressedKernelContext>,
    edit: bool,
) {
    let source = native_lookup_archive(case, kernel);
    let states = source.sketch_reference_lookup_states().unwrap();
    assert_eq!(states.len(), rows.len(), "{}", case.label);
    for (state, row) in states.iter().zip(rows) {
        assert_eq!(case.version.value() as i64, row[1]);
        assert_eq!(case.features.len() as i64, row[2]);
        verify_native_lookup_projection(&source, state, row, case.features);
        if edit {
            for kind in 0..3 {
                verify_native_lookup_edit(&source, state, row, case.features, kind);
            }
        }
    }
    assert_eq!(source.to_bytes().unwrap(), case.features);
}

fn run_native_lookup_corpus(check: NativeLookupCheck, edit: bool) {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let rows = native_lookup_rows();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(cursor.u32().unwrap(), 906);
    let mut total = 0;
    for index in 0..906 {
        let case = next_case(&mut cursor);
        let selected: Vec<_> = rows.iter().filter(|row| row[0] == index).collect();
        check(&case, &selected, kernel.clone(), edit);
        total += selected.len();
        println!("LOOKUP_NATIVE\t{index}\t{}\t{edit}", selected.len());
    }
    assert_eq!(cursor.remaining(), 0);
    assert_eq!(total, 6863);
}

#[test]
#[ignore = "Requires retained906-case corpus and explicit kernel schema"]
fn native_lookup_views_match_all_recorded_signed_spans() {
    run_native_lookup_corpus(verify_native_lookup_case, false);
}

#[test]
#[ignore = "Requires retained906-case corpus and explicit kernel schema; performs20589 edits"]
fn native_lookup_edits_match_all_recorded_literal_spans() {
    run_native_lookup_corpus(verify_native_lookup_case, true);
}

fn verify_native_lookup_relation(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    relation: &ModelFieldChange,
    parent: &ModelFieldHandle,
    value: i32,
) {
    assert_eq!(relation.relation(), ModelFieldRelation::Derived);
    assert_eq!(relation.parents(), std::slice::from_ref(parent));
    assert_eq!(
        relation.path(),
        source.resolve_field(parent).unwrap().path.as_ref()
    );
    let selected = result
        .archive()
        .resolve_field(relation.result().unwrap())
        .unwrap();
    assert_eq!(selected.value, ModelFieldValue::I32(value));
}

fn verify_native_lookup_payload(
    source: &PartResolvedFeaturesArchive,
    state: &LookupState,
    row: &NativeLookupRow,
    raw: &[u8],
    result: &PartResolvedFeaturesEdit,
    expected: [i32; 2],
) {
    let mut literal = raw.to_vec();
    let mut fields = source.fields().to_vec();
    for (role, value) in expected.into_iter().enumerate() {
        let offset = usize::try_from(row[3 + role]).unwrap();
        literal[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        fields[state.source_fields()[role].index()].value = ModelFieldValue::I32(value);
    }
    assert_eq!(result.archive().to_bytes().unwrap(), literal);
    assert_eq!(result.archive().fields(), fields);
}

fn verify_native_lookup_stale(
    result: &PartResolvedFeaturesEdit,
    state: &LookupState,
    request: LookupEdit,
) {
    let stale = result
        .archive()
        .resolve_field(&state.source_fields()[0])
        .unwrap_err();
    assert_eq!(
        result
            .archive()
            .edit_sketch_reference_lookup_keys(state, request)
            .unwrap_err(),
        stale
    );
}
