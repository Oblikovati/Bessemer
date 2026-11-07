use super::*;
use std::io::Write;
fn decoded(header: i32, values: &[u8]) -> Vec<u8> {
    let mut raw = header.to_le_bytes().to_vec();
    raw.extend_from_slice(&(values.len() as i32).to_le_bytes());
    raw.extend_from_slice(values);
    raw
}
fn compress(raw: &[u8]) -> Vec<u8> {
    let mut writer = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    writer.write_all(raw).unwrap();
    writer.finish().unwrap()
}
#[test]
fn every_compact_outer_and_inner_prefix_extension_and_checksum_failure_is_rejected() {
    let raw = decoded(i32::MIN, &[0, 127, 128, 255]);
    let stored = compress(&raw);
    check_outer_boundaries(&stored);
    for end in 0..raw.len() {
        assert!(open(&compress(&raw[..end]), options()).is_err());
    }
    let mut extended = raw.clone();
    extended.push(0);
    assert!(open(&compress(&extended), options()).is_err());
    check_checksums(&stored);
}
#[test]
fn every_negative_mismatched_and_impossible_count_keeps_a_typed_error() {
    for count in [i32::MIN, -1, 0, 1, 2, 4, i32::MAX] {
        let mut raw = decoded(7, b"abc");
        raw[4..8].copy_from_slice(&count.to_le_bytes());
        let error = open(&compress(&raw), options()).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains(&count.to_string()) && error.cause.contains("expected"));
    }
}
#[test]
fn arbitrary_headers_and_empty_arrays_accept_exact_bounds_and_zero_value_budget() {
    for header in [i32::MIN, -1, 0, 1, i32::MAX] {
        let stored = compress(&decoded(header, &[]));
        let exact = FlowByteStateOptions {
            maximum_decoded_bytes: 8,
            maximum_values: 0,
        };
        let states = open(&stored, exact).unwrap();
        assert_eq!(states.header(), header);
        assert_eq!(states.stored_count(), 0);
        assert_eq!(states.payload_interval(), (8, 8));
        assert!(states.replace_byte(0, 0).is_err());
        let reduced = FlowByteStateOptions {
            maximum_decoded_bytes: 7,
            ..exact
        };
        assert!(open(&stored, reduced).is_err());
        assert!(states.replace_header(!header).is_ok());
    }
}
#[test]
fn every_complete_case_has_independent_exact_and_one_below_inflation_and_value_limits() {
    for row in cases() {
        let exact = FlowByteStateOptions {
            maximum_decoded_bytes: row.decoded.len(),
            maximum_values: row.values.len(),
        };
        assert!(open(&row.stored, exact).is_ok());
        let mut reduced = exact;
        reduced.maximum_decoded_bytes -= 1;
        assert!(open(&row.stored, reduced).is_err());
        if exact.maximum_values == 0 {
            continue;
        }
        reduced = exact;
        reduced.maximum_values -= 1;
        assert!(open(&row.stored, reduced).is_err());
    }
}
#[test]
fn caller_mutation_external_edits_and_invalid_ids_leave_original_states_unchanged() {
    let mut stored = compress(&decoded(-7, &[0, 128, 255]));
    let states = open(&stored, options()).unwrap();
    stored.fill(99);
    drop(stored);
    assert_eq!(states.raw_bytes(), &[0, 128, 255]);
    assert!(states.replace_byte(3, 0).is_err() && states.replace_byte(usize::MAX, 0).is_err());
    check_ids(&states);
    let external = states
        .source()
        .replace_payload(states.record(), &compress(&decoded(0, &[])))
        .unwrap();
    assert_ne!(external.result().as_bytes(), states.source().as_bytes());
    assert_eq!(states.header(), -7);
    assert_eq!(states.raw_bytes(), &[0, 128, 255]);
}
fn check_ids(states: &FlowByteStates) {
    let carrier = states.source();
    let wrong = carrier.records().nth(2).unwrap().id;
    assert!(carrier.record_316_byte_states(&wrong, options()).is_err());
    let changed = states.replace_header(9).unwrap();
    assert!(
        changed
            .result()
            .source()
            .record_316_byte_states(states.record(), options())
            .is_err()
    );
    let foreign = open(&compress(&decoded(0, &[])), options()).unwrap();
    assert!(
        carrier
            .record_316_byte_states(foreign.record(), options())
            .is_err()
    );
}
fn assert_shared<T: Send + Sync>() {}
#[test]
fn both_new_and_three_reused_public_byte_state_types_are_send_and_sync() {
    assert_shared::<FlowByteStateOptions>();
    assert_shared::<FlowByteStates>();
    assert_shared::<FlowSimpleTableEdit<FlowByteStates>>();
    assert_shared::<FlowSimpleTableCoverage>();
    assert_shared::<FlowSimpleTableDiagnostic>();
}
#[test]
fn eight_workers_match_serial_compressed_bytes_for_mixed_header_and_byte_edits() {
    let states = open(
        &compress(&decoded(-7, &(0..=255).collect::<Vec<_>>())),
        options(),
    )
    .unwrap();
    let serial: Vec<_> = (0..8).map(|worker| worker_edits(&states, worker)).collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|worker| {
                let states = &states;
                scope.spawn(move || worker_edits(states, worker))
            })
            .collect();
        for (worker, expected) in workers.into_iter().zip(serial) {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(states.header(), -7);
    assert_eq!(states.raw_bytes(), (0..=255).collect::<Vec<_>>());
}
fn worker_edits(states: &FlowByteStates, worker: usize) -> Vec<Vec<u8>> {
    (0..32)
        .map(|step| {
            let edit = if step % 2 == 0 {
                changed_header(states, -((worker * 32 + step) as i32))
            } else {
                changed_byte(states, worker * 32 + step)
            };
            assert!(states.replace_byte(usize::MAX, 0).is_err());
            edit.result().source().as_bytes().to_vec()
        })
        .collect()
}

fn check_outer_boundaries(stored: &[u8]) {
    for end in 0..stored.len() {
        assert!(open(&stored[..end], options()).is_err());
    }
    for suffix in [vec![0], vec![255; 16], compress(&[])] {
        let mut extended = stored.to_vec();
        extended.extend_from_slice(&suffix);
        assert!(open(&extended, options()).is_err());
    }
}
fn check_checksums(stored: &[u8]) {
    for offset in [
        0,
        1,
        stored.len() - 4,
        stored.len() - 3,
        stored.len() - 2,
        stored.len() - 1,
    ] {
        let mut broken = stored.to_vec();
        broken[offset] ^= 1;
        assert!(open(&broken, options()).is_err());
    }
}

#[test]
fn raw_deflate_and_gzip_are_rejected_instead_of_guessed_as_zlib() {
    let raw = decoded(-1, b"abc");
    let mut deflate =
        flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    deflate.write_all(&raw).unwrap();
    assert!(open(&deflate.finish().unwrap(), options()).is_err());
    let mut gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    gzip.write_all(&raw).unwrap();
    assert!(open(&gzip.finish().unwrap(), options()).is_err());
}
