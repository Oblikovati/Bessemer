use super::*;

pub(super) fn modern_property() -> SimulationBeamProperty {
    let mut packet = OWNED;
    while !packet.is_empty() {
        let property = read(&case(&mut packet, false));
        if property.version().value() == 890022
            && property
                .fields()
                .iter()
                .any(|field| field.path.as_ref() == "record0.reference0.archive0")
        {
            return property;
        }
    }
    panic!("expected complete modern owned reference profile");
}

fn shared(
    path: &'static str,
    value: SimulationThermalValue,
) -> (&'static str, SimulationBeamValue) {
    (path, SimulationBeamValue::Shared(value))
}

fn numeric_replacements() -> Vec<(&'static str, SimulationBeamValue)> {
    use SimulationThermalValue as Value;
    vec![
        (
            "record0.beam.shorte2",
            SimulationBeamValue::SignedShort(123),
        ),
        shared("record0.beam.four_words[0]", Value::Signed(12345)),
        shared("record0.beam.embedded.word10", Value::Unsigned(0x12345678)),
        shared(
            "record0.beam.section.four_doubles[0]",
            Value::DoubleBits(0x7ff800000000abcd),
        ),
    ]
}

pub(super) fn replacements() -> Vec<(&'static str, SimulationBeamValue)> {
    use SimulationThermalValue as Value;
    let mut values = numeric_replacements();
    let text = SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 32).unwrap();
    let embedded = load_strings::framed_string_archive(false, LIMITS.embedded);
    values.extend([
        shared("record0.beam.textd0", Value::Text(text)),
        shared(
            "record0.reference0.blob",
            Value::Bytes(vec![1, 2, 3].into()),
        ),
        shared(
            "record0.reference0.archive0",
            Value::Embedded(Box::new(embedded)),
        ),
    ]);
    values
}

pub(super) fn replace(
    property: &SimulationBeamProperty,
    path: &str,
    value: SimulationBeamValue,
) -> SimulationBeamEdit {
    property
        .edit_field(&property.field_id(ordinal(property, path)).unwrap(), value)
        .unwrap()
}

pub(super) fn assert_relations(edit: &SimulationBeamEdit, index: usize) {
    assert_eq!(edit.history().len(), edit.source().fields().len());
    for (position, row) in edit.history().iter().enumerate() {
        let mut parents = vec![SimulationBeamHistoryParent::Source(
            edit.source().field_id(position).unwrap(),
        )];
        if position == index {
            parents.push(SimulationBeamHistoryParent::Replacement);
        }
        assert_eq!(row.parents.as_ref(), parents);
        assert_eq!(row.result, edit.archive().field_id(position).unwrap());
        assert_eq!(
            row.relation,
            if position == index {
                SimulationBeamRelation::Derived
            } else {
                SimulationBeamRelation::Copied
            }
        );
    }
}

pub(super) fn assert_isolated(edit: &SimulationBeamEdit, index: usize) {
    let before = edit.source().to_bytes();
    let after = edit.archive().to_bytes();
    let (start, end) = edit.source().field_offsets()[index];
    let (next_start, next_end) = edit.archive().field_offsets()[index];
    assert_eq!(start, next_start);
    assert_eq!(before[..start], after[..next_start]);
    assert_eq!(before[end..], after[next_end..]);
    for (position, field) in edit.source().fields().iter().enumerate() {
        if position != index {
            assert_eq!(field, &edit.archive().fields()[position]);
        }
    }
}

#[test]
fn all_seven_value_kinds_retain_complete_inputs_results_and_exact_isolation() {
    let source = modern_property();
    let original = source.to_bytes();
    for (path, value) in replacements() {
        let index = ordinal(&source, path);
        let edit = replace(&source, path, value.clone());
        assert_eq!(edit.replacement(), &value);
        assert_eq!(edit.source().to_bytes(), original);
        assert_eq!(edit.archive().fields()[index].value, value);
        assert_eq!(
            edit.coverage().completion,
            SimulationBeamCompletion::Changed
        );
        assert_eq!(edit.coverage().requested_pattern_error_bits, 0);
        assert_relations(&edit, index);
        assert_isolated(&edit, index);
    }
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn embedded_cstring_prefix_widths_remain_distinct_real_edits() {
    let source = modern_property();
    let path = "record0.reference0.archive0";
    let long = load_strings::framed_string_archive(true, LIMITS.embedded);
    let first = replace_embedded(&source, false);
    let second = replace_embedded(first.archive(), true);
    assert_eq!(
        second.archive().to_bytes().len(),
        first.archive().to_bytes().len() + 2
    );
    let SimulationBeamValue::Shared(value) =
        &second.archive().fields()[ordinal(&source, path)].value
    else {
        panic!("expected shared embedded value");
    };
    load_strings::assert_wire_value(value, &long);
    assert_relations(&second, ordinal(&source, path));
    assert_isolated(&second, ordinal(&source, path));
}

#[test]
fn owned_inputs_returned_buffers_and_invalid_structural_edits_cannot_mutate_source() {
    let expected = modern_property();
    let original = expected.to_bytes();
    let mut input = original.clone();
    let property = SimulationBeamProperty::from_bytes(&input, expected.version(), LIMITS).unwrap();
    input.fill(0);
    let mut returned = property.to_bytes();
    returned.fill(255);
    let mut fields = property.fields().to_vec();
    fields.clear();
    reject_invalid_count(&property);
    assert_eq!(property.to_bytes(), original);
    assert_eq!(property.fields(), expected.fields());
}

fn replace_embedded(source: &SimulationBeamProperty, extended: bool) -> SimulationBeamEdit {
    let archive = load_strings::framed_string_archive(extended, LIMITS.embedded);
    replace(
        source,
        "record0.reference0.archive0",
        SimulationBeamValue::Shared(SimulationThermalValue::Embedded(Box::new(archive))),
    )
}

fn reject_invalid_count(property: &SimulationBeamProperty) {
    let index = ordinal(property, "record0.beam.point_count");
    let id = property.field_id(index).unwrap();
    assert!(
        property
            .edit_field(
                &id,
                SimulationBeamValue::Shared(SimulationThermalValue::Signed(-1))
            )
            .is_err()
    );
}

#[test]
fn constructor_mismatches_report_the_offending_role_and_expected_shape() {
    let property = modern_property();
    let mut fields = property.fields().to_vec();
    fields[0].path = "fabricated.count".into();
    let error =
        SimulationBeamProperty::from_fields(fields, property.version(), LIMITS).unwrap_err();
    assert!(error.cause.contains("fabricated.count"));
    assert!(error.cause.contains("record0.references.count"));
    let mut fields = property.fields().to_vec();
    let index = ordinal(&property, "record0.beam.shorte2");
    fields[index].value =
        SimulationBeamValue::Shared(SimulationThermalValue::Bytes(vec![0, 128].into()));
    let error =
        SimulationBeamProperty::from_fields(fields, property.version(), LIMITS).unwrap_err();
    assert!(error.cause.contains("Shared(Bytes"));
    assert!(error.cause.contains("SignedShort(-32768)"));
}
