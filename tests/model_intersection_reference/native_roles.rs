use super::*;
use crate::model_archives::{ModelEditCheckMode, ModelEditFidelity, ModelFieldHandle, ModelFieldRelation, PartResolvedFeaturesEdit};

pub(super) fn native_index_oracles(index: u32) -> Vec<(usize, i32)> {
    let raw = std::fs::read("target/intersection-role-native-index-spans.bin").unwrap();
    let mut cursor = ArchiveCursor::new(&raw);
    assert_eq!(cursor.take(4).unwrap(), b"BRX1");
    assert_eq!(cursor.u32().unwrap(), 8);
    let mut selected = None;
    for _ in 0..8 {
        let case = cursor.u32().unwrap();
        let count = cursor.u32().unwrap();
        let spans = (0..count).map(|_| {
            let offset = cursor.u32().unwrap() as usize;
            (offset, i32::from_le_bytes(cursor.take(4).unwrap().try_into().unwrap()))
        }).collect();
        if case == index { selected = Some(spans); }
    }
    assert_eq!(cursor.remaining(), 0);
    selected.unwrap()
}

pub(super) fn verify_native_role_payload(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    parent: &ModelFieldHandle,
    replacement: ModelFieldValue,
    patch: (usize, &[u8]),
    result: &PartResolvedFeaturesEdit,
) {
    let mut literal = raw.to_vec();
    literal[patch.0..patch.0 + patch.1.len()].copy_from_slice(patch.1);
    assert_eq!(result.archive().to_bytes().unwrap(), literal);
    let mut fields = source.fields().to_vec();
    fields[parent.index()].value = replacement;
    assert_eq!(result.archive().fields(), fields);
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(result.unchanged_fields() + 1, fields.len());
    verify_native_role_history(source, parent, result);
}

fn verify_native_role_history(source: &PartResolvedFeaturesArchive, parent: &ModelFieldHandle, result: &PartResolvedFeaturesEdit) {
    assert_eq!(result.changes().len(), 1);
    let change = &result.changes()[0];
    assert_eq!(change.parents(), std::slice::from_ref(parent));
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(change.path(), source.resolve_field(parent).unwrap().path.as_ref());
    assert!(result.archive().resolve_field(change.result().unwrap()).is_ok());
    assert!(result.archive().resolve_field(parent).is_err());
    assert_eq!(result.diagnostics(), source.diagnostics());
    assert_eq!(result.coverage().mode, ModelEditCheckMode::ExactStoredArchive);
    assert_eq!(result.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(result.coverage().wire_bytes, source.to_bytes().unwrap().len());
    assert_eq!(result.coverage().typed_fields, source.fields().len());
}

fn verify_native_selectors(source: &PartResolvedFeaturesArchive, case: &ResolvedCorpusCase<'_>, oracle: &NativeReferenceOracle) {
    let states = source.intersection_reference_states().unwrap();
    assert_eq!(states.len(), oracle.spans.len());
    for (state, span) in states.iter().zip(&oracle.spans) {
        assert_eq!(state.words(), span.words);
        let value = state.classification_selector() ^ 0xa55a;
        let result = source.edit_intersection_reference_classification(state, value).unwrap();
        verify_native_role_payload(source, case.features, &state.source_fields()[0], ModelFieldValue::U16(value), (span.offset, &value.to_le_bytes()), &result);
        assert!(result.archive().edit_intersection_reference_classification(state, value).is_err());
    }
}

fn verify_native_indices(source: &PartResolvedFeaturesArchive, case: &ResolvedCorpusCase<'_>, oracle: &NativeReferenceOracle) {
    let expected = native_index_oracles(oracle.index);
    let indices = source.intersection_edge_indices().unwrap();
    assert_eq!(indices.len(), expected.len());
    for (index, (offset, original)) in indices.iter().zip(expected) {
        assert_eq!(index.value(), original);
        let value = !original;
        let result = source.edit_intersection_edge_index(index, value).unwrap();
        verify_native_role_payload(source, case.features, index.source_field(), ModelFieldValue::I32(value), (offset, &value.to_le_bytes()), &result);
        assert!(result.archive().edit_intersection_edge_index(index, value).is_err());
    }
}

fn verify_native_stored_role_case(case: &ResolvedCorpusCase<'_>, oracle: &NativeReferenceOracle, kernel: Option<CompressedKernelContext>) {
    assert_eq!(case.label, oracle.label);
    assert_eq!(case.version.value(), oracle.version);
    let configuration = configuration_for_case(case, kernel).unwrap();
    let source = PartResolvedFeaturesArchive::from_bytes(case.features, configuration, case.manager, LIMITS).unwrap();
    verify_native_selectors(&source, case, oracle);
    verify_native_indices(&source, case, oracle);
    assert_eq!(source.to_bytes().unwrap(), case.features);
    println!("INTERSECTION_NATIVE_ROLES\t{}\t{}\t{}", oracle.index, oracle.spans.len(), oracle.spans.len() / 4);
}

#[test]
#[ignore = "Requires retained native BRF1/group/index packets and explicit kernel schema"]
fn every_native_stored_index_and_selector_matches_source_spans_and_literal_edits() {
    let probe = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let spans = std::fs::read("target/intersection-reference-native-source-spans.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    run_native_reference_cohort(&probe, &spans, kernel, verify_native_stored_role_case);
}
