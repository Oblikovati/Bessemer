//! Independent versioned grouped-word schemas, complete typed edits and explicit contexts.
#[path = "flow_groups/census.rs"]
mod census;
#[path = "flow_groups/checks.rs"]
mod checks;
#[path = "flow_groups/contracts.rs"]
mod contracts;
#[path = "flow_groups/fixture.rs"]
mod fixture;
use bessemer::{FailureClass, flow::*};

fn options(compressed: bool) -> FlowGroupOptions {
    FlowGroupOptions {
        compression: if compressed {
            FlowGroupCompression::Zlib
        } else {
            FlowGroupCompression::Raw
        },
        maximum_decoded_bytes: 100_000,
        maximum_groups: 10_000,
        maximum_members: 100_000,
    }
}

fn carrier(wire: &[u8]) -> FlowCarrier {
    FlowCarrier::from_bytes(
        wire,
        FlowLimits {
            maximum_wire_bytes: 2_000_000,
            maximum_records: 100,
            maximum_payload_bytes: 2_000_000,
        },
    )
    .unwrap()
}

fn view(case: &fixture::GroupCase) -> FlowGroupedRecord {
    let source = carrier(&case.wire);
    let id = source.records().nth(case.ordinal).unwrap().id;
    source
        .record_404_groups(&id, options(case.compressed))
        .unwrap()
}

#[test]
fn all_26_grouped_records_match_complete_independent_fields_and_spans() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 26);
    for case in cases {
        let record = view(&case);
        assert_eq!(record.version().wire_value(), case.version);
        assert_eq!(record.record().ordinal(), case.ordinal);
        assert_eq!(record.source().as_bytes(), case.wire);
        assert_eq!(record.decoded_bytes(), case.decoded, "{}", case.name);
        assert_eq!(record.groups().len(), case.groups.len());
        for (index, (actual, expected)) in record.groups().zip(&case.groups).enumerate() {
            checks::assert_group(&actual, expected, index);
        }
        checks::assert_coverage(&record, &case);
    }
}

#[test]
fn all_owned_fields_and_native_endpoint_fields_preserve_complete_bytes_and_history() {
    for case in fixture::cases() {
        let record = view(&case);
        for (group, expected) in case.groups.iter().enumerate() {
            if case.groups.len() > 3 && group != 0 && group + 1 != case.groups.len() {
                continue;
            }
            for request in checks::requests(expected) {
                checks::assert_edit(&record, &case, group, request);
            }
        }
        assert!(
            record
                .replace_field(case.groups.len(), FlowGroupFieldEdit::Auxiliary(0))
                .is_err()
        );
    }
}

#[test]
fn all_group_types_are_send_sync_and_isolate_caller_buffers() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FlowGroupedRecord>();
    assert_send_sync::<FlowGroupView<'static>>();
    assert_send_sync::<FlowGroupEdit>();
    assert_send_sync::<FlowGroupFieldEdit>();
    assert_send_sync::<FlowGroupCoverage>();
    assert_send_sync::<FlowGroupDiagnostic>();
    assert_send_sync::<FlowGroupEditDiagnostic>();
    let mut case = fixture::cases().remove(0);
    let record = view(&case);
    let expected = record.decoded_bytes().to_vec();
    case.wire.fill(0);
    case.decoded.fill(0);
    case.groups.clear();
    assert_eq!(record.decoded_bytes(), expected);
}

#[test]
fn eight_mixed_success_and_failure_workers_match_serial_outputs() {
    let case = fixture::cases().remove(0);
    let record = view(&case);
    let request = FlowGroupFieldEdit::WideWord {
        member: 0,
        value: 0x7ff8000000000001,
    };
    let expected = record.replace_field(0, request).unwrap();
    assert_parallel(&record, request, &expected);
    assert_eq!(record.source().as_bytes(), case.wire);
}

fn assert_parallel(
    record: &FlowGroupedRecord,
    request: FlowGroupFieldEdit,
    expected: &FlowGroupEdit,
) {
    std::thread::scope(|scope| {
        let joins: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| worker_edit(record, request)))
            .collect();
        for join in joins {
            assert_eq!(
                join.join().unwrap().result().source().as_bytes(),
                expected.result().source().as_bytes()
            );
        }
    });
}

fn worker_edit(record: &FlowGroupedRecord, request: FlowGroupFieldEdit) -> FlowGroupEdit {
    assert!(
        record
            .replace_field(record.groups().len(), request)
            .is_err()
    );
    record.replace_field(0, request).unwrap()
}
