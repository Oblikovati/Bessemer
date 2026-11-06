use super::*;
fn payload(bytes: &[u8]) -> Vec<u8> {
    let mut raw = (bytes.len() as i32).to_le_bytes().to_vec();
    raw.extend_from_slice(bytes);
    raw
}
#[test]
fn every_compact_prefix_extension_and_invalid_count_fails_with_typed_diagnostics() {
    for count in 0..=32 {
        let raw = payload(&vec![128; count]);
        for end in 0..raw.len() {
            assert!(open(&raw[..end], options()).is_err());
        }
        let mut extended = raw.clone();
        extended.push(0);
        assert!(open(&extended, options()).is_err());
        for count in [-1i32, i32::MIN, i32::MAX] {
            check_bad_count(&raw, count);
        }
    }
}
fn check_bad_count(raw: &[u8], count: i32) {
    let mut invalid = raw.to_vec();
    invalid[..4].copy_from_slice(&count.to_le_bytes());
    let error = open(&invalid, options()).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(error.cause.contains(&count.to_string()));
    assert!(error.cause.contains("expected"));
}
#[test]
fn every_owned_payload_obeys_independent_exact_and_one_below_resource_bounds() {
    for row in cases() {
        let exact = FlowCountedByteOptions {
            maximum_payload_bytes: row.raw.len(),
            maximum_values: row.values.len(),
        };
        assert!(open(&row.raw, exact).is_ok());
        let mut reduced = exact;
        reduced.maximum_payload_bytes -= 1;
        assert!(open(&row.raw, reduced).is_err());
        if exact.maximum_values == 0 {
            continue;
        }
        reduced = exact;
        reduced.maximum_values -= 1;
        assert!(open(&row.raw, reduced).is_err());
    }
}
#[test]
fn foreign_stale_wrong_occurrences_and_missing_indices_leave_the_original_intact() {
    let values = open(&payload(&[0, 127, 128, 255]), options()).unwrap();
    let source = values.source();
    assert!(values.replace_byte(4, 0).is_err() && values.replace_byte(usize::MAX, 0).is_err());
    let wrong = source.records().nth(2).unwrap().id;
    assert!(source.record_12_counted_bytes(&wrong, options()).is_err());
    check_stale_and_foreign(&values);
    check_external_replacement(&values);
}
fn check_external_replacement(values: &FlowCountedBytes) {
    let source = values.source();
    let id = values.record();
    let external = source.replace_payload(id, &payload(&[])).unwrap();
    assert_eq!(
        external
            .result()
            .record_12_counted_bytes(&external.result().records().nth(1).unwrap().id, options())
            .unwrap()
            .stored_count(),
        0
    );
    assert_eq!(values.raw_bytes(), &[0, 127, 128, 255]);
}
#[test]
fn zero_count_is_complete_and_isolated_after_caller_mutation_and_failed_edits() {
    let mut raw = payload(&[]);
    let values = open(
        &raw,
        FlowCountedByteOptions {
            maximum_payload_bytes: 4,
            maximum_values: 0,
        },
    )
    .unwrap();
    raw.fill(255);
    drop(raw);
    assert!(values.raw_bytes().is_empty());
    assert_eq!(values.payload_interval(), (4, 4));
    assert_eq!(values.coverage().raw_bytes, 0);
    assert!(values.replace_byte(0, 1).is_err());
    assert_eq!(values.decoded_bytes(), &[0; 4]);
}
#[test]
fn nonempty_raw_values_survive_caller_buffer_mutation_and_drop() {
    let mut raw = payload(&[0, 255, 128, 1]);
    let values = open(&raw, options()).unwrap();
    raw.fill(77);
    drop(raw);
    assert_eq!(values.raw_bytes(), &[0, 255, 128, 1]);
    let edit = values.replace_byte(1, 0).unwrap();
    assert_eq!(edit.result().raw_bytes(), &[0, 0, 128, 1]);
    assert_eq!(values.raw_bytes(), &[0, 255, 128, 1]);
}
fn assert_shared<T: Send + Sync>() {}
#[test]
fn public_counted_byte_and_reused_result_types_are_send_sync() {
    assert_shared::<FlowCountedByteOptions>();
    assert_shared::<FlowCountedBytes>();
    assert_shared::<FlowSimpleTableEdit<FlowCountedBytes>>();
    assert_shared::<FlowSimpleTableCoverage>();
    assert_shared::<FlowSimpleTableDiagnostic>();
}
#[test]
fn eight_workers_replay_distinct_byte_edits_against_serial_results() {
    let values = open(&payload(&(0..=255).collect::<Vec<u8>>()), options()).unwrap();
    let serial: Vec<_> = (0..8).map(|worker| worker_edits(&values, worker)).collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|worker| {
                let values = &values;
                scope.spawn(move || worker_edits(values, worker))
            })
            .collect();
        for (worker, expected) in workers.into_iter().zip(serial) {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(values.raw_bytes(), (0..=255).collect::<Vec<u8>>());
}
fn worker_edits(values: &FlowCountedBytes, worker: usize) -> Vec<Vec<u8>> {
    (0..32)
        .map(|step| {
            let index = worker * 32 + step;
            let edit = values.replace_byte(index, !(index as u8)).unwrap();
            let mut expected = values.decoded_bytes().to_vec();
            expected[index + 4] = !(index as u8);
            assert_eq!(edit.result().decoded_bytes(), expected);
            assert!(values.replace_byte(usize::MAX, 0).is_err());
            state_carrier::check_history(edit.carrier_edit());
            edit.result().source().as_bytes().to_vec()
        })
        .collect()
}

fn check_stale_and_foreign(values: &FlowCountedBytes) {
    let source = values.source();
    let id = values.record();
    let changed = values.replace_byte(0, 255).unwrap();
    assert!(
        changed
            .result()
            .source()
            .record_12_counted_bytes(id, options())
            .is_err()
    );
    let foreign = open(&payload(&[1]), options()).unwrap();
    assert!(
        source
            .record_12_counted_bytes(foreign.record(), options())
            .is_err()
    );
}
