//! Independent raw identities, complete isolated edits and explicit external legacy-tail context.
#[path = "flow_feature_identities/api.rs"]
mod api;
#[path = "flow_feature_identities/bounds.rs"]
mod bounds;
#[path = "flow_feature_identities/edits.rs"]
mod edits;
#[path = "flow_feature_identities/fixture.rs"]
mod fixture;
use bessemer::flow::*;

fn options() -> FlowFeatureOptions {
    FlowFeatureOptions {
        maximum_objects: 100,
        maximum_text_units: 2000,
        maximum_tail_values: 100,
        legacy_tail_count: None,
    }
}

fn limits() -> FlowLimits {
    FlowLimits {
        maximum_wire_bytes: 10_000,
        maximum_records: 100,
        maximum_payload_bytes: 5000,
    }
}

fn append_record(wire: &mut Vec<u8>, identifier: u32, payload: &[u8]) {
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
}

fn wire(payload: &[u8]) -> Vec<u8> {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    append_record(&mut wire, 999, b"before");
    append_record(&mut wire, 143, payload);
    append_record(&mut wire, 111, b"after");
    wire
}

fn carrier(payload: &[u8]) -> FlowCarrier {
    FlowCarrier::from_bytes(&wire(payload), limits()).unwrap()
}

fn list(payload: &[u8]) -> FlowFeatureIdentities {
    carrier(payload)
        .feature_identities(options())
        .unwrap()
        .unwrap()
}
