use super::*;
use bessemer::flow::*;
#[path = "api/bounds.rs"]
mod bounds;
#[path = "api/edits.rs"]
mod edits;
#[path = "api/growth.rs"]
mod growth;
#[path = "api/read.rs"]
mod read;
use std::sync::Arc;

fn options(frame: &PointerFrame) -> FlowPointerBodyOptions {
    FlowPointerBodyOptions {
        compression: if frame.compressed {
            FlowLegacyTableTransport::Zlib
        } else {
            FlowLegacyTableTransport::Raw
        },
        leaf_count: FlowLeafCount::try_from(frame.leaf).unwrap(),
        group_counts: FlowPointerBodyCounts::try_from(frame.counts).unwrap(),
        entry_codes: frame.codes.as_deref().map(Arc::from),
        maximum_decoded_bytes: 2_000_000,
        maximum_pools: 10_000,
        maximum_entries: 30_000,
        maximum_vectors: 100_000,
        maximum_wide_words: 100_000,
        maximum_raw_bytes: 2_000_000,
    }
}

fn carrier(payload: &[u8], identifier: u32, maximum: usize) -> FlowCarrier {
    let mut raw = 8_u16.to_le_bytes().to_vec();
    for (id, bytes) in [
        (identifier, &b"decoy"[..]),
        (identifier, payload),
        (999, &b"opaque\0"[..]),
    ] {
        raw.extend_from_slice(&id.to_le_bytes());
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

fn open(payload: &[u8], options: FlowPointerBodyOptions) -> bessemer::Result<FlowPointerBodies> {
    let source = carrier(payload, 421, 2_000_000);
    let id = source.latest(421).unwrap().unwrap().id;
    source.record_421_bodies(&id, options)
}

fn read_case(case: &PointerCase) -> FlowPointerBodies {
    open(&case.frame.stored, options(&case.frame)).unwrap()
}

fn assert_invalid(result: bessemer::Result<FlowPointerBodies>) {
    let error = result.unwrap_err();
    assert_eq!(error.operation, "Flow legacy tables");
    assert!(error.entity.contains("record 421"));
    assert!(!error.cause.is_empty());
}
