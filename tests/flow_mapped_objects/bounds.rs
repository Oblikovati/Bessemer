use super::*;
use bessemer::FailureClass;

#[test]
fn every_fixture_prefix_and_extension_rejects_without_partial_publication() {
    for case in fixture::cases() {
        for end in 0..case.payload.len() {
            assert!(
                source(&case.payload[..end])
                    .mapped_objects(limits(), context(&case.contexts))
                    .is_err()
            );
        }
        let mut extended = case.payload.clone();
        extended.push(0);
        assert!(
            source(&extended)
                .mapped_objects(limits(), context(&case.contexts))
                .is_err()
        );
    }
}

#[test]
fn required_context_is_not_inferred_from_pairs_and_null_counts_are_invalid() {
    let case = fixture::cases().remove(2);
    let missing = source(&case.payload)
        .mapped_objects(limits(), FlowMappedContext::Unavailable)
        .unwrap_err();
    assert_eq!(missing.class, FailureClass::Missing);
    assert_eq!(missing.operation, "Flow mapped objects");
    for counts in [vec![], vec![Some(0)], vec![Some(2)], vec![Some(1), Some(1)]] {
        assert!(
            source(&case.payload)
                .mapped_objects(limits(), context(&Some(counts)))
                .is_err()
        );
    }
    let null = [4_u32.to_le_bytes(), 1_u32.to_le_bytes()].concat();
    let null = [null.as_slice(), &[0]].concat();
    assert!(
        source(&null)
            .mapped_objects(limits(), context(&Some(vec![Some(0)])))
            .is_err()
    );
}

#[test]
fn matched_count_boundary_is_normalized_once_without_saturating_or_negative_casts() {
    for value in [0, 1, 65535, i32::MAX as u32] {
        assert_eq!(
            FlowMatchedObjectCount::try_from(value).unwrap().value(),
            value
        );
    }
    for value in [i32::MAX as u32 + 1, u32::MAX] {
        assert!(FlowMatchedObjectCount::try_from(value).is_err());
    }
}

#[test]
fn numeric_and_pair_budgets_apply_to_the_complete_list() {
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.name == "mixed")
        .unwrap();
    let coverage = list(&case).coverage();
    for bound in under_bounds(coverage) {
        reject_parse(&case.payload, bound, context(&case.contexts));
    }
    let exact = FlowMappedLimits {
        maximum_entries: coverage.checked_entries,
        maximum_pairs: coverage.checked_pairs,
        maximum_numeric_words: coverage.checked_numeric_words,
    };
    assert!(
        source(&case.payload)
            .mapped_objects(exact, context(&case.contexts))
            .is_ok()
    );
}

#[test]
fn invalid_limits_and_edit_indices_fail_without_mutation() {
    let case = fixture::cases().remove(2);
    let parsed = list(&case);
    for invalid in invalid_limits() {
        reject_parse(&case.payload, invalid, context(&case.contexts));
    }
    for index in [1, usize::MAX] {
        reject_field(&parsed, index, FlowMappedFieldEdit::State(7));
    }
    reject_bad_pair(&parsed);
    reject_numeric(&parsed, FlowMappedNumericField::Prefix, 9);
    reject_numeric(&parsed, FlowMappedNumericField::Mapped, 2);
    reject_numeric(&parsed, FlowMappedNumericField::LegacyPrefix, 0);
    assert_eq!(parsed.payload(), case.payload);
    let null = fixture::cases()
        .into_iter()
        .find(|case| case.name == "nulls")
        .unwrap();
    reject_field(&list(&null), 0, FlowMappedFieldEdit::State(7));
}

fn invalid_limits() -> [FlowMappedLimits; 3] {
    let mut entries = limits();
    entries.maximum_entries = 0;
    let mut pairs = limits();
    pairs.maximum_pairs = 0;
    let mut words = limits();
    words.maximum_numeric_words = 0;
    [entries, pairs, words]
}

#[test]
fn negative_counts_oversized_products_and_unknown_versions_reject() {
    let case = fixture::cases().remove(2);
    for (offset, value) in [
        (4, u32::MAX),
        (86, u32::MAX),
        (106, u32::MAX),
        (106, i32::MAX as u32),
        (0, 7),
        (9, 7),
    ] {
        reject_word(&case, offset, value);
    }
    let legacy = fixture::cases().remove(1);
    reject_word(&legacy, 254, i32::MAX as u32);
}

#[test]
fn missing_records_and_last_duplicates_keep_absence_distinct_from_failure() {
    check_missing_record();
    let case = fixture::cases().remove(2);
    let mut raw = wire(&case.payload);
    append_record(&mut raw, 144, &[]);
    let duplicate = FlowCarrier::from_bytes(&raw, wire_limits()).unwrap();
    assert!(
        duplicate
            .mapped_objects(limits(), context(&case.contexts))
            .is_err()
    );
    assert_eq!(duplicate.as_bytes(), raw);
}

fn under_bounds(coverage: FlowMappedCoverage) -> [FlowMappedLimits; 3] {
    let mut entries = limits();
    entries.maximum_entries = coverage.checked_entries - 1;
    let mut pairs = limits();
    pairs.maximum_pairs = coverage.checked_pairs - 1;
    let mut words = limits();
    words.maximum_numeric_words = coverage.checked_numeric_words - 1;
    [entries, pairs, words]
}

fn reject_parse(payload: &[u8], bounds: FlowMappedLimits, input: FlowMappedContext) {
    assert!(source(payload).mapped_objects(bounds, input).is_err());
}

fn reject_bad_pair(parsed: &FlowMappedObjects) {
    reject_field(
        parsed,
        0,
        FlowMappedFieldEdit::PairSource { index: 2, value: 1 },
    );
    reject_field(
        parsed,
        0,
        FlowMappedFieldEdit::PairValue {
            index: usize::MAX,
            value: 1,
        },
    );
}

fn reject_field(parsed: &FlowMappedObjects, entry: usize, edit: FlowMappedFieldEdit) {
    assert!(parsed.replace_field(entry, edit).is_err());
}

fn reject_numeric(parsed: &FlowMappedObjects, field: FlowMappedNumericField, index: usize) {
    reject_field(
        parsed,
        0,
        FlowMappedFieldEdit::Numeric {
            field,
            index,
            bits: 1,
        },
    );
}

fn reject_word(case: &fixture::MappedCase, offset: usize, value: u32) {
    let mut raw = case.payload.clone();
    raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    reject_parse(&raw, limits(), context(&case.contexts));
}

fn check_missing_record() {
    let empty = FlowCarrier::from_bytes(&8_u16.to_le_bytes(), wire_limits()).unwrap();
    assert!(
        empty
            .mapped_objects(limits(), FlowMappedContext::Unavailable)
            .unwrap()
            .is_none()
    );
    assert!(
        empty
            .mapped_objects(invalid_limits()[0], FlowMappedContext::Unavailable)
            .is_err()
    );
}
