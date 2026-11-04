use super::*;

const SIGNED_INDEX_CONTROLS: [i32; 8] =
    [i32::MIN, i32::MIN + 1, -2, -1, 0, 1, i32::MAX - 1, i32::MAX];

fn verify_index_history(
    source: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionEdgeStoredIndex,
    result: &PartResolvedFeaturesEdit,
    value: i32,
) {
    let changed = state.value() != value;
    assert_eq!(result.changes().len(), usize::from(changed));
    assert_eq!(
        result.unchanged_fields() + result.changes().len(),
        source.fields().len()
    );
    if !changed {
        assert_eq!(result.completion(), ModelEditCompletion::Unchanged);
        assert_eq!(
            &result
                .archive()
                .intersection_edge_index(state.source_field())
                .unwrap(),
            state
        );
        return;
    }
    verify_changed_index_history(source, state, result, value);
}

fn verify_changed_index_history(
    source: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionEdgeStoredIndex,
    result: &PartResolvedFeaturesEdit,
    value: i32,
) {
    let change = &result.changes()[0];
    let original = source.resolve_field(state.source_field()).unwrap();
    let replacement = result
        .archive()
        .resolve_field(change.result().unwrap())
        .unwrap();
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(change.parents(), std::slice::from_ref(state.source_field()));
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(change.path(), original.path.as_ref());
    assert_eq!(replacement.value, ModelFieldValue::I32(value));
    assert!(
        result
            .archive()
            .edit_intersection_edge_index(state, value)
            .is_err()
    );
}

pub(super) fn verify_index_edit(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &ModelIntersectionEdgeStoredIndex,
    offset: usize,
    value: i32,
) -> PartResolvedFeaturesEdit {
    let result = source.edit_intersection_edge_index(state, value).unwrap();
    let mut expected = raw.to_vec();
    expected[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    assert_eq!(result.archive().to_bytes().unwrap(), expected);
    let mut fields = source.fields().to_vec();
    fields[state.source_field().index()].value = ModelFieldValue::I32(value);
    assert_eq!(result.archive().fields(), fields);
    verify_index_history(source, state, &result, value);
    contracts::verify_reference_coverage(source, &result, raw.len());
    verify_rebound_index(&result, state, value);
    result
}

fn verify_rebound_index(
    result: &PartResolvedFeaturesEdit,
    state: &ModelIntersectionEdgeStoredIndex,
    value: i32,
) {
    let leading = result
        .archive()
        .field_handle(state.source_field().index())
        .unwrap();
    assert_eq!(
        result
            .archive()
            .intersection_edge_index(&leading)
            .unwrap()
            .value(),
        value
    );
}

fn verify_index_matrix_root(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    output: &mut Option<Vec<u8>>,
) {
    let states = source.intersection_edge_indices().unwrap();
    assert_eq!(states.len(), 2);
    for (state, offset) in states.iter().zip([436, 1183]) {
        assert_eq!(
            state.value(),
            i32::from_le_bytes(raw[offset..offset + 4].try_into().unwrap())
        );
        for value in SIGNED_INDEX_CONTROLS {
            let result = verify_index_edit(source, raw, state, offset, value);
            append_intersection_export(output, &result.archive().to_bytes().unwrap());
        }
    }
    assert_eq!(source.to_bytes().unwrap(), raw);
}

#[test]
fn complete_signed_index_matrix_preserves_selectors_siblings_and_exact_history() {
    let (context, manager) = intersection_context();
    let original = context.to_bytes().unwrap();
    let path = std::env::var_os("BESSEMER_EDGE_INDEX_EXPORT");
    let mut output = path.as_ref().map(|_| 264_u32.to_le_bytes().to_vec());
    let mut cursor = ROLE_PACKET;
    for _ in 0..264 {
        let raw = next_intersection_image(&mut cursor);
        let source = intersection_archive(raw, &context, &manager);
        verify_index_matrix_root(&source, raw, &mut output);
    }
    assert_eq!(next_intersection_image(&mut cursor).len(), 1672584);
    assert!(cursor.is_empty());
    assert_eq!(context.to_bytes().unwrap(), original);
    write_intersection_export(path, output);
}

#[test]
fn foreign_index_views_fail_without_modifying_either_input() {
    let (context, manager) = intersection_context();
    let first = intersection_archive(CONTROL, &context, &manager);
    let second = intersection_archive(CONTROL, &context, &manager);
    let state = first.intersection_edge_indices().unwrap().remove(0);
    assert_eq!(
        second
            .edit_intersection_edge_index(&state, -2)
            .unwrap_err()
            .class,
        bessemer::FailureClass::Invalid
    );
    assert_eq!(first.to_bytes().unwrap(), CONTROL);
    assert_eq!(second.to_bytes().unwrap(), CONTROL);
}

#[test]
fn last_edge_in_complete_full_domain_owner_accepts_arbitrary_signed_index() {
    let (source, raw) = super::classification::full_selector_domain_archive();
    let position = source
        .fields()
        .iter()
        .position(|field| field.path.ends_with(".edges_48[16383].raw_168"))
        .unwrap();
    let state = source
        .intersection_edge_index(&source.field_handle(position).unwrap())
        .unwrap();
    assert_eq!(state.value(), i32::MAX);
    let row = include_str!("../fixtures/intersection-index-classification/scalars.tsv")
        .lines()
        .filter(|row| row.starts_with("264\tindex\t"))
        .nth(16383)
        .unwrap();
    let offset = row.split('\t').nth(2).unwrap().parse().unwrap();
    verify_index_edit(&source, raw, &state, offset, i32::MIN);
    assert_eq!(source.to_bytes().unwrap(), raw);
}
