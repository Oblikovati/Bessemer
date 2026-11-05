//! Independent complete raw-body expectations before the typed context-dependent decoder.
#[path = "flow_mapped_objects/fixture.rs"]
mod fixture;
use bessemer::flow::*;

#[test]
fn all_192_cases_rebuild_every_entry_and_numeric_bit_without_pair_count_inference() {
    for case in fixture::cases() {
        assert!(case.version <= 6);
        if let Some(contexts) = &case.contexts {
            assert_eq!(contexts.len(), case.objects.len());
        }
        let mut rebuilt = case.version.to_le_bytes().to_vec();
        rebuilt.extend(i32::try_from(case.objects.len()).unwrap().to_le_bytes());
        for (index, object) in case.objects.iter().enumerate() {
            let context = case.contexts.as_ref().and_then(|values| values[index]);
            append_object(&mut rebuilt, object, context);
        }
        assert_eq!(rebuilt, case.payload, "{}", case.name);
        check_carrier(&case);
    }
}

fn append_object(output: &mut Vec<u8>, object: &fixture::ExpectedObject, context: Option<i32>) {
    assert_eq!(output.len(), object.start);
    output.push(object.presence);
    if let Some(body) = &object.body {
        assert_ne!(object.presence, 0);
        append_body(output, body, context);
    } else {
        assert_eq!(object.presence, 0);
        assert_eq!(context, None);
    }
    assert_eq!(output.len(), object.end);
}

fn append_body(output: &mut Vec<u8>, body: &fixture::ExpectedBody, context: Option<i32>) {
    assert!(body.version <= 6);
    output.extend(body.version.to_le_bytes());
    output.push(body.state);
    assert_eq!(body.numeric.len(), 9);
    append_words(output, &body.numeric);
    output.extend(i32::try_from(body.pairs.len()).unwrap().to_le_bytes());
    for (source, value) in &body.pairs {
        output.extend(source.to_le_bytes());
        output.extend(value.to_le_bytes());
    }
    output.extend(i32::try_from(body.factor).unwrap().to_le_bytes());
    check_matched_count(body, context);
    append_words(output, &body.mapped);
    assert_eq!(body.legacy.is_some(), body.version == 5);
    if let Some(legacy) = &body.legacy {
        append_legacy(output, legacy);
    }
    assert!(!body.context_validated);
}

fn check_matched_count(body: &fixture::ExpectedBody, context: Option<i32>) {
    assert_eq!(body.matched, context);
    let expected = if body.factor == 0 {
        0
    } else {
        body.factor as usize * usize::try_from(context.unwrap()).unwrap()
    };
    assert_eq!(body.mapped_count, expected);
    assert_eq!(body.mapped.len(), expected);
}

fn append_legacy(output: &mut Vec<u8>, legacy: &fixture::ExpectedLegacy) {
    assert_eq!(legacy.prefix.len(), 16);
    append_words(output, &legacy.prefix);
    for array in &legacy.arrays {
        output.extend(i32::try_from(array.len()).unwrap().to_le_bytes());
        append_words(output, array);
    }
    let product = legacy.arrays.iter().map(Vec::len).product::<usize>() * 3;
    assert_eq!(legacy.tripled.len(), product);
    append_words(output, &legacy.tripled);
}

fn append_words(output: &mut Vec<u8>, words: &[u64]) {
    for bits in words {
        assert_eq!(f64::from_bits(*bits).to_bits(), *bits);
        output.extend(bits.to_le_bytes());
    }
}

fn check_carrier(case: &fixture::MappedCase) {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    wire.extend(144_u32.to_le_bytes());
    wire.extend((case.payload.len() as u64).to_le_bytes());
    wire.extend(&case.payload);
    let limits = FlowLimits {
        maximum_wire_bytes: 20_000,
        maximum_records: 1,
        maximum_payload_bytes: 20_000,
    };
    let carrier = FlowCarrier::from_bytes(&wire, limits).unwrap();
    assert_eq!(carrier.latest(144).unwrap().unwrap().payload, case.payload);
    assert_eq!(carrier.as_bytes(), wire);
}

#[test]
fn native_empty_and_source_synthetic_contexts_have_separate_provenance_and_products() {
    let cases = fixture::cases();
    assert_eq!(cases[0].payload.len(), 8);
    assert!(cases[0].objects.is_empty());
    assert_eq!(cases[1].payload.len(), 346);
    assert_eq!(cases[2].payload.len(), 126);
    let body = cases[2].objects[0].body.as_ref().unwrap();
    assert_eq!(body.pairs.len(), 2);
    assert_eq!(body.matched, Some(1));
    assert_eq!(body.factor, 2);
    assert_eq!(body.mapped_count, 2);
    assert_eq!(body.mapped, [0x8000000000000000, 0x7ff8000000001234]);
    assert!(body.legacy.is_none());
}
