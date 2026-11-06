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
#[path = "api/slots.rs"]
mod slots;

fn options(frame: &ReferenceFrame) -> FlowReferenceGroupOptions {
    FlowReferenceGroupOptions {
        compression: if frame.compressed {
            FlowLegacyTableTransport::Zlib
        } else {
            FlowLegacyTableTransport::Raw
        },
        first_group_mapping_enabled: frame.mapping,
        maximum_decoded_bytes: 4_000_000,
        maximum_entries: 10_000,
        maximum_reference_words: 100_000,
        maximum_integer_words: 1_000_000,
        maximum_mapping_words: 100_000,
        maximum_wide_words: 100_000,
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

fn open(raw: &[u8], options: FlowReferenceGroupOptions) -> bessemer::Result<FlowReferenceGroups> {
    let source = carrier(raw, 416, 4_000_000);
    let id = source.latest(416).unwrap().unwrap().id;
    source.record_416_references(&id, options)
}

fn read_case(case: &ReferenceCase) -> FlowReferenceGroups {
    open(&case.frame.stored, options(&case.frame)).unwrap()
}

fn assert_invalid(result: bessemer::Result<FlowReferenceGroups>) {
    let error = result.unwrap_err();
    assert_eq!(error.operation, "Flow legacy tables");
    assert!(error.entity.contains("record 416"));
    assert!(!error.cause.is_empty());
}
