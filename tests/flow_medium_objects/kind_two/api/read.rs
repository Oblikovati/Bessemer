use super::*;
#[test]
fn every_frozen_kind_two_body_matches_all_six_typed_models_raw_fields_spans_and_exclusions() {
    for case in kind_fixture::cases() {
        let body = open(&case.raw, options()).unwrap();
        check_body(&body, &case);
    }
}
pub(crate) fn check_body(body: &FlowMediumKindTwo, case: &KindTwoCase) {
    check_body_at(body, case, 35);
}
pub(crate) fn check_body_at(body: &FlowMediumKindTwo, case: &KindTwoCase, start: usize) {
    check_header(body, case, start);
    check_name(body, &case.prefix.text);
    check_curves(body, FlowMediumCurveScope::Body, &case.prefix.curves);
    check_model(body, &case.model);
    check_wides(body, case);
    check_coverage(body, case);
}
fn check_name(body: &FlowMediumKindTwo, text: &BodyText) {
    assert_eq!(body.name().stored_units(), text.units as usize);
    assert_eq!(body.name().raw_bytes(), text.raw);
    assert_eq!(body.name().native_text_bytes(), text.native);
    let base = body.payload_interval().0;
    assert_eq!(
        body.name().payload_interval(),
        (base + text.start as usize, base + text.end as usize)
    );
}
fn check_curves(body: &FlowMediumKindTwo, scope: FlowMediumCurveScope, curves: &[BodyCurve]) {
    assert_eq!(body.curves(scope).len(), curves.len());
    assert!(body.curve(scope, curves.len()).is_none());
    for (actual, expected) in body.curves(scope).zip(curves) {
        assert_eq!(
            actual.body_interval(),
            (expected.start as usize, expected.end as usize)
        );
        assert_eq!(actual.value_count(), expected.count as usize);
        assert_eq!(actual.scalar_pattern(), expected.scalar);
        assert_eq!(actual.extra_flag_byte(), expected.flag);
        assert_eq!(actual.extra_arrays_active(), expected.extra_count.is_some());
        assert_eq!(
            body.curve(scope, actual.ordinal())
                .unwrap()
                .scalar_pattern(),
            actual.scalar_pattern()
        );
    }
}
fn check_model(body: &FlowMediumKindTwo, model: &ModelCase) {
    let curves = match &model.payload {
        ModelPayload::Curves(value) => value.curves.as_slice(),
        _ => &[],
    };
    check_curves(body, FlowMediumCurveScope::Model, curves);
    assert_eq!(body.model_state_bytes(), expected::states(model).1);
    if let ModelPayload::Table(value) = &model.payload {
        check_table(body, model, value);
        return;
    }
    assert!(body.model_counts().is_none());
    assert!(body.model_word_patterns().is_none());
    assert!(body.model_member_8c().is_none());
}
fn check_table(body: &FlowMediumKindTwo, model: &ModelCase, table: &TableModel) {
    assert_eq!(
        body.model_counts(),
        Some([
            table.first_count as usize,
            table.second_count as usize,
            table.third_count as usize
        ])
    );
    assert_eq!(body.model_member_8c(), Some(table.member));
    let words = body.model_word_patterns().unwrap();
    assert_eq!(words.word_count(), table.word_patterns.len());
    let start = body.payload_interval().0 + expected::word_span(model, table);
    assert_eq!(
        words.payload_interval(),
        (start, start + table.word_patterns.len() * 4)
    );
    for (index, bits) in table.word_patterns.iter().enumerate() {
        assert_eq!(words.raw_word(index), Some(*bits));
    }
    assert!(words.raw_word(words.word_count()).is_none());
}
fn check_wides(body: &FlowMediumKindTwo, case: &KindTwoCase) {
    for (field, offset, expected) in expected::fields(case) {
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
fn check_coverage(body: &FlowMediumKindTwo, case: &KindTwoCase) {
    let coverage = body.coverage();
    assert_eq!(coverage.body_bytes, case.footer.end as usize);
    assert_eq!(coverage.name_units, case.prefix.text.units as usize);
    assert_eq!(coverage.model_kind.as_i32(), case.model.kind);
    assert!(coverage.body_framing_complete);
    assert!(!coverage.full_record_schema);
    assert!(!coverage.installed_fixture_validated);
    assert_eq!(coverage.achieved_error, 0);
    assert_eq!(
        coverage.wide_words,
        expected::fields(case)
            .iter()
            .map(|(_, _, words)| words.len())
            .sum()
    );
    let count = word_count(case);
    assert_eq!(coverage.word_patterns, count);
    check_diagnostics(body);
}

fn check_header(body: &FlowMediumKindTwo, case: &KindTwoCase, start: usize) {
    assert_eq!(body.body_bytes(), &case.raw[..case.footer.end as usize]);
    assert_eq!(body.opaque_tail(), case.footer.tail);
    assert_eq!(
        body.payload_interval(),
        (start, start + case.footer.end as usize)
    );
    assert_eq!(body.base_byte(), case.prefix.base);
    assert_eq!(body.base_key_bytes(), case.prefix.key);
    assert_eq!(body.model_kind().as_i32(), case.model.kind);
    assert_eq!(
        body.model_interval(),
        (case.model.start as usize, case.model.end as usize)
    );
    assert_eq!(body.state_byte(), case.footer.state);
    assert_eq!(body.native_state_after_load(), case.footer.loaded);
    check_header_context(body);
}
fn check_header_context(body: &FlowMediumKindTwo) {
    assert_eq!(
        body.options().maximum_word_patterns,
        options().maximum_word_patterns
    );
    assert_eq!(
        body.backend_prefix().metadata_prefix().record().ordinal(),
        1
    );
}

fn check_diagnostics(body: &FlowMediumKindTwo) {
    assert_eq!(
        body.diagnostics(),
        [
            FlowMediumKindTwoDiagnostic::InstalledFixtureNotValidated,
            FlowMediumKindTwoDiagnostic::EngineeringAndEvaluationNotChecked,
            FlowMediumKindTwoDiagnostic::NativeRuntimeNotReproduced,
            FlowMediumKindTwoDiagnostic::FollowingRecordNotDecoded,
            FlowMediumKindTwoDiagnostic::NativeApplicationNotChecked
        ]
    );
}

#[test]
fn name_edits_recompute_first_nul_text_and_preserve_surrogates_and_raw_loaded_state_distinctions() {
    for kind in 0..6 {
        let case = kind_fixture::cases()
            .into_iter()
            .find(|case| case.name == format!("tag-{kind}-text-2"))
            .unwrap();
        let body = open(&case.raw, options()).unwrap();
        let nul = body.replace_name_unit(0, 0).unwrap();
        assert!(nul.result().name().native_text_bytes().is_empty());
        let surrogate = body.replace_name_unit(0, 0xd800).unwrap();
        assert_eq!(
            &surrogate.result().name().native_text_bytes()[..2],
            &[0, 0xd8]
        );
        let state = body.replace_state_byte(0).unwrap();
        assert_eq!(state.result().state_byte(), 0);
        assert_eq!(state.result().native_state_after_load(), 1);
        assert_eq!(body.body_bytes(), &case.raw[..case.footer.end as usize]);
    }
}
