use super::*;

pub(super) fn ordinal(header: &SimulationSolidManagerProperty, path: &str) -> usize {
    header
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap()
}

pub(super) fn replacements() -> Vec<(&'static str, SimulationSolidManagerValue)> {
    let text = SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 32).unwrap();
    vec![
        ("member48", SimulationSolidManagerValue::Signed(12345)),
        ("listb0.0", SimulationSolidManagerValue::Text(text)),
    ]
}

pub(super) fn replace(
    header: &SimulationSolidManagerProperty,
    path: &str,
    value: SimulationSolidManagerValue,
) -> SimulationSolidManagerEdit {
    header
        .edit_field(&header.field_id(ordinal(header, path)).unwrap(), value)
        .unwrap()
}

pub(super) fn assert_history(edit: &SimulationSolidManagerEdit) {
    assert_eq!(edit.history().len(), edit.source().fields().len());
    for (index, row) in edit.history().iter().enumerate() {
        let derived = index == edit.parent().ordinal();
        let mut parents = vec![SimulationSolidManagerHistoryParent::Source(
            edit.source().field_id(index).unwrap(),
        )];
        if derived {
            parents.push(SimulationSolidManagerHistoryParent::Replacement);
        }
        assert_eq!(row.parents.as_ref(), parents);
        assert_eq!(row.result, edit.archive().field_id(index).unwrap());
        assert_eq!(
            row.relation,
            if derived {
                SimulationSolidManagerRelation::Derived
            } else {
                SimulationSolidManagerRelation::Copied
            }
        );
    }
}

pub(super) fn assert_preserved(edit: &SimulationSolidManagerEdit) {
    let index = edit.parent().ordinal();
    let before = edit.source().to_bytes();
    let after = edit.archive().to_bytes();
    let (start, end) = edit.source().field_offsets()[index];
    let (next_start, next_end) = edit.archive().field_offsets()[index];
    assert_eq!(start, next_start);
    assert_eq!(before[..start], after[..next_start]);
    assert_eq!(before[end..], after[next_end..]);
    for (ordinal, field) in edit.source().fields().iter().enumerate() {
        if ordinal != index {
            assert_eq!(field, &edit.archive().fields()[ordinal]);
        }
    }
}

#[test]
fn signed_and_text_edits_keep_complete_source_replacement_result_and_all_parents() {
    let source = owned_header(870008, 5);
    let original = source.to_bytes();
    for (path, value) in replacements() {
        let edit = replace(&source, path, value.clone());
        assert_eq!(edit.replacement(), &value);
        assert_eq!(edit.source().to_bytes(), original);
        assert_eq!(edit.archive().field(&edit.result()).unwrap().value, value);
        assert_eq!(
            edit.parent(),
            source.field_id(ordinal(&source, path)).unwrap()
        );
        assert_eq!(
            edit.coverage().completion,
            SimulationSolidManagerCompletion::Changed
        );
        assert_eq!(edit.coverage().requested_pattern_error_bits, 0);
        assert_history(&edit);
        assert_preserved(&edit);
    }
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn every_exact_owned_noop_retains_all_original_field_identities() {
    let mut cursor = EXPECTED;
    while !cursor.is_empty() {
        let header = read_original(&mut cursor);
        consume_expected_fields(&mut cursor);
        for (index, field) in header.fields().iter().enumerate() {
            let id = header.field_id(index).unwrap();
            let edit = header.edit_field(&id, field.value.clone()).unwrap();
            assert_eq!(edit.result(), id);
            assert_eq!(edit.archive().to_bytes(), header.to_bytes());
            assert_eq!(
                edit.coverage().completion,
                SimulationSolidManagerCompletion::Unchanged
            );
            assert!(edit.history().iter().all(|row| row.parents.len() == 1));
        }
    }
}

fn text_at<'a>(
    header: &'a SimulationSolidManagerProperty,
    path: &str,
) -> &'a SimulationPersistText {
    let SimulationSolidManagerValue::Text(text) = &header.fields()[ordinal(header, path)].value
    else {
        panic!("{path}; expected counted text");
    };
    text
}

#[test]
fn null_allocated_empty_unpaired_and_odd_frames_remain_distinct_and_unchanged() {
    let header = owned_header(870008, 5);
    assert!(text_at(&header, "listb0.0").is_null());
    assert!(!text_at(&header, "listb0.1").is_null());
    assert_eq!(text_at(&header, "listb0.1").utf16_units(), Some(vec![]));
    assert_eq!(
        text_at(&header, "listb0.2").utf16_units(),
        Some(vec![0xd800])
    );
    let odd = text_at(&header, "listb0.3");
    assert!(!odd.is_null());
    assert_eq!(odd.utf16_units(), None);
    assert_eq!(odd.payload(), &[0x33, 0, 0]);
    assert_eq!(odd.source_parent(), header.to_bytes());
}

#[test]
fn source_input_buffers_returned_bytes_and_copied_fields_cannot_mutate_snapshots() {
    let expected = owned_header(870008, 5);
    let original = expected.to_bytes();
    let mut input = original.clone();
    let header =
        SimulationSolidManagerProperty::from_bytes(&input, expected.version(), LIMITS).unwrap();
    input.fill(0);
    let mut returned = header.to_bytes();
    returned.fill(255);
    let mut fields = header.fields().to_vec();
    fields[0].path = "changed".into();
    fields.clear();
    assert_eq!(header.to_bytes(), original);
    assert_eq!(header.fields(), expected.fields());
}

#[test]
fn invalid_structural_kinds_foreign_stale_and_fabricated_requests_preserve_source() {
    let header = owned_header(870008, 5);
    let original = header.to_bytes();
    let id = header.field_id(0).unwrap();
    let foreign = owned_header(870008, 5).field_id(0).unwrap();
    assert!(header.field(&foreign).is_err());
    assert!(header.field_id(header.fields().len()).is_err());
    assert!(
        header
            .edit_field(&id, SimulationSolidManagerValue::Unsigned(1))
            .is_err()
    );
    assert_invalid_count(&header);
    let edit = header
        .edit_field(&id, SimulationSolidManagerValue::Signed(123))
        .unwrap();
    assert!(edit.archive().field(&id).is_err());
    assert_wrong_construction(&header);
    assert_eq!(header.to_bytes(), original);
}

fn assert_wrong_construction(header: &SimulationSolidManagerProperty) {
    let mut fields = header.fields().to_vec();
    fields[0].path = "fabricated.role".into();
    let error =
        SimulationSolidManagerProperty::from_fields(fields, header.version(), LIMITS).unwrap_err();
    assert!(error.cause.contains("fabricated.role") && error.cause.contains("member48"));
    let mut fields = header.fields().to_vec();
    fields[0].value = SimulationSolidManagerValue::Unsigned(0x80000000);
    let error =
        SimulationSolidManagerProperty::from_fields(fields, header.version(), LIMITS).unwrap_err();
    assert!(
        error.cause.contains("Unsigned(2147483648)") && error.cause.contains("Signed(-2147483648)")
    );
}

fn assert_invalid_count(header: &SimulationSolidManagerProperty) {
    let count = header.field_id(ordinal(header, "listb0.count")).unwrap();
    assert!(
        header
            .edit_field(&count, SimulationSolidManagerValue::Signed(-1))
            .is_err()
    );
}
