//! Bounded iterative forest reads, exact raw edits and explicit zlib reversal scope.
#[path = "flow_block_trees/bounds.rs"]
mod bounds;
#[path = "flow_block_trees/checks.rs"]
mod checks;
#[path = "flow_block_trees/edits.rs"]
mod edits;
#[path = "flow_block_trees/fixture.rs"]
mod fixture;
use bessemer::flow::*;

fn options(case: &fixture::TreeCase) -> FlowTreeOptions {
    FlowTreeOptions {
        compression: if case.compressed {
            FlowTreeCompression::Zlib
        } else {
            FlowTreeCompression::Raw
        },
        legacy_bounds: case.legacy_bounds.map(FlowTreeBounds::from_pairs),
        maximum_decoded_bytes: 100_000,
        maximum_roots: 10_000,
        maximum_nodes: 20_000,
        maximum_depth: case.maximum_depth,
    }
}

fn limits() -> FlowLimits {
    FlowLimits {
        maximum_wire_bytes: 200_000,
        maximum_records: 100,
        maximum_payload_bytes: 100_000,
    }
}

fn append_record(wire: &mut Vec<u8>, identifier: u32, payload: &[u8]) {
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
}

fn wire(stored: &[u8]) -> Vec<u8> {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    append_record(&mut wire, 999, b"before");
    append_record(&mut wire, 400, stored);
    append_record(&mut wire, 111, b"after");
    wire
}

fn source(stored: &[u8]) -> FlowCarrier {
    FlowCarrier::from_bytes(&wire(stored), limits()).unwrap()
}

fn read(source: &FlowCarrier, options: FlowTreeOptions) -> bessemer::Result<FlowBlockTrees> {
    let id = source.latest(400)?.unwrap().id;
    source.record_400_trees(&id, options)
}

fn trees(case: &fixture::TreeCase) -> FlowBlockTrees {
    read(&source(&case.stored), options(case)).unwrap()
}
