//! Complete legacy fields and isolated edits checked against independently frozen Source views.
#[path = "flow_block_data/bounds.rs"]
mod bounds;
#[path = "flow_block_data/checks.rs"]
mod checks;
#[path = "flow_block_data/context.rs"]
mod context;
#[path = "flow_block_data/contracts.rs"]
mod contracts;
#[path = "flow_block_data/edits.rs"]
mod edits;
#[path = "flow_block_data/fixture.rs"]
mod fixture;
use bessemer::flow::*;

fn options(case: &fixture::BlockCase) -> FlowBlockDataOptions {
    FlowBlockDataOptions {
        compression: if case.compressed {
            FlowBlockDataCompression::Zlib
        } else {
            FlowBlockDataCompression::Raw
        },
        leaf_count: FlowLeafCount::try_from(case.leaf_count).unwrap(),
        maximum_decoded_bytes: 500_000,
        maximum_blocks: 10_000,
        maximum_entries: 10_000,
        maximum_integer_words: 100_000,
    }
}

fn carrier(payload: &[u8], maximum: usize) -> FlowCarrier {
    let mut raw = 8_u16.to_le_bytes().to_vec();
    for (identifier, bytes) in [
        (406_u32, &b"unselected"[..]),
        (406, payload),
        (999, &b"opaque\0"[..]),
    ] {
        raw.extend_from_slice(&identifier.to_le_bytes());
        raw.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        raw.extend_from_slice(bytes);
    }
    FlowCarrier::from_bytes(
        &raw,
        FlowLimits {
            maximum_wire_bytes: maximum + 100,
            maximum_payload_bytes: maximum,
            maximum_records: 3,
        },
    )
    .unwrap()
}

fn parse_case(case: &fixture::BlockCase) -> FlowLegacyBlockData {
    let source = carrier(&case.stored, 500_000);
    source
        .record_406_blocks(&source.latest(406).unwrap().unwrap().id, options(case))
        .unwrap()
}
