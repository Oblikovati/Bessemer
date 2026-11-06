use super::*;
#[test]
fn every_complete_profile_matches_all_physical_keys_nested_bodies_references_attributes_and_coverage()
 {
    for mut case in profile_fixture::cases() {
        let profile = open(&case.raw, options()).unwrap();
        check_profile(&profile, &case);
        check_bodies(&profile, &mut case);
    }
}
fn check_profile(profile: &FlowMediumKindOneProfile, case: &ProfileCase) {
    check_header(profile, case);
    assert_eq!(profile.first_objects().len(), case.first.len());
    assert_eq!(profile.reference_objects().len(), case.second.len());
    assert!(profile.first_object(case.first.len()).is_none());
    assert!(profile.reference_object(case.second.len()).is_none());
    for (actual, expected) in profile.reference_objects().zip(&case.second) {
        check_reference(actual, expected);
    }
    check_outer(profile, case);
    check_coverage(profile, case);
}
fn check_bodies(profile: &FlowMediumKindOneProfile, case: &mut ProfileCase) {
    for (object, expected) in profile.first_objects().zip(case.first.iter_mut()) {
        assert_eq!(object.ordinal(), expected.ordinal as usize);
        assert_eq!(object.key(), guid(&expected.key));
        assert_eq!(
            object.payload_interval(),
            (expected.start as usize, expected.end as usize)
        );
        assert_eq!(
            profile.first_object(object.ordinal()).unwrap().key(),
            object.key()
        );
        expected.body.footer.tail = case.raw[expected.end as usize..].to_vec();
        expected
            .body
            .raw
            .extend_from_slice(&expected.body.footer.tail);
        crate::api::read::check_body(object.body(), &expected.body);
    }
}
fn check_reference(reference: FlowMediumProfileReference<'_>, expected: &ProfileSecond) {
    assert_eq!(reference.ordinal(), expected.ordinal as usize);
    assert_eq!(reference.key(), guid(&expected.key));
    assert_eq!(
        reference.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(reference.state_byte(), expected.state);
    assert_eq!(reference.native_state_after_load(), expected.loaded);
    assert_eq!(guid_bytes(reference.references()), expected.references);
    let start = expected.start as usize + 27;
    assert_eq!(
        reference.references_interval(),
        (start, start + expected.references.len() * 16)
    );
}
fn check_outer(profile: &FlowMediumKindOneProfile, case: &ProfileCase) {
    assert_eq!(guid_bytes(profile.outer_references()), case.footer.keys);
    assert_eq!(profile.outer_attributes(), case.footer.attributes);
    let start = case.backend_end as usize + 4;
    let attributes = start + case.footer.keys.len() * 16;
    assert_eq!(profile.outer_references_interval(), (start, attributes));
    assert_eq!(
        profile.outer_attributes_interval(),
        (attributes, attributes + case.footer.attributes.len() * 4)
    );
}
fn check_coverage(profile: &FlowMediumKindOneProfile, case: &ProfileCase) {
    let coverage = profile.coverage();
    let counts = totals(case);
    assert_eq!(
        coverage.mode,
        FlowMediumProfileCheckMode::ExactFreshKindOneRecord
    );
    assert_eq!(coverage.payload_bytes, case.raw.len());
    assert_eq!(coverage.metadata_bytes, case.backend_start as usize);
    assert_eq!(
        coverage.backend_bytes,
        (case.backend_end - case.backend_start) as usize
    );
    check_counts(&coverage, counts);
    assert!(coverage.fresh_backend_assumed);
    assert!(coverage.complete_record_framing);
    assert!(!coverage.full_engineering_schema);
    assert_eq!(coverage.achieved_error, 0);
    check_diagnostics(profile);
}

fn check_header(profile: &FlowMediumKindOneProfile, case: &ProfileCase) {
    assert_eq!(profile.payload_bytes(), case.raw);
    assert_eq!(profile.record().ordinal(), 1);
    assert_eq!(profile.source().records().len(), 3);
    assert_eq!(
        profile.options().context,
        FlowMediumProfileContext::FreshBackend
    );
    assert_eq!(
        profile.metadata_prefix().coverage().known_prefix_bytes,
        case.backend_start as usize
    );
    assert_eq!(
        profile.backend_interval(),
        (case.backend_start as usize, case.backend_end as usize)
    );
}

fn check_counts(coverage: &FlowMediumProfileCoverage, counts: [usize; 5]) {
    assert_eq!(
        [
            coverage.first_objects,
            coverage.reference_objects,
            coverage.references,
            coverage.wide_words,
            coverage.name_units
        ],
        counts
    );
}

fn check_diagnostics(profile: &FlowMediumKindOneProfile) {
    assert_eq!(
        profile.diagnostics(),
        [
            FlowMediumProfileDiagnostic::NativeRuntimeNotReproduced,
            FlowMediumProfileDiagnostic::EngineeringAndEvaluationNotChecked,
            FlowMediumProfileDiagnostic::NativeApplicationNotChecked
        ]
    );
}
