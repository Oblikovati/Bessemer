use super::*;

fn verify_classification_history(
    source: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
    result: &PartResolvedFeaturesEdit,
    selector: u16,
) {
    let changed = selector != state.classification_selector();
    assert_eq!(result.changes().len(), usize::from(changed));
    assert_eq!(
        result.unchanged_fields() + result.changes().len(),
        source.fields().len()
    );
    if !changed {
        verify_classification_identity(state, result);
        return;
    }
    verify_classification_change(source, state, result, selector);
}

fn verify_classification_identity(
    state: &ModelIntersectionReferenceStoredState,
    result: &PartResolvedFeaturesEdit,
) {
    assert_eq!(result.completion(), ModelEditCompletion::Unchanged);
    let original = result
        .archive()
        .intersection_reference_state(&state.source_fields()[0])
        .unwrap();
    assert_eq!(&original, state);
}

fn verify_classification_change(
    source: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
    result: &PartResolvedFeaturesEdit,
    selector: u16,
) {
    let change = &result.changes()[0];
    let original = source.resolve_field(&state.source_fields()[0]).unwrap();
    let replacement = result
        .archive()
        .resolve_field(change.result().unwrap())
        .unwrap();
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(change.parents(), &state.source_fields()[..1]);
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(change.path(), original.path.as_ref());
    assert_eq!(replacement.value, ModelFieldValue::U16(selector));
    assert!(
        result
            .archive()
            .edit_intersection_reference_classification(state, selector)
            .is_err()
    );
}

fn verify_classification_edit(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &ModelIntersectionReferenceStoredState,
    offset: usize,
    selector: u16,
) -> PartResolvedFeaturesEdit {
    let result = source
        .edit_intersection_reference_classification(state, selector)
        .unwrap();
    let mut literal = raw.to_vec();
    literal[offset..offset + 2].copy_from_slice(&selector.to_le_bytes());
    assert_eq!(result.archive().to_bytes().unwrap(), literal);
    let mut fields = source.fields().to_vec();
    fields[state.source_fields()[0].index()].value = ModelFieldValue::U16(selector);
    assert_eq!(result.archive().fields(), fields);
    verify_classification_history(source, state, &result, selector);
    assert_eq!(
        result.coverage().mode,
        ModelEditCheckMode::ExactStoredArchive
    );
    assert_eq!(result.coverage().wire_bytes, raw.len());
    assert_eq!(result.coverage().typed_fields, fields.len());
    assert_eq!(result.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(result.diagnostics(), source.diagnostics());
    verify_reopened_classification(&result, state, selector);
    result
}

fn verify_reopened_classification(
    result: &PartResolvedFeaturesEdit,
    state: &ModelIntersectionReferenceStoredState,
    selector: u16,
) {
    let leading = result
        .archive()
        .field_handle(state.source_fields()[0].index())
        .unwrap();
    let reopened = result
        .archive()
        .intersection_reference_state(&leading)
        .unwrap();
    assert_eq!(reopened.classification_selector(), selector);
    assert_eq!(reopened.words().raw_word_20, state.words().raw_word_20);
    assert_eq!(
        reopened.words().span_count_state,
        state.words().span_count_state
    );
    assert_eq!(
        reopened.words().backward_offset_state,
        state.words().backward_offset_state
    );
}

#[test]
fn every_u16_classification_selector_preserves_complete_archive_bytes_fields_and_history() {
    let (context, manager) = intersection_context();
    let original = context.to_bytes().unwrap();
    let source = intersection_archive(CONTROL, &context, &manager);
    let state = source.intersection_reference_states().unwrap().remove(0);
    let output_path = std::env::var_os("BESSEMER_CLASSIFICATION_EXPORT");
    let mut output = output_path
        .as_ref()
        .map(|_| 65536_u32.to_le_bytes().to_vec());
    for selector in 0..=u16::MAX {
        let result = verify_classification_edit(&source, CONTROL, &state, 440, selector);
        append_intersection_export(&mut output, &result.archive().to_bytes().unwrap());
    }
    assert_eq!(source.to_bytes().unwrap(), CONTROL);
    assert_eq!(context.to_bytes().unwrap(), original);
    write_intersection_export(output_path, output);
}

#[test]
fn classification_rejects_foreign_revisions_without_changing_either_archive() {
    let (context, manager) = intersection_context();
    let source = intersection_archive(CONTROL, &context, &manager);
    let foreign = intersection_archive(CONTROL, &context, &manager);
    let state = source.intersection_reference_states().unwrap().remove(0);
    let failure = foreign
        .edit_intersection_reference_classification(&state, 65535)
        .unwrap_err();
    assert_eq!(failure.class, bessemer::FailureClass::Invalid);
    assert_eq!(source.to_bytes().unwrap(), CONTROL);
    assert_eq!(foreign.to_bytes().unwrap(), CONTROL);
}

fn verify_classification_matrix_root(source: &PartResolvedFeaturesArchive, raw: &[u8]) {
    let states = source.intersection_reference_states().unwrap();
    assert_eq!(states.len(), 8);
    for (state, offset) in states
        .iter()
        .zip([440, 460, 480, 500, 1187, 1207, 1227, 1247])
    {
        for selector in [0, 65535, state.classification_selector() ^ 0x55aa] {
            verify_classification_edit(source, raw, state, offset, selector);
        }
    }
    assert_eq!(source.to_bytes().unwrap(), raw);
}

#[test]
fn complete_owned_role_matrix_classification_edits_preserve_all_signed_index_and_sibling_controls()
{
    let (context, manager) = intersection_context();
    let original = context.to_bytes().unwrap();
    let mut cursor = ROLE_PACKET;
    for _ in 0..264 {
        let raw = next_intersection_image(&mut cursor);
        let source = intersection_archive(raw, &context, &manager);
        verify_classification_matrix_root(&source, raw);
    }
    assert_eq!(next_intersection_image(&mut cursor).len(), 1672584);
    assert!(cursor.is_empty());
    assert_eq!(context.to_bytes().unwrap(), original);
}

pub(super) fn full_selector_domain_archive() -> (PartResolvedFeaturesArchive, &'static [u8]) {
    let (context, manager) = intersection_context();
    let mut cursor = ROLE_PACKET;
    for _ in 0..264 {
        next_intersection_image(&mut cursor);
    }
    let raw = next_intersection_image(&mut cursor);
    assert!(cursor.is_empty());
    let limits = ModelArchiveLimits {
        maximum_bytes: 64 * 1024 * 1024,
        maximum_fields: 2_000_000,
        maximum_entries: 2_000_000,
        maximum_depth: 128,
        maximum_objects: 100_000,
    };
    let source = PartResolvedFeaturesArchive::from_bytes(raw, context, &manager, limits).unwrap();
    (source, raw)
}

fn full_selector_domain_fields(source: &PartResolvedFeaturesArchive) -> Vec<(usize, u16)> {
    source
        .fields()
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            if !field.path.contains(".styled_intersections.intersections[")
                || !field.path.contains(".edges_48[")
                || !field.path.ends_with(".raw_00")
            {
                return None;
            }
            let ModelFieldValue::U16(value) = field.value else {
                panic!("{:?}; expected u16 classification selector", field.value);
            };
            Some((index, value))
        })
        .collect()
}

fn final_full_domain_group_offset() -> usize {
    include_str!("../fixtures/intersection-index-classification/scalars.tsv")
        .lines()
        .filter(|row| row.starts_with("264\tgroup\t"))
        .nth(65535)
        .unwrap()
        .split('\t')
        .nth(2)
        .unwrap()
        .parse()
        .unwrap()
}

#[test]
fn complete_full_u16_owner_reconstructs_every_selector_and_edits_its_last_physical_group() {
    let (source, raw) = full_selector_domain_archive();
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(source.object_slots(), (97, 100));
    assert_eq!(source.root_count(), 3);
    let fields = full_selector_domain_fields(&source);
    assert_eq!(fields.len(), 65540);
    for (value, (_, actual)) in fields[..65536].iter().enumerate() {
        assert_eq!(*actual, u16::try_from(value).unwrap());
    }
    let leading = source.field_handle(fields[65535].0).unwrap();
    let state = source.intersection_reference_state(&leading).unwrap();
    assert_eq!(state.classification_selector(), 65535);
    verify_classification_edit(
        &source,
        raw,
        &state,
        final_full_domain_group_offset(),
        0x5aa5,
    );
    assert_eq!(source.to_bytes().unwrap(), raw);
}
