use super::{fixture, types::*};
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/expected.rs"]
mod expected;
#[path = "api/read.rs"]
mod read;
#[path = "api/replay.rs"]
mod replay;

fn options() -> FlowPrimaryTableOptions {
    FlowPrimaryTableOptions {
        profile: FlowPrimaryTableProfile::PrimaryClass,
        maximum_decoded_bytes: 16_000_000,
        maximum_items: 100_000,
        maximum_elements: 100_000,
        maximum_rows: 100_000,
        maximum_wide_blocks: 100_000,
        maximum_scopes: 1_000_000,
        maximum_fields: 1_000_000,
        maximum_values: 2_000_000,
    }
}

fn carrier(payload: &[u8]) -> FlowCarrier {
    let mut wire = 8u16.to_le_bytes().to_vec();
    for (id, bytes) in [(439, &b"decoy"[..]), (439, payload), (999, &b"opaque"[..])] {
        append_record(&mut wire, id, bytes);
    }
    FlowCarrier::from_bytes(
        &wire,
        FlowLimits {
            maximum_wire_bytes: 20_000_000,
            maximum_payload_bytes: 16_000_000,
            maximum_records: 3,
        },
    )
    .unwrap()
}

fn append_record(wire: &mut Vec<u8>, identifier: u32, payload: &[u8]) {
    wire.extend_from_slice(&identifier.to_le_bytes());
    wire.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    wire.extend_from_slice(payload);
}

fn open(payload: &[u8], options: FlowPrimaryTableOptions) -> bessemer::Result<FlowPrimaryTable> {
    let source = carrier(payload);
    let id = source.latest(439).unwrap().unwrap().id;
    source.record_439_primary_table(&id, options)
}
