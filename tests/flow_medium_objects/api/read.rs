use super::*;
#[test]
fn every_complete_kind_one_body_matches_all_frozen_fields_spans_and_exclusions() {
    for case in fixture::cases() {
        let body = open(&case.raw, options()).unwrap();
        check_body(&body, &case);
    }
}
pub(crate) fn check_body(body: &FlowMediumKindOne, case: &BodyCase) {
    assert_eq!(body.body_bytes(), &case.raw[..case.footer.end as usize]);
    assert_eq!(body.opaque_tail(), case.footer.tail);
    assert_eq!(body.base_byte(), case.prefix.base);
    assert_eq!(body.base_key_bytes(), case.prefix.key);
    assert_eq!(body.state_byte(), case.footer.state);
    assert_eq!(body.native_state_after_load(), case.footer.loaded_state);
    assert_eq!(
        body.options().maximum_body_bytes,
        options().maximum_body_bytes
    );
    assert_eq!(
        body.backend_prefix().metadata_prefix().record().ordinal(),
        1
    );
    check_text(body, case);
    check_curves(body, case);
    check_words(body, case);
    check_coverage(body, case);
}
fn check_text(body: &FlowMediumKindOne, case: &BodyCase) {
    let text = body.name();
    let base = body.payload_interval().0;
    assert_eq!(
        text.payload_interval(),
        (
            base + case.prefix.text.start as usize,
            base + case.prefix.text.end as usize
        )
    );
    assert_eq!(text.stored_units(), case.prefix.text.units as usize);
    assert_eq!(text.raw_bytes(), case.prefix.text.raw);
    assert_eq!(text.native_text_bytes(), case.prefix.text.native);
}
fn check_curves(body: &FlowMediumKindOne, case: &BodyCase) {
    assert_eq!(body.curves().len(), 7);
    assert!(body.curve(7).is_none());
    for (curve, expected) in body.curves().zip(&case.prefix.curves) {
        assert_eq!(
            curve.body_interval(),
            (expected.start as usize, expected.end as usize)
        );
        assert_eq!(curve.value_count(), expected.count as usize);
        assert_eq!(curve.scalar_pattern(), expected.scalar);
        assert_eq!(curve.extra_flag_byte(), expected.flag);
        assert_eq!(curve.extra_arrays_active(), expected.extra_count.is_some());
        assert_eq!(
            body.curve(curve.ordinal()).unwrap().scalar_pattern(),
            curve.scalar_pattern()
        );
    }
}
fn check_words(body: &FlowMediumKindOne, case: &BodyCase) {
    for (field, offset, expected) in fields(case) {
        let words = body.wide_words(field).unwrap();
        let start = body.payload_interval().0 + offset;
        assert_eq!(
            words.payload_interval(),
            (start, start + expected.len() * 8)
        );
        assert_eq!(words.word_count(), expected.len());
        assert_eq!(
            words.as_bytes(),
            &case.raw[offset..offset + expected.len() * 8]
        );
        for (index, bits) in expected.into_iter().enumerate() {
            assert_eq!(words.raw_bits(index), Some(bits));
        }
        assert!(words.raw_bits(words.word_count()).is_none());
    }
}
fn check_coverage(body: &FlowMediumKindOne, case: &BodyCase) {
    let coverage = body.coverage();
    assert_eq!(coverage.body_bytes, case.footer.end as usize);
    assert_eq!(
        coverage.wide_words,
        fields(case).iter().map(|(_, _, words)| words.len()).sum()
    );
    assert_eq!(coverage.name_units, case.prefix.text.units as usize);
    assert!(coverage.body_framing_complete);
    assert!(!coverage.full_record_schema);
    assert_eq!(coverage.achieved_error, 0);
    assert_eq!(
        body.diagnostics(),
        [
            FlowMediumBodyDiagnostic::EngineeringAndEvaluationNotChecked,
            FlowMediumBodyDiagnostic::NativeRuntimeNotReproduced,
            FlowMediumBodyDiagnostic::FollowingRecordNotDecoded,
            FlowMediumBodyDiagnostic::NativeApplicationNotChecked
        ]
    );
}

#[test]
fn complete_native_record_composes_the_actual_metadata_backend_and_body_offsets() {
    let body = native_body();
    assert_eq!(body.payload_interval(), (138, 13111));
    check_body(&body, &fixture::cases().remove(0));
    check_native_reverse(&body);
}
fn native_record_payload() -> Vec<u8> {
    let mut cursor = crate::packet::cursor(
        include_bytes!("../../fixtures/flow-metadata/prefix.bin.gz"),
        *b"FMP1",
    );
    assert_eq!(crate::packet::word(&mut cursor), 537);
    let name = crate::packet::blob(&mut cursor);
    assert!(name.starts_with(b"native-"));
    crate::packet::blob(&mut cursor)
}

fn native_body() -> FlowMediumKindOne {
    let raw = native_record_payload();
    let source = carrier(&raw);
    let id = source.latest(424).unwrap().unwrap().id;
    let prefix = source
        .record_424_metadata_prefix(&id, prefix_options())
        .unwrap();
    prefix
        .medium_backend_prefix()
        .unwrap()
        .first_kind_one_body(options())
        .unwrap()
}
fn check_native_reverse(body: &FlowMediumKindOne) {
    let field = FlowMediumWideField::FinalScalars;
    let old = body.wide_words(field).unwrap().raw_bits(3).unwrap();
    let edit = body.replace_wide_word(field, 3, 1).unwrap();
    let reverse = edit.result().replace_wide_word(field, 3, old).unwrap();
    let original = body.backend_prefix().metadata_prefix().source();
    assert_eq!(
        reverse.prefix_edit().carrier_edit().result().as_bytes(),
        original.as_bytes()
    );
}

#[test]
fn native_name_unit_edits_recompute_lossless_text_without_normalizing_stored_state() {
    let body = native_body();
    let nul = body.replace_name_unit(0, 0).unwrap();
    assert!(nul.result().name().native_text_bytes().is_empty());
    let surrogate = body.replace_name_unit(0, 0xd800).unwrap();
    assert_eq!(
        &surrogate.result().name().native_text_bytes()[..2],
        &[0, 0xd8]
    );
    let stored = body.replace_state_byte(0).unwrap();
    assert_eq!(stored.result().state_byte(), 0);
    assert_eq!(stored.result().native_state_after_load(), 1);
    assert_eq!(stored.result().opaque_tail(), body.opaque_tail());
    assert_eq!(body.state_byte(), 1);
}
