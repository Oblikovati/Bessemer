use super::*;

pub(super) fn owned_groups() -> Vec<SimulationShellGroup> {
    let mut cursor = EXPECTED;
    let mut groups = Vec::new();
    while !cursor.is_empty() {
        groups.push(read_group(&mut cursor));
        projection::consume_fields(&mut cursor);
    }
    assert_eq!(groups.len(), 72);
    groups
}

fn ordinal(group: &SimulationShellGroup, path: &str) -> usize {
    group
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap()
}

fn assert_noop(group: &SimulationShellGroup, index: usize) {
    let parent = group.field_id(index).unwrap();
    let edit = group
        .edit_field(&parent, group.fields()[index].value.clone())
        .unwrap();
    assert_eq!(edit.result(), parent);
    assert_eq!(edit.archive().to_bytes(), group.to_bytes());
    assert_eq!(
        edit.coverage().completion,
        SimulationShellGroupCompletion::Unchanged
    );
    for (ordinal, row) in edit.history().iter().enumerate() {
        assert_eq!(row.relation, SimulationShellGroupRelation::Copied);
        assert_eq!(
            row.parents.as_ref(),
            &[SimulationShellGroupHistoryParent::Source(
                group.field_id(ordinal).unwrap()
            )]
        );
        assert_eq!(row.result, group.field_id(ordinal).unwrap());
    }
}

#[test]
fn every_owned_field_noop_preserves_exact_mfc_frames_ids_and_all_copied_parents() {
    for group in owned_groups() {
        for index in 0..group.fields().len() {
            assert_noop(&group, index);
        }
    }
}

#[test]
fn revision_ownership_rejects_foreign_stale_and_out_of_bounds_handles() {
    for group in owned_groups() {
        assert_group_ownership(&group);
    }
}

fn fresh_group(group: &SimulationShellGroup, raw: &[u8]) -> SimulationShellGroup {
    SimulationShellGroup::from_bytes(raw, group.version(), group.wire_profile(), LIMITS).unwrap()
}

fn assert_group_ownership(group: &SimulationShellGroup) {
    let independent = fresh_group(group, &group.to_bytes());
    assert!(independent.field(&group.field_id(0).unwrap()).is_err());
    assert!(group.field_id(usize::MAX).is_err());
    if group.coverage().record_count == 0 {
        return;
    }
    let original = group.field_id(ordinal(group, "record0.word28")).unwrap();
    let edit = group
        .edit_field(&original, SimulationShellGroupValue::Signed(17))
        .unwrap();
    assert!(edit.archive().field(&original).is_err());
    assert!(group.field(&edit.result()).is_err());
    assert!(group.field(&original).is_ok());
    assert!(edit.archive().field(&edit.result()).is_ok());
}

fn incompatible(value: &SimulationShellGroupValue) -> SimulationShellGroupValue {
    match value {
        SimulationShellGroupValue::Signed(_) => SimulationShellGroupValue::DoubleBits(0),
        SimulationShellGroupValue::DoubleBits(_) => {
            SimulationShellGroupValue::Text(ArchiveString::new("", TextEncoding::Latin1).unwrap())
        }
        SimulationShellGroupValue::Text(_) => SimulationShellGroupValue::Signed(0),
    }
}

#[test]
fn every_value_kind_rejects_incompatible_replacements_without_mutating_inputs() {
    for group in owned_groups() {
        let original = group.to_bytes();
        for (index, field) in group.fields().iter().enumerate() {
            let result =
                group.edit_field(&group.field_id(index).unwrap(), incompatible(&field.value));
            assert_eq!(result.unwrap_err().class, bessemer::FailureClass::Invalid);
            assert_eq!(group.to_bytes(), original);
        }
    }
}

fn invalid_field_sets(group: &SimulationShellGroup) -> [Vec<SimulationShellGroupField>; 4] {
    let mut path = group.fields().to_vec();
    path[0].path = "unbound-group-count".into();
    let mut kind = group.fields().to_vec();
    kind[0].value = SimulationShellGroupValue::DoubleBits(0);
    let mut missing = group.fields().to_vec();
    missing.pop();
    let mut extra = group.fields().to_vec();
    extra.push(group.fields()[0].clone());
    [path, kind, missing, extra]
}

#[test]
fn complete_construction_rejects_wrong_paths_kinds_and_missing_or_extra_fields() {
    for group in owned_groups() {
        for fields in invalid_field_sets(&group) {
            assert!(
                SimulationShellGroup::from_fields(
                    fields,
                    group.version(),
                    group.wire_profile(),
                    LIMITS
                )
                .is_err()
            );
        }
    }
}

#[test]
fn mutable_input_and_caller_field_vectors_are_isolated_after_success_and_failure() {
    for group in owned_groups() {
        assert_input_isolation(&group);
    }
}

#[test]
fn failed_text_reparse_preserves_original_bytes_fields_and_revision_after_partial_validation() {
    let group = owned_groups().remove(1);
    let original = group.to_bytes();
    let limits = SimulationShellGroupLimits {
        maximum_text_bytes: group.coverage().text_bytes,
        ..LIMITS
    };
    let source =
        SimulationShellGroup::from_bytes(&original, group.version(), group.wire_profile(), limits)
            .unwrap();
    let parent = source
        .field_id(ordinal(&source, "record0.string1"))
        .unwrap();
    let (_, replacement) = typed_group_option(2);
    let error = source.edit_field(&parent, replacement).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert_eq!(error.entity, "record0.string1");
    assert_eq!(source.to_bytes(), original);
    assert_eq!(source.fields(), group.fields());
    assert!(source.field(&parent).is_ok());
}

fn construct_caller_fields(
    group: &SimulationShellGroup,
    fields: &[SimulationShellGroupField],
) -> SimulationShellGroup {
    SimulationShellGroup::from_fields(
        fields.to_vec(),
        group.version(),
        group.wire_profile(),
        LIMITS,
    )
    .unwrap()
}

fn assert_failed_count_requests(group: &SimulationShellGroup) {
    let parent = group.field_id(0).unwrap();
    assert!(
        group
            .edit_field(&parent, SimulationShellGroupValue::DoubleBits(0))
            .is_err()
    );
    assert!(
        group
            .edit_field(&parent, SimulationShellGroupValue::Signed(-1))
            .is_err()
    );
}

fn mutate_caller_fields(fields: &mut [SimulationShellGroupField]) {
    fields[0].path = "mutated-after-construction".into();
    fields[0].value = SimulationShellGroupValue::Signed(-1);
    if let Some(field) = fields
        .iter_mut()
        .find(|field| matches!(field.value, SimulationShellGroupValue::Text(_)))
    {
        field.value = SimulationShellGroupValue::Text(
            ArchiveString::new("caller changed", TextEncoding::Utf16).unwrap(),
        );
    }
}

fn assert_input_isolation(group: &SimulationShellGroup) {
    let mut input = group.to_bytes();
    let source = fresh_group(group, &input);
    let mut fields = source.fields().to_vec();
    let constructed = construct_caller_fields(&source, &fields);
    assert_failed_count_requests(&source);
    mutate_caller_fields(&mut fields);
    input.fill(0x5a);
    assert_eq!(source.to_bytes(), group.to_bytes());
    assert_eq!(constructed.to_bytes(), group.to_bytes());
    assert_eq!(constructed.fields(), group.fields());
}

fn request(
    group: &SimulationShellGroup,
    option: usize,
) -> (SimulationShellGroupFieldId, SimulationShellGroupValue) {
    if group.coverage().record_count == 0 {
        return (
            group.field_id(0).unwrap(),
            SimulationShellGroupValue::Signed(0),
        );
    }
    let (path, value) = typed_group_option(option);
    (group.field_id(ordinal(group, path)).unwrap(), value)
}

fn edit_signature(group: &SimulationShellGroup, option: usize) -> Vec<u8> {
    let (parent, value) = request(group, option);
    let edit = group.edit_field(&parent, value).unwrap();
    let mut output = edit.archive().to_bytes();
    projection::project_fields(&mut output, edit.archive());
    let source_ids: Vec<_> = (0..group.fields().len())
        .map(|index| group.field_id(index).unwrap())
        .collect();
    let result_ids: Vec<_> = (0..edit.archive().fields().len())
        .map(|index| edit.archive().field_id(index).unwrap())
        .collect();
    load_projection::project_outer_history(&mut output, edit.history(), &source_ids, &result_ids);
    output
}

fn assert_parallel_options(
    groups: &[SimulationShellGroup],
    expected: &[[Vec<u8>; 3]],
    worker: usize,
) {
    for iteration in 0..32 {
        let source = (worker * 32 + iteration) % groups.len();
        let option = (worker * 7 + iteration) % 3;
        assert_eq!(
            edit_signature(&groups[source], option),
            expected[source][option]
        );
    }
}

#[test]
fn mixed_threads_match_all_serial_bytes_fields_and_ordered_parents_across_both_profiles() {
    let groups = std::sync::Arc::new(owned_groups());
    let expected: Vec<_> = groups
        .iter()
        .map(|group| [0, 1, 2].map(|option| edit_signature(group, option)))
        .collect();
    std::thread::scope(|scope| {
        for worker in 0..8 {
            let groups = groups.clone();
            let expected = &expected;
            scope.spawn(move || assert_parallel_options(&groups, expected, worker));
        }
    });
}

fn typed_group_option(option: usize) -> (&'static str, SimulationShellGroupValue) {
    match option {
        0 => ("record0.word28", SimulationShellGroupValue::Signed(17)),
        1 => (
            "record0.double30",
            SimulationShellGroupValue::DoubleBits(0x7ff8000000000042),
        ),
        2 => (
            "record0.string1",
            SimulationShellGroupValue::Text(
                ArchiveString::new("parallel\0Ω", TextEncoding::Utf16).unwrap(),
            ),
        ),
        _ => panic!("option {option}; expected three explicit typed requests"),
    }
}
