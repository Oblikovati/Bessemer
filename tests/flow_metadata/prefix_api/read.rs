use super::*;
#[test]
fn every_frozen_prefix_field_interval_native_projection_and_tail_matches_the_typed_reader() {
    for case in fixture::prefix_cases() {
        check_case(&case);
    }
}
fn check_case(case: &PrefixCase) {
    let view = open(&case.raw, options()).unwrap();
    assert_eq!(view.payload_bytes(), case.raw);
    assert_eq!(view.version_byte(), case.version);
    assert_eq!(view.signed_version(), case.signed_version);
    assert_eq!(view.body_active(), case.active);
    assert_eq!(view.opaque_tail(), case.tail);
    assert_eq!(
        view.options().string_mode,
        FlowMetadataStringMode::NativeUtf16
    );
    assert_eq!(view.record().ordinal(), 1);
    assert_eq!(view.source().records().len(), 3);
    check_coverage(&view, case);
    check_vectors(&view, case);
    assert_eq!(view.objects().len(), case.objects.len());
    for (actual, expected) in view.objects().zip(&case.objects) {
        check_object(actual, expected);
    }
    assert!(view.object(case.objects.len()).is_none());
}
fn check_coverage(view: &FlowMetadataPrefix, case: &PrefixCase) {
    let coverage = view.coverage();
    assert_eq!(coverage.payload_bytes, case.raw.len());
    assert_eq!(coverage.known_prefix_bytes, case.known as usize);
    assert_eq!(coverage.unknown_bytes, case.unknown as usize);
    assert!(!coverage.full_payload_schema);
    assert_eq!(coverage.achieved_error, 0);
    let stop = if case.active {
        FlowMetadataPrefixStop::RequiresBackend
    } else {
        FlowMetadataPrefixStop::InactiveVersionBranch
    };
    assert_eq!(view.stop(), stop);
    assert_eq!(
        view.diagnostics(),
        [
            FlowMetadataPrefixDiagnostic::BackendNotDecoded,
            FlowMetadataPrefixDiagnostic::NativeRuntimeNotReproduced,
            FlowMetadataPrefixDiagnostic::NativeApplicationNotChecked
        ]
    );
}
fn check_vectors(view: &FlowMetadataPrefix, case: &PrefixCase) {
    assert_eq!(view.vectors().len(), case.vectors.len());
    for (actual, expected) in view.vectors().zip(&case.vectors) {
        assert_eq!(actual.ordinal(), expected.ordinal as usize);
        assert_eq!(
            actual.payload_interval(),
            (expected.start as usize, expected.end as usize)
        );
        assert_eq!(actual.values().collect::<Vec<_>>(), expected.values);
        assert_eq!(
            view.vector(actual.ordinal())
                .unwrap()
                .values()
                .collect::<Vec<_>>(),
            expected.values
        );
    }
    assert!(view.vector(case.vectors.len()).is_none());
}
fn check_object(actual: FlowMetadataObject<'_>, expected: &PrefixObject) {
    assert_eq!(actual.ordinal(), expected.ordinal as usize);
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(actual.state_byte(), expected.state);
    assert_eq!(actual.native_state(), expected.native_state);
    assert_eq!(actual.member_c4(), expected.word_c4);
    assert_eq!(actual.fourth_text_is_derived(), expected.derived_fourth);
    assert_eq!(actual.native_fourth_text_bytes(), expected.native_fourth);
    check_object_strings(actual, expected);
}
fn check_object_strings(actual: FlowMetadataObject<'_>, expected: &PrefixObject) {
    let strings = [
        Some(&expected.first),
        Some(&expected.second),
        Some(&expected.third),
        expected.fourth.as_ref(),
    ];
    for (role, expected) in roles().into_iter().zip(strings) {
        match (actual.string(role), expected) {
            (Some(text), Some(expected)) => check_text(text, expected),
            (None, None) => (),
            _ => panic!("stored string presence differs"),
        }
    }
}
fn check_text(text: FlowMetadataText<'_>, expected: &PrefixString) {
    assert_eq!(
        text.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(text.stored_units(), expected.units as usize);
    assert_eq!(text.raw_bytes(), expected.raw);
    assert_eq!(text.native_text_bytes(), expected.native_text);
}
