//! Synthetic populated bodies, explicit legacy context and complete isolated wire edits.
#[path = "flow_settings_body/fixture.rs"]
mod fixture;
#[path = "flow_carriers/fixture.rs"]
mod native_fixture;
use bessemer::flow::*;

fn limits() -> FlowLimits {
    FlowLimits {
        maximum_wire_bytes: 10_000,
        maximum_records: 100,
        maximum_payload_bytes: 5000,
    }
}

fn append_record(wire: &mut Vec<u8>, identifier: u32, payload: &[u8]) {
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
}

fn carrier(payload: &[u8]) -> FlowCarrier {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    append_record(&mut wire, 999, b"before");
    append_record(&mut wire, 511, payload);
    append_record(&mut wire, 111, b"after");
    FlowCarrier::from_bytes(&wire, limits()).unwrap()
}

#[test]
fn all_twelve_synthetic_versions_and_72_contexts_match_independent_fields_and_legacy_states() {
    for case in fixture::cases() {
        let source = carrier(&case.payload);
        for (context, expected) in &case.contexts {
            let body = source.settings_body(*context).unwrap();
            assert_eq!(body.source().as_bytes(), source.as_bytes());
            assert_eq!(
                body.record(),
                Some(&source.latest(511).unwrap().unwrap().id)
            );
            assert_eq!(body.version().unwrap().wire_value(), case.version);
            assert_eq!(body.payload(), Some(case.payload.as_slice()));
            assert_eq!(body.runtime_integer_slot_88(), *context);
            assert_eq!(body.legacy_member_0f(), *expected);
            check_fields(&body, &case);
            check_coverage(&body, &case);
        }
    }
}

fn check_fields(body: &FlowSettingsBody, case: &fixture::BodyCase) {
    assert_eq!(body.fields().len(), case.fields.len());
    for (actual, expected) in body.fields().iter().zip(&case.fields) {
        assert_eq!(actual.member(), expected.member);
        assert_eq!(actual.payload_offset(), expected.offset);
        assert_eq!(actual.value(), expected.value);
        assert_eq!(body.field(expected.member), Some(actual));
    }
}

fn check_coverage(body: &FlowSettingsBody, case: &fixture::BodyCase) {
    let coverage = body.coverage();
    assert_eq!(
        coverage.mode,
        FlowSettingsBodyCheckMode::ExactOptionalVersionedBody
    );
    assert_eq!(coverage.checked_bytes, case.payload.len());
    assert_eq!(coverage.checked_members, case.fields.len());
    assert_eq!(coverage.opaque_member_bytes, case.payload.len() - 8);
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(body.diagnostics().len(), 3);
}

#[test]
fn every_present_member_supports_exact_noop_changed_wire_and_fresh_reversal() {
    let mut checked = 0;
    for case in fixture::cases() {
        let body = carrier(&case.payload).settings_body(None).unwrap();
        for expected in &case.fields {
            check_edit(&body, expected);
            checked += 1;
        }
    }
    assert_eq!(checked, 145);
}

fn changed_value(value: FlowSettingsBodyValue) -> FlowSettingsBodyValue {
    match value {
        FlowSettingsBodyValue::Byte(value) => FlowSettingsBodyValue::Byte(value ^ 0xff),
        FlowSettingsBodyValue::Word(value) => FlowSettingsBodyValue::Word(value ^ 0x80000001),
        FlowSettingsBodyValue::Raw4(bytes) => {
            FlowSettingsBodyValue::Raw4(bytes.map(|byte| byte ^ 0xa5))
        }
        FlowSettingsBodyValue::Raw8(bytes) => {
            FlowSettingsBodyValue::Raw8(bytes.map(|byte| byte ^ 0x5a))
        }
    }
}

fn value_bytes(value: FlowSettingsBodyValue) -> Vec<u8> {
    match value {
        FlowSettingsBodyValue::Byte(byte) => vec![byte],
        FlowSettingsBodyValue::Word(word) => word.to_le_bytes().to_vec(),
        FlowSettingsBodyValue::Raw4(raw) => raw.to_vec(),
        FlowSettingsBodyValue::Raw8(raw) => raw.to_vec(),
    }
}

fn check_edit(body: &FlowSettingsBody, field: &fixture::ExpectedMember) {
    let noop = body.replace_member(field.member, field.value).unwrap();
    assert_eq!(noop.result().as_bytes(), body.source().as_bytes());
    let changed = changed_value(field.value);
    let edit = body.replace_member(field.member, changed).unwrap();
    check_edit_wire(body, field, changed, &edit);
    let reopened = edit.result().settings_body(None).unwrap();
    assert_eq!(reopened.field(field.member).unwrap().value(), changed);
    let reversed = reopened.replace_member(field.member, field.value).unwrap();
    assert_eq!(reversed.result().as_bytes(), body.source().as_bytes());
    assert_eq!(body.field(field.member).unwrap().value(), field.value);
    assert!(
        body.replace_member(field.member, wrong_shape(field.value))
            .is_err()
    );
}

fn check_edit_wire(
    body: &FlowSettingsBody,
    field: &fixture::ExpectedMember,
    changed: FlowSettingsBodyValue,
    edit: &FlowCarrierEdit,
) {
    let mut expected = body.source().as_bytes().to_vec();
    let record = body.source().latest(511).unwrap().unwrap();
    let start = record.header_offset + 12 + field.offset;
    let bytes = value_bytes(changed);
    expected[start..start + bytes.len()].copy_from_slice(&bytes);
    assert_eq!(edit.result().as_bytes(), expected);
    assert_eq!(edit.source().as_bytes(), body.source().as_bytes());
    assert_eq!(edit.history().len(), 3);
    check_history(edit);
}

fn check_history(edit: &FlowCarrierEdit) {
    for ((history, source), result) in edit
        .history()
        .iter()
        .zip(edit.source().records())
        .zip(edit.result().records())
    {
        assert_eq!(history.source, source.id);
        assert_eq!(history.result, result.id);
        let expected = if source.identifier == 511 {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        };
        assert_eq!(history.relation, expected);
    }
}

fn wrong_shape(value: FlowSettingsBodyValue) -> FlowSettingsBodyValue {
    match value {
        FlowSettingsBodyValue::Word(_) => FlowSettingsBodyValue::Raw4([0; 4]),
        _ => FlowSettingsBodyValue::Word(0),
    }
}

#[test]
fn every_truncated_prefix_extension_and_unknown_version_rejects() {
    for case in fixture::cases() {
        for length in 0..case.payload.len() {
            assert!(
                carrier(&case.payload[..length])
                    .settings_body(None)
                    .is_err()
            );
        }
        let mut extended = case.payload;
        extended.push(0);
        assert!(carrier(&extended).settings_body(None).is_err());
    }
    check_version_boundaries();
}

#[test]
fn absent_and_omitted_members_require_explicit_runtime_context_without_record_7_inference() {
    let mut wire = 7_u16.to_le_bytes().to_vec();
    append_record(&mut wire, 7, &[0; 400]);
    let source = FlowCarrier::from_bytes(&wire, limits()).unwrap();
    check_missing_snapshot(&source);
    check_absent_context(&source);
    let old = carrier(&fixture::cases()[0].payload)
        .settings_body(Some(0))
        .unwrap();
    assert!(old.field(FlowSettingsBodyMember::Member0f).is_none());
    assert!(
        old.replace_member(
            FlowSettingsBodyMember::Member0f,
            FlowSettingsBodyValue::Byte(1)
        )
        .is_err()
    );
}

fn check_absent_context(source: &FlowCarrier) {
    for context in [0, 1, -1, i32::MIN, i32::MAX] {
        let body = source.settings_body(Some(context)).unwrap();
        assert_eq!(
            body.legacy_member_0f(),
            FlowSettingsLegacyMember::DerivedRuntime {
                context,
                value: u8::from(context == 0)
            }
        );
        assert!(body.fields().is_empty());
    }
}

#[test]
fn malformed_last_duplicate_never_falls_back_and_retained_views_survive_parallel_edits() {
    let case = fixture::cases().remove(11);
    let source = carrier(&case.payload);
    let original = source.settings_body(None).unwrap();
    let mut wire = source.as_bytes().to_vec();
    append_record(&mut wire, 511, &[]);
    assert!(
        FlowCarrier::from_bytes(&wire, limits())
            .unwrap()
            .settings_body(None)
            .is_err()
    );
    check_parallel(&original, &case.fields[0]);
    assert_eq!(original.payload(), Some(case.payload.as_slice()));
}

fn check_parallel(body: &FlowSettingsBody, field: &fixture::ExpectedMember) {
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(move || check_edit(body, field));
        }
    });
}

#[test]
fn all_published_body_contracts_are_send_sync() {
    assert_traits::<FlowSettingsBody>();
    assert_traits::<FlowSettingsBodyVersion>();
    assert_traits::<FlowSettingsBodyMember>();
    assert_traits::<FlowSettingsBodyValue>();
    assert_traits::<FlowSettingsBodyField>();
    assert_traits::<FlowSettingsLegacyMember>();
    assert_traits::<FlowSettingsBodyCheckMode>();
    assert_traits::<FlowSettingsBodyCoverage>();
    assert_traits::<FlowSettingsBodyDiagnostic>();
}

fn assert_traits<T: Send + Sync>() {}

#[test]
fn all_three_owned_native_carriers_confirm_absence_without_inferred_runtime_state() {
    for case in native_fixture::cases().into_iter().take(3) {
        assert!(case.records.iter().all(|record| record.0 != 511));
        let source = FlowCarrier::from_bytes(&case.raw, native_fixture::limits()).unwrap();
        let body = source.settings_body(None).unwrap();
        assert!(body.record().is_none(), "{}", case.name);
        assert_eq!(body.source().as_bytes(), case.raw);
        assert_eq!(
            body.legacy_member_0f(),
            FlowSettingsLegacyMember::RequiresRuntimeIntegerSlot88
        );
        assert_eq!(body.coverage().checked_bytes, 0);
        assert!(body.fields().is_empty());
    }
}

fn check_version_boundaries() {
    for (outer, inner) in [(1_u32, 0_u32), (u32::MAX, 0), (0, 12), (0, u32::MAX)] {
        let raw = [outer.to_le_bytes(), inner.to_le_bytes()].concat();
        assert!(carrier(&raw).settings_body(None).is_err());
    }
    for value in 0..=u32::from(u16::MAX) {
        assert_eq!(
            FlowSettingsBodyVersion::try_from(value).is_ok(),
            value <= 11
        );
    }
}

fn check_missing_snapshot(source: &FlowCarrier) {
    let missing = source.settings_body(None).unwrap();
    assert!(missing.record().is_none());
    assert!(missing.payload().is_none());
    assert!(missing.version().is_none());
    assert_eq!(missing.coverage().checked_bytes, 0);
    assert_eq!(
        missing.legacy_member_0f(),
        FlowSettingsLegacyMember::RequiresRuntimeIntegerSlot88
    );
    assert!(
        missing
            .replace_member(
                FlowSettingsBodyMember::Member0f,
                FlowSettingsBodyValue::Byte(1)
            )
            .is_err()
    );
}
