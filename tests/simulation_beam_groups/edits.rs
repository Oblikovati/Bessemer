use super::*;

pub(super) fn owned_body(index: usize) -> SimulationBeamGroupBody {
    let mut cursor = EXPECTED;
    while !cursor.is_empty() {
        let body = read_body(&mut cursor);
        consume_expected_fields(&mut cursor);
        if body.version().value() == 890022
            && kind_index(body.kind()) == index
            && selected_profile(&body)
        {
            return body;
        }
    }
    panic!("kind {index}; expected complete modern owned profile");
}

fn selected_profile(body: &SimulationBeamGroupBody) -> bool {
    match body.kind() {
        SimulationBeamGroupKind::JointGroup => body
            .fields()
            .iter()
            .any(|field| field.path.as_ref() == "group.references.reference0.archive0"),
        SimulationBeamGroupKind::JointList { .. } => expected_count(body.kind()) == Some(2),
        _ => true,
    }
}

pub(super) fn replacements() -> Vec<(&'static str, SimulationBeamGroupValue)> {
    use SimulationBeamGroupValue as Value;
    let text = SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 32).unwrap();
    let archive = load_strings::framed_string_archive(false, LIMITS.embedded);
    vec![
        ("group.wordd0", Value::Signed(12345)),
        ("group.doubleb8", Value::DoubleBits(0x7ff800000000abcd)),
        ("group.textc8", Value::Text(text)),
        (
            "group.references.reference0.blob",
            Value::Bytes(vec![1, 2, 3].into()),
        ),
        (
            "group.references.reference0.archive0",
            Value::Embedded(Box::new(archive)),
        ),
    ]
}

pub(super) fn replace(
    body: &SimulationBeamGroupBody,
    path: &str,
    value: SimulationBeamGroupValue,
) -> SimulationBeamGroupEdit {
    let index = body
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap();
    body.edit_field(&body.field_id(index).unwrap(), value)
        .unwrap()
}

pub(super) fn assert_history(edit: &SimulationBeamGroupEdit) {
    let requested = edit.parent().ordinal();
    assert_eq!(edit.history().len(), edit.source().fields().len());
    for (index, row) in edit.history().iter().enumerate() {
        let mut parents = vec![SimulationBeamGroupHistoryParent::Source(
            edit.source().field_id(index).unwrap(),
        )];
        if index == requested {
            parents.push(SimulationBeamGroupHistoryParent::Replacement);
        }
        assert_eq!(row.parents.as_ref(), parents);
        assert_eq!(row.result, edit.archive().field_id(index).unwrap());
        assert_eq!(
            row.relation,
            if index == requested {
                SimulationBeamGroupRelation::Derived
            } else {
                SimulationBeamGroupRelation::Copied
            }
        );
    }
}

pub(super) fn assert_isolated(edit: &SimulationBeamGroupEdit) {
    let requested = edit.parent().ordinal();
    let before = edit.source().to_bytes();
    let after = edit.archive().to_bytes();
    let (start, end) = edit.source().field_offsets()[requested];
    let (next_start, next_end) = edit.archive().field_offsets()[requested];
    assert_eq!(start, next_start);
    assert_eq!(before[..start], after[..next_start]);
    assert_eq!(before[end..], after[next_end..]);
    for (index, field) in edit.source().fields().iter().enumerate() {
        if index != requested {
            assert_eq!(field, &edit.archive().fields()[index]);
        }
    }
}

#[test]
fn all_five_physical_value_kinds_retain_inputs_outputs_and_all_outer_relations() {
    let body = owned_body(0);
    let original = body.to_bytes();
    for (path, value) in replacements() {
        let edit = replace(&body, path, value.clone());
        assert_eq!(edit.replacement(), &value);
        assert_eq!(edit.source().to_bytes(), original);
        assert_eq!(edit.archive().field(&edit.result()).unwrap().value, value);
        assert_eq!(
            edit.coverage().completion,
            SimulationBeamGroupCompletion::Changed
        );
        assert_eq!(edit.coverage().requested_pattern_error_bits, 0);
        assert_history(&edit);
        assert_isolated(&edit);
    }
    assert_eq!(body.to_bytes(), original);
}

fn embedded_edit(body: &SimulationBeamGroupBody, extended: bool) -> SimulationBeamGroupEdit {
    let archive = load_strings::framed_string_archive(extended, LIMITS.embedded);
    replace(
        body,
        "group.references.reference0.archive0",
        SimulationBeamGroupValue::Embedded(Box::new(archive)),
    )
}

#[test]
fn embedded_cstring_prefix_widths_remain_distinct_isolated_requests() {
    let body = owned_body(0);
    let first = embedded_edit(&body, false);
    let second = embedded_edit(first.archive(), true);
    let expected = load_strings::framed_string_archive(true, LIMITS.embedded);
    assert_eq!(
        second.archive().to_bytes().len(),
        first.archive().to_bytes().len() + 2
    );
    load_strings::assert_wire_value(
        &second.archive().field(&second.result()).unwrap().value,
        &expected,
    );
    assert_history(&second);
    assert_isolated(&second);
}

#[test]
fn every_byte_identical_owned_request_retains_all_revision_bound_identities() {
    let mut cursor = EXPECTED;
    while !cursor.is_empty() {
        let body = read_body(&mut cursor);
        consume_expected_fields(&mut cursor);
        for (index, field) in body.fields().iter().enumerate() {
            let id = body.field_id(index).unwrap();
            let edit = body.edit_field(&id, field.value.clone()).unwrap();
            assert_eq!(edit.result(), id);
            assert_eq!(edit.archive().to_bytes(), body.to_bytes());
            assert_eq!(
                edit.coverage().completion,
                SimulationBeamGroupCompletion::Unchanged
            );
            assert!(edit.history().iter().all(|row| row.parents.len() == 1));
        }
    }
}

#[test]
fn source_buffers_copied_fields_and_returned_bytes_remain_isolated() {
    for index in 0..4 {
        let expected = owned_body(index);
        let original = expected.to_bytes();
        let mut input = original.clone();
        let body = SimulationBeamGroupBody::from_bytes(
            &input,
            expected.version(),
            expected.kind(),
            LIMITS,
        )
        .unwrap();
        input.fill(0);
        let mut returned = body.to_bytes();
        returned.fill(255);
        let mut fields = body.fields().to_vec();
        fields[0].path = "modified".into();
        fields.clear();
        assert_eq!(body.to_bytes(), original);
        assert_eq!(body.fields(), expected.fields());
    }
}

#[test]
fn stale_ids_and_fabricated_construction_roles_fail_without_source_mutation() {
    let body = owned_body(3);
    let original = body.to_bytes();
    let id = body.field_id(0).unwrap();
    let edit = body
        .edit_field(&id, SimulationBeamGroupValue::Signed(123))
        .unwrap();
    assert!(edit.archive().field(&id).is_err());
    assert!(body.field(&id).is_ok());
    let mut fields = body.fields().to_vec();
    fields[0].path = "fabricated.role".into();
    let error = SimulationBeamGroupBody::from_fields(fields, body.version(), body.kind(), LIMITS)
        .unwrap_err();
    assert!(error.cause.contains("fabricated.role") && error.cause.contains("folder.word20"));
    assert_wrong_value_kind(&body);
    assert_eq!(body.to_bytes(), original);
}

fn assert_wrong_value_kind(body: &SimulationBeamGroupBody) {
    let mut fields = body.fields().to_vec();
    fields[0].value = SimulationBeamGroupValue::Unsigned(0x80000000);
    let error = SimulationBeamGroupBody::from_fields(fields, body.version(), body.kind(), LIMITS)
        .unwrap_err();
    assert!(error.cause.contains("Unsigned(2147483648)"));
    assert!(error.cause.contains("Signed(-2147483648)"));
}
