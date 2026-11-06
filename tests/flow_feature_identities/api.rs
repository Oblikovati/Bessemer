use super::*;
#[path = "../flow_carriers/fixture.rs"]
mod native_fixture;

#[test]
fn all_22_identity_lists_and_87_entries_match_source_fields_raw_units_and_coverage() {
    let mut objects = 0;
    for case in fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 143)
    {
        assert!(case.flags.is_none());
        let parsed = list(&case.payload);
        check_list_headers(&parsed, &case);
        for (actual, expected) in parsed.objects().iter().zip(&case.objects) {
            check_object(actual, expected);
        }
        check_tail(&parsed, &case);
        check_coverage(&parsed, &case);
        objects += parsed.objects().len();
    }
    assert_eq!(objects, 87);
}

fn units(bytes: &[u8]) -> Vec<u16> {
    assert_eq!(bytes.len() % 2, 0);
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|bytes| u16::from_le_bytes(*bytes))
        .collect()
}

fn check_object(actual: &FlowFeatureIdentity, expected: &fixture::ExpectedObject) {
    let [start, end, header, kind, body, count, text_start] = expected.words;
    assert_eq!(actual.payload_interval(), (start as usize, end as usize));
    assert_eq!(actual.header_version(), header);
    assert_eq!(actual.type_code(), kind);
    assert_eq!(actual.body_version(), body);
    assert_eq!(actual.native_class().native_name(), expected.class);
    assert_eq!(actual.wire_text_units(), units(&expected.text));
    assert_eq!(actual.wire_text_units().len(), count as usize);
    assert_eq!(actual.identity_units(), units(&expected.logical));
    assert_eq!(text_start, start + 16);
    assert_eq!(actual.state_byte(), expected.state);
    assert_eq!(actual.state_boolean(), expected.boolean);
}

fn check_tail(parsed: &FlowFeatureIdentities, case: &fixture::FeatureCase) {
    if let Some(expected) = &case.tail {
        let FlowFeatureTail::Stored(actual) = parsed.tail() else {
            panic!("expected stored tail")
        };
        assert_eq!(actual.as_ref(), expected);
        let contextual = read_context(parsed, Some(i32::MIN));
        assert_eq!(contextual.tail(), parsed.tail());
        return;
    }
    for (context, count, derived) in &case.legacy {
        let result = read_context(parsed, *context);
        assert_eq!(result.options().legacy_tail_count, *context);
        assert_eq!(result.payload(), parsed.payload());
        assert_eq!(*result.tail(), expected_tail(*context, *count, *derived));
    }
}

fn check_coverage(parsed: &FlowFeatureIdentities, case: &fixture::FeatureCase) {
    let coverage = parsed.coverage();
    assert_eq!(coverage.mode, FlowFeatureCheckMode::ExactRawIdentityList);
    assert_eq!(coverage.checked_bytes, case.payload.len(), "{}", case.name);
    assert_eq!(coverage.checked_objects, case.objects.len());
    assert_eq!(
        coverage.checked_text_units,
        case.objects
            .iter()
            .map(|object| object.text.len() / 2)
            .sum()
    );
    assert_eq!(
        coverage.checked_tail_values,
        case.tail.as_ref().map_or(0, Vec::len)
    );
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(parsed.diagnostics().len(), 3);
}

#[test]
fn all_three_complete_native_sources_preserve_optional_presence_and_selected_payloads() {
    let mut present = 0;
    for case in native_fixture::cases().into_iter().take(3) {
        let source = FlowCarrier::from_bytes(&case.raw, native_fixture::limits()).unwrap();
        let parsed = source.feature_identities(options()).unwrap();
        assert_eq!(
            parsed.is_some(),
            case.records.iter().any(|record| record.0 == 143)
        );
        if let Some(parsed) = parsed {
            assert_eq!(parsed.objects().len(), 3, "{}", case.name);
            assert_eq!(parsed.payload().len(), 211);
            assert_eq!(parsed.source().as_bytes(), case.raw);
            present += 1;
        }
    }
    assert_eq!(present, 1);
}

#[test]
fn published_contracts_are_send_sync_and_caller_mutation_does_not_change_shared_views() {
    assert_public_traits();
    let mut input = wire(&fixture::cases()[0].payload);
    let source = FlowCarrier::from_bytes(&input, limits()).unwrap();
    let parsed = source.feature_identities(options()).unwrap().unwrap();
    input.fill(0);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let shared = &parsed;
            scope.spawn(move || check_parallel_edit(shared));
        }
    });
    assert_eq!(parsed.objects()[0].state_byte(), 1);
}

fn check_parallel_edit(parsed: &FlowFeatureIdentities) {
    let edited = parsed
        .replace_object_field(0, FlowFeatureFieldEdit::State(255))
        .unwrap();
    assert_eq!(edited.result().objects()[0].state_byte(), 255);
    assert_eq!(parsed.objects()[0].state_byte(), 1);
    let reversed = edited
        .result()
        .replace_object_field(0, FlowFeatureFieldEdit::State(1))
        .unwrap();
    assert_eq!(
        reversed.result().source().as_bytes(),
        parsed.source().as_bytes()
    );
}

fn assert_public_traits() {
    assert_traits::<FlowFeatureOptions>();
    assert_traits::<FlowFeatureClass>();
    assert_traits::<FlowFeatureIdentity>();
    assert_traits::<FlowFeatureTail>();
    assert_traits::<FlowFeatureIdentities>();
    assert_traits::<FlowFeatureFieldEdit>();
    assert_traits::<FlowFeatureEdit>();
    assert_traits::<FlowFeatureCheckMode>();
    assert_traits::<FlowFeatureCoverage>();
    assert_traits::<FlowFeatureDiagnostic>();
}

fn assert_traits<T: Send + Sync>() {}

fn check_list_headers(parsed: &FlowFeatureIdentities, case: &fixture::FeatureCase) {
    assert_eq!(parsed.source().as_bytes(), wire(&case.payload));
    assert_eq!(
        parsed.record(),
        &parsed.source().latest(143).unwrap().unwrap().id
    );
    assert_eq!(parsed.payload(), case.payload);
    assert_eq!(parsed.version(), case.version);
    assert_eq!(parsed.objects().len(), case.count);
}

fn read_context(parsed: &FlowFeatureIdentities, context: Option<i32>) -> FlowFeatureIdentities {
    let mut configured = options();
    configured.legacy_tail_count = context;
    parsed
        .source()
        .feature_identities(configured)
        .unwrap()
        .unwrap()
}

fn expected_tail(context: Option<i32>, count: u32, derived: bool) -> FlowFeatureTail {
    if !derived {
        return FlowFeatureTail::RequiresRuntimeCount;
    }
    FlowFeatureTail::DerivedRuntime {
        runtime_count: context.unwrap(),
        count,
        fill_value: 2,
    }
}
