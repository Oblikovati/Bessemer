use super::*;
use std::sync::Arc;

#[test]
fn every_object_field_supports_exact_noops_changes_variable_text_and_fresh_reversals() {
    let mut requests = 0;
    for case in fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 143)
    {
        let parsed = list(&case.payload);
        for (index, object) in case.objects.iter().enumerate() {
            for request in object_requests(object) {
                check_request(&parsed, &case, index, object, request);
                requests += 1;
            }
        }
    }
    assert_eq!(requests, 1044);
}

fn object_requests(object: &fixture::ExpectedObject) -> Vec<FlowFeatureFieldEdit> {
    let raw = text_units(&object.text);
    vec![
        FlowFeatureFieldEdit::HeaderVersion(object.words[2]),
        FlowFeatureFieldEdit::TypeCode(object.words[3]),
        FlowFeatureFieldEdit::BodyVersion(object.words[4]),
        FlowFeatureFieldEdit::State(object.state),
        FlowFeatureFieldEdit::Text(Arc::from(raw)),
        FlowFeatureFieldEdit::HeaderVersion(object.words[2] ^ 0x80000001),
        FlowFeatureFieldEdit::TypeCode(object.words[3] ^ 0x40000000),
        FlowFeatureFieldEdit::BodyVersion(object.words[4] ^ 0x80000001),
        FlowFeatureFieldEdit::State(object.state ^ 255),
        FlowFeatureFieldEdit::Text(Arc::from([])),
        FlowFeatureFieldEdit::Text(Arc::from([0])),
        FlowFeatureFieldEdit::Text(Arc::from([0xd800, 0, 0xdc00, 0])),
    ]
}

fn text_units(bytes: &[u8]) -> Vec<u16> {
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|bytes| u16::from_le_bytes(*bytes))
        .collect()
}

fn expected_payload(
    case: &fixture::FeatureCase,
    object: &fixture::ExpectedObject,
    request: &FlowFeatureFieldEdit,
) -> Vec<u8> {
    let (offset, end) = request_span(object, request);
    let bytes = request_bytes(request);
    [&case.payload[..offset], &bytes, &case.payload[end..]].concat()
}

fn text_bytes(units: &[u16]) -> Vec<u8> {
    let mut bytes = i32::try_from(units.len()).unwrap().to_le_bytes().to_vec();
    bytes.extend(units.iter().flat_map(|unit| unit.to_le_bytes()));
    bytes
}

fn check_request(
    parsed: &FlowFeatureIdentities,
    case: &fixture::FeatureCase,
    index: usize,
    object: &fixture::ExpectedObject,
    request: FlowFeatureFieldEdit,
) {
    let expected = expected_payload(case, object, &request);
    let reverse_request = original_request(object, &request);
    let edited = parsed.replace_object_field(index, request).unwrap();
    assert_eq!(edited.result().payload(), expected);
    assert_eq!(edited.result().source().as_bytes(), wire(&expected));
    assert_eq!(edited.source().payload(), case.payload);
    assert_eq!(parsed.payload(), case.payload);
    assert_eq!(edited.result().coverage().achieved_byte_error, 0);
    check_history(edited.carrier_edit(), expected != case.payload);
    let reversed = edited
        .result()
        .replace_object_field(index, reverse_request)
        .unwrap();
    assert_eq!(
        reversed.result().source().as_bytes(),
        parsed.source().as_bytes()
    );
}

fn original_request(
    object: &fixture::ExpectedObject,
    request: &FlowFeatureFieldEdit,
) -> FlowFeatureFieldEdit {
    match request {
        FlowFeatureFieldEdit::HeaderVersion(_) => {
            FlowFeatureFieldEdit::HeaderVersion(object.words[2])
        }
        FlowFeatureFieldEdit::TypeCode(_) => FlowFeatureFieldEdit::TypeCode(object.words[3]),
        FlowFeatureFieldEdit::BodyVersion(_) => FlowFeatureFieldEdit::BodyVersion(object.words[4]),
        FlowFeatureFieldEdit::State(_) => FlowFeatureFieldEdit::State(object.state),
        FlowFeatureFieldEdit::Text(_) => {
            FlowFeatureFieldEdit::Text(Arc::from(text_units(&object.text)))
        }
    }
}

fn check_history(edited: &FlowCarrierEdit, changed: bool) {
    assert_eq!(edited.history().len(), 3);
    for ((relation, source), result) in edited
        .history()
        .iter()
        .zip(edited.source().records())
        .zip(edited.result().records())
    {
        assert_eq!(relation.source, source.id);
        assert_eq!(relation.result, result.id);
        let expected = if changed && source.identifier == 143 {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        };
        assert_eq!(relation.relation, expected);
    }
}

#[test]
fn all_18_stored_tail_values_support_noops_exact_patches_and_reversals() {
    let mut checked = 0;
    for case in fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 143)
    {
        let Some(tail) = &case.tail else {
            continue;
        };
        let parsed = list(&case.payload);
        for (index, before) in tail.iter().enumerate() {
            check_tail_edit(&parsed, &case.payload, tail.len(), index, *before);
            checked += 1;
        }
    }
    assert_eq!(checked, 18);
}

fn check_tail_edit(
    parsed: &FlowFeatureIdentities,
    original: &[u8],
    count: usize,
    index: usize,
    before: i32,
) {
    let noop = parsed.replace_tail_value(index, before).unwrap();
    assert_eq!(
        noop.result().source().as_bytes(),
        parsed.source().as_bytes()
    );
    let after = before ^ 0x40000000;
    let edited = parsed.replace_tail_value(index, after).unwrap();
    let offset = original.len() - count * 4 + index * 4;
    let mut expected = original.to_vec();
    expected[offset..offset + 4].copy_from_slice(&after.to_le_bytes());
    assert_eq!(edited.result().source().as_bytes(), wire(&expected));
    check_history(edited.carrier_edit(), true);
    let reversed = edited.result().replace_tail_value(index, before).unwrap();
    assert_eq!(
        reversed.result().source().as_bytes(),
        parsed.source().as_bytes()
    );
}

fn request_span(
    object: &fixture::ExpectedObject,
    request: &FlowFeatureFieldEdit,
) -> (usize, usize) {
    let start = object.words[0] as usize;
    let end = object.words[1] as usize;
    match request {
        FlowFeatureFieldEdit::HeaderVersion(_) => (start, start + 4),
        FlowFeatureFieldEdit::TypeCode(_) => (start + 4, start + 8),
        FlowFeatureFieldEdit::BodyVersion(_) => (start + 8, start + 12),
        FlowFeatureFieldEdit::State(_) => (end - 1, end),
        FlowFeatureFieldEdit::Text(_) => (start + 12, end - 1),
    }
}

fn request_bytes(request: &FlowFeatureFieldEdit) -> Vec<u8> {
    match request {
        FlowFeatureFieldEdit::HeaderVersion(value)
        | FlowFeatureFieldEdit::TypeCode(value)
        | FlowFeatureFieldEdit::BodyVersion(value) => value.to_le_bytes().to_vec(),
        FlowFeatureFieldEdit::State(value) => vec![*value],
        FlowFeatureFieldEdit::Text(units) => text_bytes(units),
    }
}
