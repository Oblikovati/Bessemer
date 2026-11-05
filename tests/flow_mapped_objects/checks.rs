use super::*;
#[path = "../flow_carriers/fixture.rs"]
mod native_fixture;

pub(super) type ExpectedBlock<'a> = (FlowMappedNumericField, usize, &'a [u64]);

pub(super) fn blocks<'a>(
    entry: &fixture::ExpectedObject,
    body: &'a fixture::ExpectedBody,
) -> Vec<ExpectedBlock<'a>> {
    let numeric = entry.start + 6;
    let mapped = numeric + 9 * 8 + 4 + body.pairs.len() * 8 + 4;
    let mut blocks = vec![
        (
            FlowMappedNumericField::Prefix,
            numeric,
            body.numeric.as_slice(),
        ),
        (
            FlowMappedNumericField::Mapped,
            mapped,
            body.mapped.as_slice(),
        ),
    ];
    if let Some(legacy) = &body.legacy {
        legacy_blocks(&mut blocks, mapped + body.mapped.len() * 8, legacy);
    }
    blocks
}

fn legacy_blocks<'a>(
    blocks: &mut Vec<ExpectedBlock<'a>>,
    start: usize,
    legacy: &'a fixture::ExpectedLegacy,
) {
    blocks.push((FlowMappedNumericField::LegacyPrefix, start, &legacy.prefix));
    let mut offset = start + 16 * 8;
    for (field, values) in [
        FlowMappedNumericField::LegacyFirst,
        FlowMappedNumericField::LegacySecond,
        FlowMappedNumericField::LegacyThird,
    ]
    .into_iter()
    .zip(&legacy.arrays)
    {
        offset += 4;
        blocks.push((field, offset, values));
        offset += values.len() * 8;
    }
    blocks.push((
        FlowMappedNumericField::LegacyTripled,
        offset,
        &legacy.tripled,
    ));
}

pub(super) fn actual_block(
    body: &FlowMappedBody,
    field: FlowMappedNumericField,
) -> &FlowMappedNumericBlock {
    use FlowMappedNumericField::*;
    match field {
        Prefix => body.numeric(),
        Mapped => body.mapped_values(),
        LegacyPrefix => body.legacy().unwrap().prefix(),
        LegacyFirst => &body.legacy().unwrap().arrays()[0],
        LegacySecond => &body.legacy().unwrap().arrays()[1],
        LegacyThird => &body.legacy().unwrap().arrays()[2],
        LegacyTripled => body.legacy().unwrap().tripled_values(),
    }
}

#[test]
fn all_192_context_profiles_match_every_raw_field_interval_and_numeric_word() {
    for case in fixture::cases() {
        let parsed = list(&case);
        assert_eq!(parsed.source().as_bytes(), wire(&case.payload));
        assert_eq!(
            parsed.record(),
            &parsed.source().latest(144).unwrap().unwrap().id
        );
        assert_eq!(parsed.version(), case.version);
        assert_eq!(parsed.payload(), case.payload, "{}", case.name);
        assert_eq!(parsed.entries().len(), case.objects.len());
        for (actual, expected) in parsed.entries().iter().zip(&case.objects) {
            check_entry(actual, expected);
        }
        check_context(&parsed, &case);
        check_coverage(&parsed, &case);
    }
}

fn check_entry(actual: &FlowMappedEntry, expected: &fixture::ExpectedObject) {
    assert_eq!(actual.payload_interval(), (expected.start, expected.end));
    assert_eq!(actual.presence_byte(), expected.presence);
    assert_eq!(actual.body().is_some(), expected.body.is_some());
    if let Some(body) = &expected.body {
        check_body(actual.body().unwrap(), expected, body);
    }
}

fn check_body(
    actual: &FlowMappedBody,
    entry: &fixture::ExpectedObject,
    expected: &fixture::ExpectedBody,
) {
    assert_eq!(actual.version(), expected.version);
    assert_eq!(actual.state_byte(), expected.state);
    assert_eq!(actual.stored_factor(), expected.factor);
    assert_eq!(
        actual.matched_count().map(|value| value.value() as i32),
        expected.matched
    );
    assert_eq!(
        actual.mapped_values().raw_bits().len(),
        expected.mapped_count
    );
    assert_eq!(actual.legacy().is_some(), expected.legacy.is_some());
    assert!(!expected.context_validated);
    check_body_arrays(actual, entry, expected);
}

fn check_context(parsed: &FlowMappedObjects, case: &fixture::MappedCase) {
    match (parsed.context(), &case.contexts) {
        (FlowMappedContext::Unavailable, None) => {}
        (FlowMappedContext::Aligned(actual), Some(expected)) => {
            let counts: Vec<Option<i32>> = actual
                .iter()
                .map(|value| value.map(|value| value.value() as i32))
                .collect();
            assert_eq!(counts, *expected);
        }
        other => panic!("context mismatch {other:?}"),
    }
    assert_eq!(
        parsed.limits().maximum_numeric_words,
        limits().maximum_numeric_words
    );
}

fn check_coverage(parsed: &FlowMappedObjects, case: &fixture::MappedCase) {
    let coverage = parsed.coverage();
    assert_eq!(
        coverage.mode,
        FlowMappedCheckMode::ExactExplicitContextFrames
    );
    assert_eq!(coverage.checked_bytes, case.payload.len());
    assert_eq!(coverage.checked_entries, case.objects.len());
    assert_eq!(coverage.checked_numeric_words, expected_words(case));
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(parsed.diagnostics().len(), 3);
    check_context_coverage(coverage, case);
}

#[test]
fn complete_native_carriers_confirm_two_absences_and_one_empty_list() {
    let mut present = 0;
    for case in native_fixture::cases().into_iter().take(3) {
        let carrier = FlowCarrier::from_bytes(&case.raw, native_fixture::limits()).unwrap();
        let parsed = carrier
            .mapped_objects(limits(), FlowMappedContext::Unavailable)
            .unwrap();
        assert_eq!(
            parsed.is_some(),
            case.records.iter().any(|record| record.0 == 144)
        );
        if let Some(parsed) = parsed {
            assert!(parsed.entries().is_empty(), "{}", case.name);
            assert_eq!(parsed.payload().len(), 8);
            assert_eq!(parsed.source().as_bytes(), case.raw);
            present += 1;
        }
    }
    assert_eq!(present, 1);
}

#[test]
fn all_published_contracts_are_send_sync() {
    assert_traits::<FlowMatchedObjectCount>();
    assert_traits::<FlowMappedContext>();
    assert_traits::<FlowMappedLimits>();
    assert_traits::<FlowMappedIdentifierPair>();
    assert_traits::<FlowMappedNumericBlock>();
    assert_traits::<FlowMappedLegacy>();
    assert_traits::<FlowMappedBody>();
    assert_traits::<FlowMappedEntry>();
    assert_traits::<FlowMappedObjects>();
    assert_traits::<FlowMappedCheckMode>();
    assert_traits::<FlowMappedCoverage>();
    assert_traits::<FlowMappedDiagnostic>();
    assert_traits::<FlowMappedNumericField>();
    assert_traits::<FlowMappedFieldEdit>();
    assert_traits::<FlowMappedEdit>();
}

fn assert_traits<T: Send + Sync>() {}

fn check_body_arrays(
    actual: &FlowMappedBody,
    entry: &fixture::ExpectedObject,
    expected: &fixture::ExpectedBody,
) {
    let pairs: Vec<(i32, i32)> = actual
        .pairs()
        .iter()
        .map(|pair| (pair.source_identifier(), pair.associated_value()))
        .collect();
    assert_eq!(pairs, expected.pairs);
    for (field, offset, bits) in blocks(entry, expected) {
        let block = actual_block(actual, field);
        assert_eq!(block.payload_offset(), offset);
        assert_eq!(block.raw_bits(), bits);
        assert_eq!(
            block.numeric_values().map(f64::to_bits).collect::<Vec<_>>(),
            bits
        );
    }
}

fn expected_words(case: &fixture::MappedCase) -> usize {
    case.objects
        .iter()
        .flat_map(|entry| {
            entry
                .body
                .as_ref()
                .map(|body| blocks(entry, body))
                .unwrap_or_default()
        })
        .map(|block| block.2.len())
        .sum()
}

fn check_context_coverage(coverage: FlowMappedCoverage, case: &fixture::MappedCase) {
    let bodies: Vec<_> = case
        .objects
        .iter()
        .filter_map(|entry| entry.body.as_ref())
        .collect();
    assert_eq!(
        coverage.checked_pairs,
        bodies.iter().map(|body| body.pairs.len()).sum::<usize>()
    );
    assert_eq!(
        coverage.supplied_matched_counts,
        bodies.iter().filter(|body| body.matched.is_some()).count()
    );
    assert_eq!(
        coverage.unresolved_zero_factor_counts,
        bodies.iter().filter(|body| body.matched.is_none()).count()
    );
}
