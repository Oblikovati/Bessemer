//! Independent raw settings slots before introducing the typed settings body API.
use bessemer::flow::*;
use std::io::{Cursor, Read};

#[test]
fn six_reference_payloads_rebuild_every_integer_and_binary64_bit() {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("fixtures/flow-fixed-settings/slots.bin.gz")[..])
        .take(50_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FFS1");
    assert_eq!(u32::from_le_bytes(number(&mut cursor)), 6);
    for _ in 0..6 {
        check_case(&mut cursor);
    }
    assert_eq!(cursor.position(), raw.len() as u64);
}

fn check_case(cursor: &mut Cursor<&[u8]>) {
    let name_length = u32::from_le_bytes(number(cursor)) as usize;
    let mut name = vec![0; name_length];
    cursor.read_exact(&mut name).unwrap();
    assert!(std::str::from_utf8(&name).is_ok());
    let identifier = u32::from_le_bytes(number(cursor));
    let length = u32::from_le_bytes(number(cursor)) as usize;
    let count = u32::from_le_bytes(number(cursor)) as usize;
    let mut payload = vec![0; length];
    cursor.read_exact(&mut payload).unwrap();
    let bits: Vec<u64> = (0..count)
        .map(|_| u64::from_le_bytes(number(cursor)))
        .collect();
    let rebuilt = rebuild_slots(identifier, &bits);
    assert_eq!(payload, rebuilt);
    check_carrier(identifier, &payload);
    check_typed_slots(identifier, &payload, &bits);
}

fn rebuild_slots(identifier: u32, bits: &[u64]) -> Vec<u8> {
    match identifier {
        7 => {
            assert_eq!(bits.len(), 100);
            bits.iter()
                .flat_map(|value| u32::try_from(*value).unwrap().to_le_bytes())
                .collect()
        }
        102 => {
            assert_eq!(bits.len(), 320);
            bits.iter().flat_map(|value| value.to_le_bytes()).collect()
        }
        other => panic!("fixture identifier {other}; expected 7 or 102"),
    }
}

fn check_carrier(identifier: u32, payload: &[u8]) {
    let mut wire = 7_u16.to_le_bytes().to_vec();
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
    let limits = FlowLimits {
        maximum_wire_bytes: 3000,
        maximum_records: 1,
        maximum_payload_bytes: 2560,
    };
    let carrier = FlowCarrier::from_bytes(&wire, limits).unwrap();
    assert_eq!(carrier.as_bytes(), wire);
    assert_eq!(
        carrier.latest(identifier).unwrap().unwrap().payload,
        payload
    );
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut bytes = [0; N];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}

fn make_carrier(identifier: u32, payload: &[u8]) -> FlowCarrier {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
    FlowCarrier::from_bytes(
        &wire,
        FlowLimits {
            maximum_wire_bytes: 10_000,
            maximum_records: 10,
            maximum_payload_bytes: 5000,
        },
    )
    .unwrap()
}

fn kind(identifier: u32) -> FlowFixedSettingsKind {
    match identifier {
        7 => FlowFixedSettingsKind::Integer,
        102 => FlowFixedSettingsKind::Numeric,
        other => panic!("unexpected fixture {other}"),
    }
}

fn check_typed_slots(identifier: u32, payload: &[u8], bits: &[u64]) {
    let source = make_carrier(identifier, payload);
    let view = source.fixed_settings(kind(identifier)).unwrap().unwrap();
    assert_eq!(view.source().as_bytes(), source.as_bytes());
    assert_eq!(
        view.record(),
        &source.latest(identifier).unwrap().unwrap().id
    );
    assert_eq!(view.kind().identifier(), identifier);
    assert_eq!(view.coverage().checked_slots, bits.len());
    assert_eq!(view.coverage().checked_bytes, payload.len());
    assert_eq!(view.coverage().achieved_byte_error, 0);
    assert_eq!(view.diagnostics().len(), 3);
    assert_eq!(view.slot(bits.len()), None);
    for (index, expected) in bits.iter().enumerate() {
        let slot = expected_slot(identifier, *expected);
        assert_eq!(view.slot(index), Some(slot));
        check_slot_interpretation(slot);
        check_edit(&view, index, slot);
    }
}

fn expected_slot(identifier: u32, bits: u64) -> FlowFixedSlot {
    match identifier {
        7 => FlowFixedSlot::Integer(u32::try_from(bits).unwrap()),
        102 => FlowFixedSlot::Numeric(bits),
        other => panic!("unexpected fixture {other}"),
    }
}

fn check_slot_interpretation(slot: FlowFixedSlot) {
    match slot {
        FlowFixedSlot::Integer(bits) => {
            assert_eq!(slot.signed(), Some(i32::from_le_bytes(bits.to_le_bytes())));
            assert!(slot.numeric().is_none());
        }
        FlowFixedSlot::Numeric(bits) => {
            assert_eq!(slot.numeric().unwrap().to_bits(), bits);
            assert!(slot.signed().is_none());
        }
    }
}

fn check_edit(view: &FlowFixedSettings, index: usize, slot: FlowFixedSlot) {
    let changed = match slot {
        FlowFixedSlot::Integer(bits) => FlowFixedSlot::Integer(bits ^ 0x80000001),
        FlowFixedSlot::Numeric(bits) => FlowFixedSlot::Numeric(bits ^ 0x8000000000000001),
    };
    let noop = view.replace_slot(index, slot).unwrap();
    assert_eq!(noop.result().as_bytes(), view.source().as_bytes());
    let edited = view.replace_slot(index, changed).unwrap();
    assert_eq!(edited.source().as_bytes(), view.source().as_bytes());
    assert_eq!(edited.history().len(), 1);
    let reopened = edited
        .result()
        .fixed_settings(view.kind())
        .unwrap()
        .unwrap();
    assert_eq!(reopened.slot(index), Some(changed));
    check_reverse(&reopened, index, slot, view.source().as_bytes());
    check_changed_wire(view, index, changed, edited.result().as_bytes());
    assert_eq!(view.slot(index), Some(slot));
}

fn check_reverse(view: &FlowFixedSettings, index: usize, slot: FlowFixedSlot, expected: &[u8]) {
    let reverse = view.replace_slot(index, slot).unwrap();
    assert_eq!(reverse.result().as_bytes(), expected);
}

fn check_changed_wire(
    view: &FlowFixedSettings,
    index: usize,
    changed: FlowFixedSlot,
    actual: &[u8],
) {
    let bytes = match changed {
        FlowFixedSlot::Integer(bits) => bits.to_le_bytes().to_vec(),
        FlowFixedSlot::Numeric(bits) => bits.to_le_bytes().to_vec(),
    };
    let mut expected = view.source().as_bytes().to_vec();
    let start = 14 + index * bytes.len();
    expected[start..start + bytes.len()].copy_from_slice(&bytes);
    assert_eq!(actual, expected);
}

#[test]
fn missing_duplicate_malformed_and_wrong_slot_profiles_fail_without_mutation() {
    for profile in [
        FlowFixedSettingsKind::Integer,
        FlowFixedSettingsKind::Numeric,
    ] {
        check_rejections(profile);
    }
    let empty = make_carrier(999, &[]);
    assert!(
        empty
            .fixed_settings(FlowFixedSettingsKind::Integer)
            .unwrap()
            .is_none()
    );
}

fn check_rejections(profile: FlowFixedSettingsKind) {
    let size = match profile {
        FlowFixedSettingsKind::Integer => 400,
        FlowFixedSettingsKind::Numeric => 2560,
    };
    for length in (0..size).chain([size + 1]) {
        let malformed = make_carrier(profile.identifier(), &vec![0; length]);
        assert!(malformed.fixed_settings(profile).is_err());
    }
    let source = make_carrier(profile.identifier(), &vec![0; size]);
    let view = source.fixed_settings(profile).unwrap().unwrap();
    let wrong = match profile {
        FlowFixedSettingsKind::Integer => FlowFixedSlot::Numeric(0),
        FlowFixedSettingsKind::Numeric => FlowFixedSlot::Integer(0),
    };
    let correct = view.slot(0).unwrap();
    assert!(view.replace_slot(profile.slot_count(), correct).is_err());
    assert!(view.replace_slot(usize::MAX, correct).is_err());
    assert!(view.replace_slot(0, wrong).is_err());
    check_duplicate(&source, profile);
}

fn check_duplicate(source: &FlowCarrier, profile: FlowFixedSettingsKind) {
    let mut wire = source.as_bytes().to_vec();
    wire.extend(profile.identifier().to_le_bytes());
    wire.extend(0_u64.to_le_bytes());
    let limits = FlowLimits {
        maximum_wire_bytes: 10_000,
        maximum_records: 10,
        maximum_payload_bytes: 5000,
    };
    let duplicate = FlowCarrier::from_bytes(&wire, limits).unwrap();
    assert!(duplicate.fixed_settings(profile).is_err());
    assert_eq!(duplicate.as_bytes(), wire);
}

#[test]
fn published_views_and_edits_are_send_sync_and_isolated_across_workers() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<FlowFixedSettings>();
    assert_traits::<FlowFixedSettingsKind>();
    assert_traits::<FlowFixedSlot>();
    assert_traits::<FlowFixedSettingsCoverage>();
    assert_traits::<FlowFixedSettingsCheckMode>();
    assert_traits::<FlowFixedSettingsDiagnostic>();
    let source = make_carrier(102, &[0; 2560]);
    let view = source
        .fixed_settings(FlowFixedSettingsKind::Numeric)
        .unwrap()
        .unwrap();
    std::thread::scope(|scope| {
        for worker in 0..8 {
            let shared = &view;
            scope.spawn(move || check_edit(shared, worker, FlowFixedSlot::Numeric(0)));
        }
    });
    assert_eq!(view.as_bytes(), &[0; 2560]);
}
