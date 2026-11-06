use super::*;
use bessemer::FailureClass;
use std::io::Write;

fn raw_options(case: &fixture::TreeCase) -> FlowTreeOptions {
    let mut configured = options(case);
    configured.compression = FlowTreeCompression::Raw;
    configured
}

#[test]
fn every_small_owned_prefix_and_extension_rejects_without_partial_trees() {
    for case in fixture::cases()
        .into_iter()
        .skip(1)
        .filter(|case| case.decoded.len() < 200)
    {
        for end in 0..case.decoded.len() {
            assert!(read(&source(&case.decoded[..end]), raw_options(&case)).is_err());
        }
        let mut raw = case.decoded.clone();
        raw.push(0);
        assert!(read(&source(&raw), raw_options(&case)).is_err());
    }
}

#[test]
fn legacy_bounds_active_delegation_and_negative_versions_have_explicit_failures() {
    let cases = fixture::cases();
    let legacy = cases.iter().find(|case| case.version == 0).unwrap();
    let mut configured = options(legacy);
    configured.legacy_bounds = None;
    assert_eq!(
        read(&source(&legacy.stored), configured).unwrap_err().class,
        FailureClass::Missing
    );
    reject_unused_bounds(&cases[0]);
    let inactive = cases
        .iter()
        .find(|case| case.version == 4 && !case.compressed)
        .unwrap();
    let mut raw = inactive.decoded.clone();
    *raw.last_mut().unwrap() = 1;
    assert_eq!(
        read(&source(&raw), options(inactive)).unwrap_err().class,
        FailureClass::Unsupported
    );
    assert!(read(&source(&(-1_i32).to_le_bytes()), raw_options(legacy)).is_err());
}

#[test]
fn decoded_root_node_and_depth_budgets_are_independent_and_inclusive() {
    let case = fixture::cases().remove(0);
    let carrier = source(&case.stored);
    let parsed = trees(&case);
    let exact = exact_options(&parsed);
    assert!(read(&carrier, exact).is_ok());
    for configured in below_options(exact) {
        assert!(read(&carrier, configured).is_err());
    }
    for configured in invalid_options(exact) {
        assert!(read(&carrier, configured).is_err());
    }
    let inactive = fixture::cases()
        .into_iter()
        .find(|case| case.grid_count == 1_u128 << 96)
        .unwrap();
    let mut tiny = options(&inactive);
    tiny.maximum_roots = 1;
    tiny.maximum_nodes = 1;
    tiny.maximum_depth = 0;
    assert!(read(&source(&inactive.stored), tiny).is_ok());
}

fn exact_options(parsed: &FlowBlockTrees) -> FlowTreeOptions {
    let coverage = parsed.coverage();
    let mut configured = parsed.options();
    configured.maximum_decoded_bytes = coverage.decoded_bytes;
    configured.maximum_roots = coverage.checked_roots;
    configured.maximum_nodes = coverage.checked_nodes;
    configured.maximum_depth = coverage.maximum_wire_depth;
    configured
}

fn below_options(exact: FlowTreeOptions) -> [FlowTreeOptions; 4] {
    let mut bytes = exact;
    bytes.maximum_decoded_bytes -= 1;
    let mut roots = exact;
    roots.maximum_roots -= 1;
    let mut nodes = exact;
    nodes.maximum_nodes -= 1;
    let mut depth = exact;
    depth.maximum_depth -= 1;
    [bytes, roots, nodes, depth]
}

fn invalid_options(exact: FlowTreeOptions) -> [FlowTreeOptions; 4] {
    let mut bytes = exact;
    bytes.maximum_decoded_bytes = 3;
    let mut roots = exact;
    roots.maximum_roots = 0;
    let mut nodes = exact;
    nodes.maximum_nodes = 0;
    let mut depth = exact;
    depth.maximum_depth = 257;
    [bytes, roots, nodes, depth]
}

#[test]
fn zlib_requires_one_complete_checksum_validated_stream_and_exact_output_bound() {
    let case = fixture::cases().remove(0);
    let mut truncated = case.stored.clone();
    truncated.pop();
    let mut trailing = case.stored.clone();
    trailing.push(0);
    let concatenated = [case.stored.as_slice(), case.stored.as_slice()].concat();
    let mut checksum = case.stored.clone();
    *checksum.last_mut().unwrap() ^= 1;
    for bytes in [truncated, trailing, concatenated, checksum, Vec::new()] {
        assert!(read(&source(&bytes), options(&case)).is_err());
    }
    let mut gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    gzip.write_all(&case.decoded).unwrap();
    assert!(read(&source(&gzip.finish().unwrap()), options(&case)).is_err());
}

#[test]
fn foreign_stale_wrong_record_and_topology_changing_requests_are_rejected() {
    let case = fixture::cases().remove(0);
    let carrier = source(&case.stored);
    let parsed = trees(&case);
    check_foreign_ids(&carrier, &case);
    check_topology_requests(&parsed);
    check_stale(&parsed);
    assert_eq!(parsed.source().as_bytes(), carrier.as_bytes());
}

fn reject_unused_bounds(case: &fixture::TreeCase) {
    let mut configured = options(case);
    configured.legacy_bounds = Some(FlowTreeBounds::from_pairs([[0, 0]; 3]));
    assert!(read(&source(&case.stored), configured).is_err());
}

fn check_foreign_ids(carrier: &FlowCarrier, case: &fixture::TreeCase) {
    let foreign = source(&case.stored).latest(400).unwrap().unwrap().id;
    assert!(carrier.record_400_trees(&foreign, options(case)).is_err());
    let wrong = carrier.records().next().unwrap().id;
    assert!(carrier.record_400_trees(&wrong, options(case)).is_err());
}

fn check_topology_requests(parsed: &FlowBlockTrees) {
    let node = parsed
        .nodes()
        .iter()
        .position(|node| node.marker_byte() != 255)
        .unwrap();
    for request in [
        FlowTreeFieldEdit::LeafMarker { node, byte: 255 },
        FlowTreeFieldEdit::LeafMarker {
            node: usize::MAX,
            byte: 1,
        },
        FlowTreeFieldEdit::RootPresence { root: 0, byte: 0 },
        FlowTreeFieldEdit::DiscardedLeafWord { node, word: 7 },
    ] {
        assert!(parsed.replace_field(request).is_err());
    }
}

fn check_stale(parsed: &FlowBlockTrees) {
    let node = parsed
        .nodes()
        .iter()
        .position(|node| node.marker_byte() == 0)
        .unwrap();
    let edit = parsed
        .replace_field(FlowTreeFieldEdit::LeafMarker { node, byte: 7 })
        .unwrap();
    assert!(
        edit.carrier_edit()
            .result()
            .record_400_trees(parsed.record(), parsed.options())
            .is_err()
    );
}

#[test]
fn omitted_prefixes_words_and_root_presence_remain_typed_missing_fields() {
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.version == 0 && case.name == "split")
        .unwrap();
    let parsed = trees(&case);
    for request in [
        FlowTreeFieldEdit::RootPresence { root: 0, byte: 1 },
        FlowTreeFieldEdit::ChildPrefix {
            node: 0,
            child: 0,
            byte: 7,
        },
        FlowTreeFieldEdit::DiscardedLeafWord { node: 1, word: 1 },
    ] {
        assert_eq!(
            parsed.replace_field(request).unwrap_err().class,
            FailureClass::Missing
        );
    }
    check_inactive_missing();
}

fn check_inactive_missing() {
    let inactive = fixture::cases()
        .into_iter()
        .find(|case| case.version == 4)
        .unwrap();
    assert_eq!(
        trees(&inactive)
            .replace_field(FlowTreeFieldEdit::RootPresence { root: 0, byte: 0 })
            .unwrap_err()
            .class,
        FailureClass::Missing
    );
}

#[test]
fn measured_compressed_growth_respects_the_original_wire_limit() {
    let stored = growth_payload();
    let carrier = source(&stored);
    let configured = options(&fixture::cases()[0]);
    let parsed = read(&carrier, configured).unwrap();
    let request = FlowTreeFieldEdit::LeafMarker { node: 0, byte: 7 };
    let grown = parsed.replace_field(request).unwrap();
    let result_bytes = grown.carrier_edit().result().as_bytes();
    assert!(result_bytes.len() > carrier.as_bytes().len());
    check_growth_rejection(&carrier, configured, request);
}

fn growth_payload() -> Vec<u8> {
    let mut decoded = 3_i32.to_le_bytes().to_vec();
    decoded.extend(
        [0_i32, 1023, 0, 0, 0, 0]
            .into_iter()
            .flat_map(i32::to_le_bytes),
    );
    for _ in 0..1024 {
        decoded.extend([1, 0]);
    }
    let mut encoded = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
    encoded.write_all(&decoded).unwrap();
    encoded.finish().unwrap()
}

fn check_growth_rejection(
    carrier: &FlowCarrier,
    configured: FlowTreeOptions,
    request: FlowTreeFieldEdit,
) {
    let mut tight = limits();
    tight.maximum_wire_bytes = carrier.as_bytes().len();
    let strict_source = FlowCarrier::from_bytes(carrier.as_bytes(), tight).unwrap();
    let strict = read(&strict_source, configured).unwrap();
    assert!(strict.replace_field(request).is_err());
    assert_eq!(strict.source().as_bytes(), carrier.as_bytes());
}
