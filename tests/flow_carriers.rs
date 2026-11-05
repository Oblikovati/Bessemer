//! Independent complete native partitions and isolated ordinary physical carrier edits.
#[path = "flow_carriers/fixture.rs"]
mod fixture;
use bessemer::{FailureClass, flow::*};

#[test]
fn every_native_and_owned_record_matches_the_independent_partition() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 6);
    assert_eq!(
        cases[..3]
            .iter()
            .map(|case| case.records.len())
            .sum::<usize>(),
        66
    );
    for case in cases {
        let carrier = FlowCarrier::from_bytes(&case.raw, fixture::limits()).unwrap();
        assert_eq!(carrier.as_bytes(), case.raw);
        assert_eq!(
            carrier.version(),
            u16::from_le_bytes([case.raw[0], case.raw[1]])
        );
        assert_partition(&carrier, &case);
        assert_coverage(&carrier, &case);
    }
}

fn assert_coverage(carrier: &FlowCarrier, case: &fixture::CarrierCase) {
    let coverage = carrier.coverage();
    assert_eq!(coverage.mode, FlowCheckMode::ExactOrdinaryPhysicalCarrier);
    assert_eq!(coverage.checked_records, case.records.len());
    assert_eq!(coverage.framing_bytes, 2 + 12 * case.records.len());
    assert_eq!(
        coverage.opaque_payload_bytes,
        case.raw.len() - coverage.framing_bytes
    );
    assert_eq!(coverage.wire_bytes, case.raw.len());
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(carrier.diagnostics().len(), 3);
}

fn assert_partition(carrier: &FlowCarrier, case: &fixture::CarrierCase) {
    for (ordinal, (actual, &(identifier, offset, size))) in
        carrier.records().zip(&case.records).enumerate()
    {
        assert_eq!(actual.id.ordinal(), ordinal, "{}", case.name);
        assert_eq!(actual.identifier, identifier);
        assert_eq!(actual.header_offset, offset);
        assert_eq!(actual.payload, &case.raw[offset + 12..offset + 12 + size]);
        assert_lookup(carrier, case, identifier);
    }
    assert!(carrier.latest(0xabcdef98).unwrap().is_none());
}

fn assert_lookup(carrier: &FlowCarrier, case: &fixture::CarrierCase, identifier: u32) {
    if matches!(identifier, 0xffffff9b | 0xffffff9d) {
        assert_eq!(
            carrier.latest(identifier).unwrap_err().class,
            FailureClass::Unsupported
        );
    } else {
        let expected = case
            .records
            .iter()
            .rposition(|item| item.0 == identifier)
            .unwrap();
        assert_eq!(
            carrier.latest(identifier).unwrap().unwrap().id.ordinal(),
            expected
        );
    }
}

#[test]
fn every_record_supports_empty_changed_and_noop_edits_with_complete_history() {
    for case in fixture::cases() {
        let carrier = FlowCarrier::from_bytes(&case.raw, fixture::limits()).unwrap();
        for record in carrier.records() {
            for payload in [record.payload, &[][..], &[0, 0xff, 19, 7][..]] {
                let edit = carrier.replace_payload(&record.id, payload).unwrap();
                assert_edit(&carrier, &edit, record.id.ordinal(), payload);
                let next = edit.result().records().nth(record.id.ordinal()).unwrap();
                let reversed = edit
                    .result()
                    .replace_payload(&next.id, record.payload)
                    .unwrap();
                assert_eq!(reversed.result().as_bytes(), case.raw);
                assert_eq!(edit.source().as_bytes(), case.raw);
            }
        }
    }
}

fn assert_edit(source: &FlowCarrier, edit: &FlowCarrierEdit, selected: usize, payload: &[u8]) {
    assert_eq!(edit.history().len(), source.records().len());
    assert_untouched_wire(source, edit.result(), selected);
    for (index, (before, after)) in source.records().zip(edit.result().records()).enumerate() {
        let expected = if index == selected {
            payload
        } else {
            before.payload
        };
        assert_eq!(after.payload, expected);
        assert_eq!(after.identifier, before.identifier);
        let parent = &edit.history()[index];
        assert_eq!(parent.source, before.id);
        assert_eq!(parent.result, after.id);
        let changed = index == selected && before.payload != payload;
        assert_relation(parent, changed);
        if source.as_bytes() == edit.result().as_bytes() {
            assert_eq!(before.id, after.id);
        }
    }
    FlowCarrier::from_bytes(edit.result().as_bytes(), fixture::limits()).unwrap();
}

fn assert_untouched_wire(source: &FlowCarrier, result: &FlowCarrier, selected: usize) {
    let before = source.records().nth(selected).unwrap();
    let after = result.records().nth(selected).unwrap();
    let prefix = before.header_offset + 4;
    assert_eq!(&source.as_bytes()[..prefix], &result.as_bytes()[..prefix]);
    let before_end = before.header_offset + 12 + before.payload.len();
    let after_end = after.header_offset + 12 + after.payload.len();
    assert_eq!(
        &source.as_bytes()[before_end..],
        &result.as_bytes()[after_end..]
    );
}

#[test]
fn appended_complete_unknown_record_is_retained_instead_of_discarded_as_tail() {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    wire.extend(0xaabbccdd_u32.to_le_bytes());
    wire.extend(0_u64.to_le_bytes());
    let carrier = FlowCarrier::from_bytes(&wire, fixture::limits()).unwrap();
    assert_eq!(carrier.records().len(), 1);
    let record = carrier.records().next().unwrap();
    assert_eq!(record.identifier, 0xaabbccdd);
    assert!(record.payload.is_empty());
    assert_eq!(carrier.as_bytes(), wire);
}

fn assert_relation(parent: &FlowRecordHistory, changed: bool) {
    assert_eq!(
        parent.relation,
        if changed {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        }
    );
}

#[test]
fn unsupported_versions_and_every_partial_small_carrier_fail_without_tail_loss() {
    for version in 0_u16..=u16::MAX {
        let result = FlowCarrier::from_bytes(&version.to_le_bytes(), fixture::limits());
        assert_eq!(result.is_ok(), matches!(version, 7 | 8));
    }
    let raw = &fixture::cases()[4].raw;
    for end in 0..raw.len() {
        let valid = [2, 19, 31].contains(&end);
        assert_eq!(
            FlowCarrier::from_bytes(&raw[..end], fixture::limits()).is_ok(),
            valid
        );
    }
    assert_trailing_failures(raw);
}

fn assert_trailing_failures(raw: &[u8]) {
    for length in 1..12 {
        let mut trailing = raw.to_vec();
        trailing.extend(vec![0; length]);
        assert!(FlowCarrier::from_bytes(&trailing, fixture::limits()).is_err());
    }
    let mut oversized = 7_u16.to_le_bytes().to_vec();
    oversized.extend(1_u32.to_le_bytes());
    oversized.extend(u64::MAX.to_le_bytes());
    assert!(FlowCarrier::from_bytes(&oversized, fixture::limits()).is_err());
}

#[test]
fn exact_bounds_foreign_revisions_and_failed_growth_preserve_inputs() {
    let raw = &fixture::cases()[4].raw;
    let limits = FlowLimits {
        maximum_wire_bytes: raw.len(),
        maximum_records: 3,
        maximum_payload_bytes: 5,
    };
    let carrier = FlowCarrier::from_bytes(raw, limits).unwrap();
    assert_revision_failures(&carrier, raw, limits);
    assert_limit_failures(raw, limits);
    assert_eq!(carrier.as_bytes(), raw);
}

fn assert_revision_failures(carrier: &FlowCarrier, raw: &[u8], limits: FlowLimits) {
    let first = carrier.records().next().unwrap();
    let foreign_carrier = FlowCarrier::from_bytes(raw, limits).unwrap();
    let foreign = foreign_carrier.records().next().unwrap().id;
    assert_ne!(first.id, foreign);
    assert_eq!(
        carrier.replace_payload(&foreign, b"").unwrap_err().class,
        FailureClass::Invalid
    );
    assert!(carrier.replace_payload(&first.id, b"123456").is_err());
    assert!(
        carrier
            .replace_payload(&carrier.records().nth(1).unwrap().id, b"x")
            .is_err()
    );
    let changed = carrier.replace_payload(&first.id, b"q").unwrap();
    assert!(changed.result().replace_payload(&first.id, b"x").is_err());
    assert_eq!(carrier.as_bytes(), raw);
}

fn assert_limit_failures(raw: &[u8], exact: FlowLimits) {
    for axis in 0..3 {
        let mut smaller = exact;
        match axis {
            0 => smaller.maximum_wire_bytes -= 1,
            1 => smaller.maximum_records -= 1,
            _ => smaller.maximum_payload_bytes -= 1,
        }
        assert!(FlowCarrier::from_bytes(raw, smaller).is_err());
        match axis {
            0 => smaller.maximum_wire_bytes = 0,
            1 => smaller.maximum_records = 0,
            _ => smaller.maximum_payload_bytes = 0,
        }
        assert!(FlowCarrier::from_bytes(raw, smaller).is_err());
    }
}

#[test]
fn snapshots_isolate_caller_mutations_and_eight_concurrent_editors() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FlowCarrier>();
    assert_send_sync::<FlowCarrierEdit>();
    assert_send_sync::<FlowRecord<'static>>();
    let mut raw = fixture::cases().remove(4).raw;
    let carrier = FlowCarrier::from_bytes(&raw, fixture::limits()).unwrap();
    let original = carrier.as_bytes().to_vec();
    raw.fill(0);
    let id = carrier.records().next().unwrap().id;
    let expected = carrier.replace_payload(&id, b"parallel").unwrap();
    assert_parallel_edits(&carrier, &id, &expected);
    assert_eq!(carrier.as_bytes(), original);
}

fn assert_parallel_edits(carrier: &FlowCarrier, id: &FlowRecordId, expected: &FlowCarrierEdit) {
    std::thread::scope(|scope| {
        let joins: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| carrier.replace_payload(id, b"parallel").unwrap()))
            .collect();
        for join in joins {
            assert_eq!(
                join.join().unwrap().result().as_bytes(),
                expected.result().as_bytes()
            );
        }
    });
}
