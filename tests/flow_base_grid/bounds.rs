use super::*;
use bessemer::FailureClass;
use std::io::Write;

fn read_with(payload: &[u8], options: FlowGridOptions) -> bessemer::Result<FlowBaseGrid> {
    let source = carrier(payload, 100_000);
    source.record_101_grid(&source.latest(101).unwrap().unwrap().id, options)
}

pub fn check_prefixes_and_limits(stored: &[u8], compressed: bool, decoded: usize) {
    for end in 0..stored.len() {
        assert!(read_with(&stored[..end], options(compressed)).is_err());
    }
    let mut extra = stored.to_vec();
    extra.push(0);
    assert!(read_with(&extra, options(compressed)).is_err());
    let mut exact = options(compressed);
    exact.maximum_decoded_bytes = decoded;
    exact.maximum_boundary_words = (decoded - 12) / 8;
    assert!(read_with(stored, exact).is_ok());
    let mut short = exact;
    short.maximum_decoded_bytes -= 1;
    assert!(read_with(stored, short).is_err());
    short = exact;
    short.maximum_boundary_words -= 1;
    assert!(read_with(stored, short).is_err());
}

fn small_grid() -> Vec<u8> {
    let mut raw = vec![0; 36];
    raw[12..20].copy_from_slice(&0x8000000000000000_u64.to_le_bytes());
    raw
}

#[test]
fn invalid_dimensions_and_options_fail_before_dimension_allocations() {
    for dimensions in [[-1, 0, 0], [0, i32::MIN, 1], [i32::MAX; 3]] {
        let raw: Vec<u8> = dimensions.into_iter().flat_map(i32::to_le_bytes).collect();
        let error = read_with(&raw, options(false)).unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert_eq!(error.operation, "Flow base grid");
    }
    for (bytes, words) in [(0, 3), (35, 3), (36, 0), (36, 2)] {
        let mut limits = options(false);
        limits.maximum_decoded_bytes = bytes;
        limits.maximum_boundary_words = words;
        assert!(read_with(&small_grid(), limits).is_err());
    }
}

fn compressed(raw: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}

#[test]
fn checksum_extra_stream_and_wrong_wrapper_are_rejected() {
    let wire = compressed(&small_grid());
    let mut bad_checksum = wire.clone();
    *bad_checksum.last_mut().unwrap() ^= 1;
    let mut concatenated = wire.clone();
    concatenated.extend_from_slice(&wire);
    for malformed in [
        bad_checksum,
        concatenated,
        Vec::new(),
        vec![0x1f, 0x8b, 8, 0],
    ] {
        assert!(read_with(&malformed, options(true)).is_err());
    }
}

#[test]
fn record_identity_and_edit_indices_are_checked() {
    let raw = small_grid();
    let first = carrier(&raw, 100_000);
    let other = carrier(&raw, 100_000);
    let id = first.latest(101).unwrap().unwrap().id;
    assert!(other.record_101_grid(&id, options(false)).is_err());
    let wrong = first.latest(999).unwrap().unwrap().id;
    assert!(first.record_101_grid(&wrong, options(false)).is_err());
    let grid = first.record_101_grid(&id, options(false)).unwrap();
    for (array, index) in [(3, 0), (usize::MAX, 0), (0, 1), (2, usize::MAX)] {
        assert!(grid.replace_boundary_word(array, index, 0).is_err());
    }
    let edit = grid.replace_boundary_word(0, 0, 7).unwrap();
    assert!(
        edit.result()
            .source()
            .record_101_grid(&id, options(false))
            .is_err()
    );
    assert_eq!(grid.decoded_bytes(), raw);
}

#[test]
fn source_mutation_and_eight_workers_cannot_change_published_grid() {
    let mut raw = small_grid();
    let grid = parse(&raw, false);
    raw.fill(255);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|worker| {
                let source = &grid;
                scope.spawn(move || {
                    check_worker_edit(source, worker + 1);
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert_eq!(grid.decoded_bytes(), small_grid());
}

fn check_worker_edit(source: &FlowBaseGrid, bits: u64) {
    let edit = source.replace_boundary_word(1, 0, bits).unwrap();
    assert_eq!(edit.result().boundary(1).unwrap().raw_bits(0), Some(bits));
    assert_eq!(source.decoded_bytes(), small_grid());
    let reversed = edit.result().replace_boundary_word(1, 0, 0).unwrap();
    assert_eq!(reversed.result().decoded_bytes(), small_grid());
}

#[test]
fn measured_zlib_growth_obeys_original_physical_payload_limit() {
    let mut raw = vec![0; 12 + 8 * 1026];
    raw[..4].copy_from_slice(&1023_i32.to_le_bytes());
    let stored = compressed(&raw);
    let grid = parse(&stored, true);
    let grown = grid
        .replace_boundary_word(0, 0, 0x1234567812345678)
        .unwrap();
    assert!(grown.result().coverage().stored_bytes > stored.len());
    let limited = carrier(&stored, stored.len());
    let id = limited.latest(101).unwrap().unwrap().id;
    let selected = limited.record_101_grid(&id, options(true)).unwrap();
    assert!(
        selected
            .replace_boundary_word(0, 0, 0x1234567812345678)
            .is_err()
    );
    assert_eq!(selected.decoded_bytes(), raw);
    assert_eq!(selected.source().as_bytes(), limited.as_bytes());
}
