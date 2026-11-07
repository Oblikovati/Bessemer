use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "edit.rs"]
mod edit;
#[path = "projection.rs"]
mod projection;
#[path = "replay.rs"]
mod replay;
fn options() -> FlowTextSlotOptions {
    FlowTextSlotOptions {
        string_mode: FlowMetadataStringMode::NativeUtf16,
        maximum_payload_bytes: 2_000_000,
        maximum_units: 1_000_000,
    }
}
fn open(raw: &[u8], options: FlowTextSlotOptions) -> bessemer::Result<FlowTextSlots> {
    let source = state_carrier::carrier(311, raw);
    let id = source.records().nth(1).unwrap().id;
    source.record_311_text_slots(&id, options)
}
fn units(raw: &[u8]) -> Vec<u16> {
    let (units, remainder) = raw.as_chunks::<2>();
    assert!(remainder.is_empty());
    units.iter().map(|unit| u16::from_le_bytes(*unit)).collect()
}
fn first_slot_payload(header: i32, first: &[u16]) -> Vec<u8> {
    let mut raw = header.to_le_bytes().to_vec();
    raw.extend_from_slice(&(first.len() as i32).to_le_bytes());
    for unit in first {
        raw.extend_from_slice(&unit.to_le_bytes());
    }
    raw.extend([0; 49 * 4]);
    raw
}

fn check_case(row: &TextSlotCase) -> FlowTextSlots {
    let view = open(&row.raw, options()).unwrap();
    assert_eq!(view.header(), row.header);
    assert_eq!(view.options(), options());
    assert_eq!(view.decoded_bytes(), row.raw);
    assert_eq!(view.record(), &view.source().records().nth(1).unwrap().id);
    for expected in &row.slots {
        check_slot(&view, expected);
    }
    assert!(view.slot(50).is_err() && view.slot(usize::MAX).is_err());
    check_coverage(&view, row);
    view
}
fn check_slot(view: &FlowTextSlots, expected: &TextSlot) {
    let slot = view.slot(expected.ordinal as usize).unwrap();
    assert_eq!(slot.ordinal(), expected.ordinal as usize);
    assert_eq!(
        slot.interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(slot.stored_units(), expected.units as usize);
    assert_eq!(slot.raw_bytes(), expected.raw);
    assert_eq!(slot.native_prefix_bytes(), expected.prefix);
    for (index, unit) in units(&expected.raw).into_iter().enumerate() {
        assert_eq!(slot.unit(index), Some(unit));
    }
    assert!(slot.unit(slot.stored_units()).is_none() && slot.unit(usize::MAX).is_none());
}
fn check_coverage(view: &FlowTextSlots, row: &TextSlotCase) {
    let proof = view.coverage();
    assert_eq!(proof.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(proof.payload_bytes, row.raw.len());
    assert_eq!(proof.slots, 50);
    assert_eq!(proof.achieved_error, 0);
    assert_eq!(
        proof.stored_units,
        row.slots.iter().map(|slot| slot.units as usize).sum()
    );
    assert_eq!(
        proof.empty_prefixes,
        row.slots
            .iter()
            .filter(|slot| slot.prefix.is_empty())
            .count()
    );
    check_diagnostics(view);
}
fn check_diagnostics(view: &FlowTextSlots) {
    use FlowTextSlotDiagnostic::*;
    assert_eq!(
        view.diagnostics(),
        &[
            NativeUtf16ModeAssumed,
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn every_owned_header_slot_raw_unit_native_prefix_and_span_matches_the_source() {
    for row in cases() {
        check_case(&row);
    }
}
