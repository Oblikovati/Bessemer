use super::{fixture, types::*};
use bessemer::flow::*;
#[path = "prefix_api/contracts.rs"]
mod contracts;
#[path = "prefix_api/edit.rs"]
mod edit;
#[path = "prefix_api/read.rs"]
mod read;
fn options() -> FlowMetadataPrefixOptions {
    FlowMetadataPrefixOptions {
        string_mode: FlowMetadataStringMode::NativeUtf16,
        maximum_payload_bytes: 1_000_000,
        maximum_vectors: 1024,
        maximum_objects: 1024,
        maximum_values: 100_000,
        maximum_string_units: 100_000,
    }
}
fn carrier(raw: &[u8], identifier: u32) -> FlowCarrier {
    let mut bytes = 8u16.to_le_bytes().to_vec();
    for (id, payload) in [
        (identifier, &b"decoy"[..]),
        (identifier, raw),
        (999, &b"opaque"[..]),
    ] {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(payload);
    }
    FlowCarrier::from_bytes(
        &bytes,
        FlowLimits {
            maximum_wire_bytes: 2_000_000,
            maximum_payload_bytes: 1_000_000,
            maximum_records: 3,
        },
    )
    .unwrap()
}
fn open(raw: &[u8], options: FlowMetadataPrefixOptions) -> bessemer::Result<FlowMetadataPrefix> {
    let source = carrier(raw, 424);
    source.record_424_metadata_prefix(&source.latest(424).unwrap().unwrap().id, options)
}
fn roles() -> [FlowMetadataStringRole; 4] {
    [
        FlowMetadataStringRole::First,
        FlowMetadataStringRole::Second,
        FlowMetadataStringRole::Third,
        FlowMetadataStringRole::Fourth,
    ]
}
