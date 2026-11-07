use super::{fixture, types::*};
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/paths.rs"]
mod paths;
#[path = "api/read.rs"]
mod read;
#[path = "api/replay.rs"]
mod replay;

fn options() -> FlowConditionalItemOptions {
    FlowConditionalItemOptions {
        maximum_stored_bytes: 2_000_000,
        maximum_decoded_bytes: 2_000_000,
        maximum_entries: 100_000,
        maximum_items: 100_000,
        maximum_groups: 100_000,
        maximum_subrecords: 100_000,
        maximum_vector_values: 1_000_000,
        maximum_values: 2_000_000,
        maximum_scopes: 1_000_000,
        maximum_fields: 1_000_000,
    }
}

fn carrier(identifier: u32, payload: &[u8]) -> FlowCarrier {
    let mut raw = 8u16.to_le_bytes().to_vec();
    for (id, bytes) in [
        (identifier, &b"decoy"[..]),
        (identifier, payload),
        (999, &b"opaque"[..]),
    ] {
        raw.extend_from_slice(&id.to_le_bytes());
        raw.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        raw.extend_from_slice(bytes);
    }
    FlowCarrier::from_bytes(
        &raw,
        FlowLimits {
            maximum_wire_bytes: 4_000_000,
            maximum_payload_bytes: 2_000_000,
            maximum_records: 3,
        },
    )
    .unwrap()
}

fn open(
    identifier: u32,
    payload: &[u8],
    options: FlowConditionalItemOptions,
) -> bessemer::Result<FlowConditionalItems> {
    let source = carrier(identifier, payload);
    let id = source.latest(identifier).unwrap().unwrap().id;
    match identifier {
        126 => source.record_126_items(&id, options),
        315 => source.record_315_items(&id, options),
        128 => source.record_128_items(&id, options),
        other => panic!("Expected conditional identifier126/315/128, got {other}"),
    }
}

fn item_version(items: &FlowConditionalItems, field: FlowConditionalField<'_>) -> i32 {
    let version = items
        .fields()
        .find(|candidate| {
            candidate.scope() == field.scope()
                && candidate.kind() == FlowConditionalFieldKind::Version
        })
        .unwrap()
        .value(0)
        .unwrap();
    match version {
        FlowConditionalValue::Signed32(value) => value,
        _ => panic!("Expected signed item version"),
    }
}
