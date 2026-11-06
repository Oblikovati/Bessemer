use super::*;
use bessemer::FailureClass;
use std::sync::Arc;

#[test]
fn every_reference_prefix_and_extension_rejects_without_partial_publication() {
    for case in fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 143)
    {
        for end in 0..case.payload.len() {
            assert!(
                carrier(&case.payload[..end])
                    .feature_identities(options())
                    .is_err()
            );
        }
        let mut extended = case.payload;
        extended.push(0);
        assert!(carrier(&extended).feature_identities(options()).is_err());
    }
}

#[test]
fn negative_or_impossible_counts_and_unterminated_strings_have_typed_errors() {
    let original = fixture::cases().remove(0).payload;
    for (offset, value) in [
        (4, u32::MAX),
        (4, i32::MAX as u32),
        (20, u32::MAX),
        (20, i32::MAX as u32),
        (207, u32::MAX),
        (207, i32::MAX as u32),
    ] {
        let mut malformed = original.clone();
        malformed[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        assert_count_error(&malformed);
    }
    let mut unterminated = original;
    unterminated[113] = 1;
    assert!(
        carrier(&unterminated)
            .feature_identities(options())
            .is_err()
    );
}

#[test]
fn every_allocation_axis_is_explicit_and_checked_before_results() {
    let native = fixture::cases().remove(0);
    for configured in invalid_options() {
        reject_read(&native.payload, configured);
    }
    let mut object_bound = options();
    object_bound.maximum_objects = 2;
    reject_read(&native.payload, object_bound);
    let mut text_bound = options();
    text_bound.maximum_text_units = 73;
    reject_read(&native.payload, text_bound);
    let tail_case = populated_tail_case();
    let mut tail_bound = options();
    tail_bound.maximum_tail_values = 3;
    reject_read(&tail_case.payload, tail_bound);
}

fn invalid_options() -> [FlowFeatureOptions; 3] {
    let mut object = options();
    object.maximum_objects = 0;
    let mut text = options();
    text.maximum_text_units = 0;
    let mut tail = options();
    tail.maximum_tail_values = 0;
    [object, text, tail]
}

#[test]
fn edits_reject_indices_legacy_tail_and_invalid_or_overbudget_text_without_mutation() {
    let parsed = list(&fixture::cases()[0].payload);
    for index in [parsed.objects().len(), usize::MAX] {
        reject_edit(&parsed, index, FlowFeatureFieldEdit::State(7));
    }
    assert!(parsed.replace_tail_value(0, 1).is_err());
    reject_edit(&parsed, 0, FlowFeatureFieldEdit::Text(Arc::from([65])));
    reject_edit(
        &parsed,
        0,
        FlowFeatureFieldEdit::Text(Arc::from(vec![0; 2001])),
    );
    let legacy = fixture::cases()
        .into_iter()
        .find(|case| case.identifier == 143 && case.version == 0)
        .unwrap();
    assert!(list(&legacy.payload).replace_tail_value(0, 1).is_err());
    assert_eq!(parsed.payload(), fixture::cases()[0].payload);
    check_growth_limits(&parsed);
}

fn check_growth_limits(parsed: &FlowFeatureIdentities) {
    let mut wire_bound = limits();
    wire_bound.maximum_wire_bytes = parsed.source().as_bytes().len();
    let strict = strict_list(parsed, wire_bound);
    reject_edit(&strict, 0, FlowFeatureFieldEdit::Text(Arc::from([65, 0])));
    let mut payload_bound = limits();
    payload_bound.maximum_payload_bytes = parsed.payload().len();
    let strict = strict_list(parsed, payload_bound);
    reject_edit(&strict, 0, FlowFeatureFieldEdit::Text(Arc::from([65, 0])));
    assert_eq!(strict.source().as_bytes(), parsed.source().as_bytes());
}

#[test]
fn last_malformed_duplicate_rejects_and_missing_record_is_completed_absence() {
    let native = fixture::cases().remove(0);
    let mut bytes = wire(&native.payload);
    append_record(&mut bytes, 143, &[]);
    let duplicate = FlowCarrier::from_bytes(&bytes, limits()).unwrap();
    assert!(duplicate.feature_identities(options()).is_err());
    assert_eq!(duplicate.as_bytes(), bytes);
    let missing = FlowCarrier::from_bytes(&8_u16.to_le_bytes(), limits()).unwrap();
    assert!(missing.feature_identities(options()).unwrap().is_none());
    assert!(missing.feature_identities(invalid_options()[0]).is_err());
}

#[test]
fn list_version_is_distinct_from_independent_object_header_and_body_words() {
    for version in 0..=u32::from(u16::MAX) {
        let mut payload = version.to_le_bytes().to_vec();
        payload.extend(0_i32.to_le_bytes());
        if version >= 4 {
            payload.extend(0_i32.to_le_bytes());
        }
        assert_eq!(
            carrier(&payload).feature_identities(options()).is_ok(),
            version <= 6
        );
    }
    let mut payload = fixture::cases()[0].payload.clone();
    payload[..4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(
        carrier(&payload)
            .feature_identities(options())
            .unwrap_err()
            .class,
        FailureClass::Unsupported
    );
}

fn assert_count_error(malformed: &[u8]) {
    let error = carrier(malformed)
        .feature_identities(options())
        .unwrap_err();
    assert_eq!(error.operation, "Flow feature identities");
    assert!(error.entity.contains("record 143"));
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("expected"));
}

fn reject_read(payload: &[u8], configured: FlowFeatureOptions) {
    assert!(carrier(payload).feature_identities(configured).is_err());
}

fn populated_tail_case() -> fixture::FeatureCase {
    fixture::cases()
        .into_iter()
        .find(|case| case.tail.as_ref().is_some_and(|values| values.len() == 4))
        .unwrap()
}

fn reject_edit(parsed: &FlowFeatureIdentities, index: usize, edit: FlowFeatureFieldEdit) {
    assert!(parsed.replace_object_field(index, edit).is_err());
}

fn strict_list(parsed: &FlowFeatureIdentities, bounds: FlowLimits) -> FlowFeatureIdentities {
    FlowCarrier::from_bytes(parsed.source().as_bytes(), bounds)
        .unwrap()
        .feature_identities(options())
        .unwrap()
        .unwrap()
}
