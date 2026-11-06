//! Pin raw identities and null-list framing before introducing their typed APIs.
#[path = "flow_feature_identities/fixture.rs"]
mod fixture;
use bessemer::flow::*;

#[test]
fn all_37_reference_payloads_rebuild_exactly_and_survive_complete_carrier_reads() {
    let cases = fixture::cases();
    assert_eq!(
        cases.iter().filter(|case| case.identifier == 143).count(),
        22
    );
    assert_eq!(
        cases.iter().filter(|case| case.identifier == 144).count(),
        15
    );
    for case in cases {
        let mut rebuilt = case.version.to_le_bytes().to_vec();
        rebuilt.extend(i32::try_from(case.count).unwrap().to_le_bytes());
        append_contents(&case, &mut rebuilt);
        assert_eq!(rebuilt, case.payload, "{}", case.name);
        check_carrier(&case);
    }
}

fn append_contents(case: &fixture::FeatureCase, rebuilt: &mut Vec<u8>) {
    if let Some(flags) = &case.flags {
        assert_eq!(flags.len(), case.count);
        assert!(flags.iter().all(|byte| *byte == 0));
        rebuilt.extend(flags);
        return;
    }
    for object in &case.objects {
        append_object(rebuilt, object);
    }
    if let Some(tail) = &case.tail {
        assert!(case.version >= 4);
        rebuilt.extend(i32::try_from(tail.len()).unwrap().to_le_bytes());
        for value in tail {
            rebuilt.extend(value.to_le_bytes());
        }
        return;
    }
    check_legacy(case);
}

fn append_object(rebuilt: &mut Vec<u8>, object: &fixture::ExpectedObject) {
    let [start, end, header, kind, body, count, text_start] = object.words;
    assert_eq!(rebuilt.len(), start as usize);
    rebuilt.extend(header.to_le_bytes());
    rebuilt.extend(kind.to_le_bytes());
    rebuilt.extend(body.to_le_bytes());
    rebuilt.extend(i32::try_from(count).unwrap().to_le_bytes());
    assert_eq!(rebuilt.len(), text_start as usize);
    assert_eq!(object.text.len(), count as usize * 2);
    rebuilt.extend(&object.text);
    rebuilt.push(object.state);
    assert_eq!(rebuilt.len(), end as usize);
    assert_eq!(object.boolean, object.state != 0);
    assert_eq!(object.class, native_class(kind));
    check_text(object);
}

fn check_text(object: &fixture::ExpectedObject) {
    let units: Vec<[u8; 2]> = object.text.as_chunks::<2>().0.to_vec();
    if !units.is_empty() {
        assert_eq!(units.last(), Some(&[0, 0]));
    }
    let length = units
        .iter()
        .position(|unit| *unit == [0, 0])
        .unwrap_or(units.len());
    let logical: Vec<u8> = units[..length].iter().flatten().copied().collect();
    assert_eq!(logical, object.logical);
}

fn native_class(kind: u32) -> &'static str {
    match kind {
        0 => "SBoundaryCondition",
        5 => "SVSource",
        9 => "SSSource",
        10 => "SFan",
        _ => "SUnknownFeature",
    }
}

fn check_legacy(case: &fixture::FeatureCase) {
    assert!(case.version < 4);
    assert_eq!(case.legacy.len(), 6);
    for (context, count, derived) in &case.legacy {
        assert_eq!(*derived, context.is_some());
        assert_eq!(*count, context.map_or(0, |value| value.max(0) as u32));
    }
}

fn check_carrier(case: &fixture::FeatureCase) {
    let mut wire = 7_u16.to_le_bytes().to_vec();
    wire.extend(case.identifier.to_le_bytes());
    wire.extend((case.payload.len() as u64).to_le_bytes());
    wire.extend(&case.payload);
    let limits = FlowLimits {
        maximum_wire_bytes: 10_000,
        maximum_records: 1,
        maximum_payload_bytes: 10_000,
    };
    let carrier = FlowCarrier::from_bytes(&wire, limits).unwrap();
    assert_eq!(carrier.as_bytes(), wire);
    assert_eq!(
        carrier.latest(case.identifier).unwrap().unwrap().payload,
        case.payload
    );
}

#[test]
fn the_two_native_payloads_have_three_objects_empty_tail_and_empty_optional_list() {
    let cases = fixture::cases();
    assert_eq!(cases[0].payload.len() + cases[1].payload.len(), 219);
    assert_eq!(cases[0].objects.len(), 3);
    assert_eq!(
        cases[0]
            .objects
            .iter()
            .map(|object| object.words[3])
            .collect::<Vec<_>>(),
        [39, 0, 0]
    );
    assert_eq!(cases[0].tail.as_deref(), Some([].as_slice()));
    assert_eq!(cases[1].flags.as_deref(), Some([].as_slice()));
}
