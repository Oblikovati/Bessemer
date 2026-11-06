use super::*;
#[test]
fn every_complete_mixed_profile_matches_all_nested_fields_typed_maps_intervals_and_coverage() {
    for mut case in mixed_fixture::cases() {
        let profile = open(&case.raw, options()).unwrap();
        check_profile(&profile, &case);
        check_bodies(&profile, &mut case);
    }
}
fn check_profile(profile: &FlowMediumMixedProfile, case: &MixedCase) {
    assert_eq!(profile.payload_bytes(), case.raw);
    assert_eq!(
        profile.metadata_prefix().opaque_tail(),
        &case.raw[case.backend_start as usize..]
    );
    assert_eq!(
        profile.backend_interval(),
        (case.backend_start as usize, case.backend_end as usize)
    );
    assert_eq!(profile.first_objects().len(), case.first.len());
    assert_eq!(profile.reference_objects().len(), case.second.len());
    check_outer(profile, case);
    check_references(profile, case);
    check_coverage(profile, case);
}
fn check_outer(profile: &FlowMediumMixedProfile, case: &MixedCase) {
    let keys: Vec<_> = case.footer.keys.iter().map(|key| guid(key)).collect();
    assert_eq!(profile.outer_references(), keys);
    assert_eq!(profile.outer_attributes(), case.footer.attributes);
    let start = case.backend_end as usize + 4;
    let end = start + 16 * keys.len();
    assert_eq!(profile.outer_references_interval(), (start, end));
    assert_eq!(
        profile.outer_attributes_interval(),
        (end, end + 4 * case.footer.attributes.len())
    );
    assert_eq!(
        profile.options().maximum_body_word_patterns,
        options().maximum_body_word_patterns
    );
    assert_eq!(
        profile.source().latest(424).unwrap().unwrap().id,
        *profile.record()
    );
}

fn check_bodies(profile: &FlowMediumMixedProfile, case: &mut MixedCase) {
    for (actual, expected) in profile.first_objects().iter().zip(&mut case.first) {
        assert_eq!(
            actual.payload_interval(),
            (expected.start as usize, expected.end as usize)
        );
        assert_eq!(actual.key(), guid(&expected.key));
        assert_eq!(actual.body().kind().as_i32(), expected.kind);
        assert_eq!(
            actual.body().payload_interval(),
            (expected.start as usize + 20, expected.end as usize)
        );
        let tail = case.raw[expected.end as usize..].to_vec();
        check_nested_body(
            actual.body(),
            &mut expected.body,
            tail,
            expected.start as usize + 20,
        );
    }
}
fn check_nested_body(
    actual: &FlowMediumBody,
    expected: &mut MixedBody,
    tail: Vec<u8>,
    start: usize,
) {
    match (actual, expected) {
        (FlowMediumBody::KindOne(body), MixedBody::One(value)) => {
            value.footer.tail = tail;
            crate::api::read::check_body(body, value);
        }
        (FlowMediumBody::KindTwo(body), MixedBody::Two(value)) => {
            value.footer.tail = tail;
            super::super::super::api::read::check_body_at(body, value, start);
        }
        _ => panic!("Expected matching typed nested body"),
    }
}
fn check_references(profile: &FlowMediumMixedProfile, case: &MixedCase) {
    for (actual, expected) in profile.reference_objects().iter().zip(&case.second) {
        check_reference(actual, expected);
    }
}
fn check_reference(actual: &FlowMediumMixedReference, expected: &MixedSecond) {
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(actual.kind().as_i32(), expected.kind);
    assert_eq!(actual.key(), guid(&expected.key));
    assert_eq!(actual.state_byte(), expected.state);
    assert_eq!(actual.native_state_after_load(), expected.loaded);
    let keys: Vec<_> = expected.references.iter().map(|key| guid(key)).collect();
    assert_eq!(actual.references(), keys);
    let start = expected.start as usize + if expected.kind == 1 { 27 } else { 26 };
    assert_eq!(
        actual.references_interval(),
        (start, start + 16 * keys.len())
    );
}
fn check_coverage(profile: &FlowMediumMixedProfile, case: &MixedCase) {
    let coverage = profile.coverage();
    let (wide, words, names, refs) = totals(case);
    assert_eq!(coverage.payload_bytes, case.raw.len());
    assert_eq!(coverage.metadata_bytes, case.backend_start as usize);
    assert_eq!(
        coverage.backend_bytes,
        (case.backend_end - case.backend_start) as usize
    );
    assert_eq!(coverage.first_objects, case.first.len());
    assert_eq!(coverage.reference_objects, case.second.len());
    assert_eq!(coverage.wide_words, wide);
    assert_eq!(coverage.word_patterns, words);
    assert_eq!(coverage.name_units, names);
    assert_eq!(coverage.references, refs);
    check_exclusions(profile);
}
fn check_exclusions(profile: &FlowMediumMixedProfile) {
    let coverage = profile.coverage();
    assert_eq!(
        coverage.mode,
        FlowMediumMixedCheckMode::ExactFreshMixedRecord
    );
    assert!(coverage.complete_record_framing && coverage.fresh_backend_assumed);
    assert!(!coverage.full_engineering_schema && !coverage.installed_kind_two_fixture_validated);
    assert_eq!(coverage.achieved_error, 0);
    assert_eq!(
        profile.diagnostics(),
        [
            FlowMediumMixedDiagnostic::InstalledKindTwoFixtureNotValidated,
            FlowMediumMixedDiagnostic::EngineeringAndEvaluationNotChecked,
            FlowMediumMixedDiagnostic::NativeRuntimeNotReproduced,
            FlowMediumMixedDiagnostic::NativeApplicationNotChecked
        ]
    );
}
