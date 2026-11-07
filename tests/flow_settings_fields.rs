//! Seven-field oracle including explicit missing guard and floating-point prerequisites.
use bessemer::flow::*;
use std::io::{Cursor, Read};

struct ExpectedFields {
    guard: Option<i32>,
    fields: Vec<(usize, u64, FlowSettingsFieldValue)>,
}

fn limits() -> FlowLimits {
    FlowLimits {
        maximum_wire_bytes: 20_000,
        maximum_records: 10,
        maximum_payload_bytes: 5000,
    }
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut bytes = [0; N];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}

fn packet() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("fixtures/flow-settings-fields/fields.bin.gz")[..],
    )
    .take(1_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    raw
}

fn expected_fields(cursor: &mut Cursor<&[u8]>) -> Option<ExpectedFields> {
    if number::<1>(cursor)[0] == 0 {
        return None;
    }
    let present = number::<1>(cursor)[0] != 0;
    let guard = i32::from_le_bytes(number(cursor));
    let fields = (0..7).map(|_| expected_field(cursor)).collect();
    Some(ExpectedFields {
        guard: present.then_some(guard),
        fields,
    })
}

fn expected_field(cursor: &mut Cursor<&[u8]>) -> (usize, u64, FlowSettingsFieldValue) {
    let index = u32::from_le_bytes(number(cursor)) as usize;
    let bits = u64::from_le_bytes(number(cursor));
    let status = number::<1>(cursor)[0];
    let value = i32::from_le_bytes(number(cursor));
    let expected = match status {
        0 => FlowSettingsFieldValue::Derived(value),
        1 => FlowSettingsFieldValue::RequiresIntegerSlot88,
        2 => FlowSettingsFieldValue::RequiresFloatingPointContext,
        other => panic!("fixture status {other}; expected 0..=2"),
    };
    (index, bits, expected)
}

#[test]
fn all_141_source_cases_preserve_bits_labels_and_explicit_prerequisites() {
    let raw = packet();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FSF1");
    assert_eq!(u32::from_le_bytes(number(&mut cursor)), 141);
    for _ in 0..141 {
        let length = u32::from_le_bytes(number(&mut cursor)) as usize;
        let mut wire = vec![0; length];
        cursor.read_exact(&mut wire).unwrap();
        let expected = expected_fields(&mut cursor);
        check_case(&wire, expected);
    }
    assert_eq!(cursor.position(), raw.len() as u64);
}

fn check_case(wire: &[u8], expected: Option<ExpectedFields>) {
    let carrier = FlowCarrier::from_bytes(wire, limits()).unwrap();
    let projected = carrier.settings_fields().unwrap();
    let Some(expected) = expected else {
        assert!(projected.is_none());
        return;
    };
    let projected = projected.unwrap();
    assert_eq!(projected.numeric().source().as_bytes(), wire);
    assert_eq!(projected.integer_slot_88(), expected.guard);
    assert_eq!(projected.integer().is_some(), expected.guard.is_some());
    assert_eq!(projected.diagnostics().len(), 3);
    check_labels(&projected, &expected);
    check_coverage(&projected, &expected);
    assert_eq!(carrier.as_bytes(), wire);
}

fn check_labels(projected: &FlowSettingsFields, expected: &ExpectedFields) {
    let labels = ["EXTERNAL", "LAMTURB", "CHT", "US_DIM", "IT", "NT", "NZ"];
    for ((actual, (index, bits, value)), label) in
        projected.fields().iter().zip(&expected.fields).zip(labels)
    {
        assert_eq!(actual.field().label(), label);
        assert_eq!(actual.field().slot_index(), *index);
        assert_eq!(actual.stored_bits(), *bits);
        assert_eq!(actual.value(), *value);
    }
}

fn check_coverage(projected: &FlowSettingsFields, expected: &ExpectedFields) {
    let coverage = projected.coverage();
    assert_eq!(
        coverage.mode,
        FlowSettingsFieldsCheckMode::ExactStoredFieldProjection
    );
    let derived = expected
        .fields
        .iter()
        .filter(|entry| matches!(entry.2, FlowSettingsFieldValue::Derived(_)))
        .count();
    let guard = expected
        .fields
        .iter()
        .filter(|entry| entry.2 == FlowSettingsFieldValue::RequiresIntegerSlot88)
        .count();
    assert_eq!(coverage.derived_fields, derived);
    assert_eq!(coverage.missing_guard_fields, guard);
    assert_eq!(coverage.floating_point_context_fields, 7 - derived - guard);
    assert_eq!(coverage.achieved_byte_error, 0);
}

fn array_carrier(include_integer: bool) -> FlowCarrier {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    append_record(&mut wire, 102, &[0; 2560]);
    if include_integer {
        append_record(&mut wire, 7, &[0; 400]);
    }
    FlowCarrier::from_bytes(&wire, limits()).unwrap()
}

fn append_record(wire: &mut Vec<u8>, identifier: u32, payload: &[u8]) {
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
}

#[test]
fn selected_malformed_records_reject_without_falling_back_or_assuming_guard() {
    for identifier in [102, 7] {
        let source = array_carrier(true);
        let mut wire = source.as_bytes().to_vec();
        append_record(&mut wire, identifier, &[]);
        let duplicate = FlowCarrier::from_bytes(&wire, limits()).unwrap();
        assert!(duplicate.settings_fields().is_err());
        assert_eq!(duplicate.as_bytes(), wire);
    }
    let mut wire = 7_u16.to_le_bytes().to_vec();
    append_record(&mut wire, 7, &[]);
    let absent = FlowCarrier::from_bytes(&wire, limits()).unwrap();
    assert!(absent.settings_fields().unwrap().is_none());
}

#[test]
fn numeric_edits_compose_without_normalizing_or_mutating_the_original_projection() {
    let source = array_carrier(true);
    let original = source.settings_fields().unwrap().unwrap();
    let edit = original
        .numeric()
        .replace_slot(117, FlowFixedSlot::Numeric(0.25_f64.to_bits()))
        .unwrap();
    let changed = edit.result().settings_fields().unwrap().unwrap();
    assert_eq!(
        changed.fields()[2].value(),
        FlowSettingsFieldValue::Derived(1)
    );
    assert_eq!(
        changed.numeric().slot(117),
        Some(FlowFixedSlot::Numeric(0.25_f64.to_bits()))
    );
    assert_eq!(
        original.fields()[2].value(),
        FlowSettingsFieldValue::Derived(0)
    );
    check_original_arrays(&original, &source);
}

#[test]
fn published_projections_are_send_sync_and_eight_workers_match_serial_fields() {
    assert_traits::<FlowSettingsFields>();
    assert_traits::<FlowSettingsField>();
    assert_traits::<FlowSettingsFieldValue>();
    assert_traits::<FlowSettingsFieldProjection>();
    assert_traits::<FlowSettingsFieldsCoverage>();
    assert_traits::<FlowSettingsFieldsCheckMode>();
    let source = array_carrier(false);
    let serial = source.settings_fields().unwrap().unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let shared = &source;
            let expected = &serial;
            scope.spawn(move || {
                assert_eq!(
                    shared.settings_fields().unwrap().unwrap().fields(),
                    expected.fields()
                )
            });
        }
    });
}

fn assert_traits<T: Send + Sync>() {}

fn check_original_arrays(original: &FlowSettingsFields, source: &FlowCarrier) {
    assert_eq!(original.numeric().source().as_bytes(), source.as_bytes());
    assert_eq!(original.numeric().as_bytes(), &[0; 2560]);
    assert_eq!(original.integer().unwrap().as_bytes(), &[0; 400]);
}
