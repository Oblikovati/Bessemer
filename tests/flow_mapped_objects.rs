//! Explicit-context mapped bodies, bounded products and isolated fixed-width wire edits.
#[path = "flow_mapped_objects/bounds.rs"]
mod bounds;
#[path = "flow_mapped_objects/checks.rs"]
mod checks;
#[path = "flow_mapped_objects/edits.rs"]
mod edits;
#[path = "flow_mapped_objects/fixture.rs"]
mod fixture;
use bessemer::flow::*;
use std::sync::Arc;

fn limits() -> FlowMappedLimits {
    FlowMappedLimits {
        maximum_entries: 100,
        maximum_pairs: 1000,
        maximum_numeric_words: 10_000,
    }
}

fn wire_limits() -> FlowLimits {
    FlowLimits {
        maximum_wire_bytes: 20_000,
        maximum_records: 100,
        maximum_payload_bytes: 10_000,
    }
}

fn context(values: &Option<Vec<Option<i32>>>) -> FlowMappedContext {
    let Some(values) = values else {
        return FlowMappedContext::Unavailable;
    };
    let counts: Vec<Option<FlowMatchedObjectCount>> = values
        .iter()
        .map(|value| {
            value.map(|value| {
                FlowMatchedObjectCount::try_from(u32::try_from(value).unwrap()).unwrap()
            })
        })
        .collect();
    FlowMappedContext::Aligned(Arc::from(counts))
}

fn append_record(wire: &mut Vec<u8>, identifier: u32, payload: &[u8]) {
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
}

fn wire(payload: &[u8]) -> Vec<u8> {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    append_record(&mut wire, 999, b"before");
    append_record(&mut wire, 144, payload);
    append_record(&mut wire, 111, b"after");
    wire
}

fn source(payload: &[u8]) -> FlowCarrier {
    FlowCarrier::from_bytes(&wire(payload), wire_limits()).unwrap()
}

fn list(case: &fixture::MappedCase) -> FlowMappedObjects {
    source(&case.payload)
        .mapped_objects(limits(), context(&case.contexts))
        .unwrap()
        .unwrap()
}
