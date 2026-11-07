use super::*;
fn mixed_payload(raw: &[u8]) -> Vec<u8> {
    let mut payload = vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    payload.extend_from_slice(&2i32.to_le_bytes());
    payload.extend_from_slice(&2i32.to_le_bytes());
    payload.extend_from_slice(&[0xa5; 16]);
    let mut body = vec![0; 198];
    body[136..140].copy_from_slice(&3i32.to_le_bytes());
    payload.extend_from_slice(&body);
    payload.extend_from_slice(&1i32.to_le_bytes());
    payload.extend_from_slice(&[0x5a; 16]);
    payload.extend_from_slice(raw);
    payload.extend_from_slice(&[0; 17]);
    payload
}
fn mixed_options() -> FlowMediumMixedOptions {
    FlowMediumMixedOptions {
        profile: FlowMediumProfileOptions {
            metadata: crate::api::prefix_options(),
            body: crate::api::options(),
            context: FlowMediumProfileContext::FreshBackend,
            maximum_first_objects: 1024,
            maximum_reference_objects: 1024,
            maximum_references: 100_000,
            maximum_total_wide_words: 1_000_000,
            maximum_total_name_units: 1_000_000,
        },
        maximum_body_word_patterns: 100_000,
        maximum_total_word_patterns: 1_000_000,
    }
}
fn kind_one(profile: &FlowMediumMixedProfile) -> &FlowMediumKindOne {
    match profile.first_objects()[1].body() {
        FlowMediumBody::KindOne(body) => body,
        _ => panic!("Expected selected later kind-one fixture body"),
    }
}
#[test]
fn evaluation_composes_with_a_later_kind_one_body_and_exact_full_mixed_carrier_reversal() {
    let cases = evaluation_fixture::cases();
    let case = zero_case(&cases);
    let source = crate::api::carrier(&mixed_payload(&case.body.raw));
    let id = source.latest(424).unwrap().unwrap().id;
    let profile = source
        .record_424_mixed_profile(&id, mixed_options())
        .unwrap();
    let result = kind_one(&profile)
        .evaluate_curve(FlowMediumCurveAccessor::CurveTwo, 0.)
        .unwrap();
    assert_eq!(result.body_payload_interval().0, 253);
    assert_eq!(result.value(), 0.);
    let curve = &case.body.prefix.curves[2];
    assert_eq!(
        result.curve_payload_interval(),
        (253 + curve.start as usize, 253 + curve.end as usize)
    );
    check_nested_edit(&profile, &result);
}
fn scalar_edit(profile: &FlowMediumMixedProfile, value: f64) -> FlowMediumMixedEdit {
    let edit = kind_one(profile)
        .replace_wide_word(
            FlowMediumWideField::CurveScalar { curve: 2 },
            0,
            value.to_bits(),
        )
        .unwrap();
    profile.apply_kind_one_body_edit(1, &edit).unwrap()
}
fn check_nested_edit(profile: &FlowMediumMixedProfile, original: &FlowMediumCurveEvaluation) {
    let changed = scalar_edit(profile, 9.);
    let result = kind_one(changed.result())
        .evaluate_curve(FlowMediumCurveAccessor::CurveTwo, 0.)
        .unwrap();
    assert_eq!(result.value(), 9.);
    assert_ne!(result.source_record(), original.source_record());
    assert_eq!(original.value(), 0.);
    assert_eq!(original.source_record(), profile.record());
    let reverse = scalar_edit(changed.result(), 0.);
    assert_eq!(
        reverse.result().source().as_bytes(),
        profile.source().as_bytes()
    );
}
