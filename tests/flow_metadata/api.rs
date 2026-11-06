use super::types::*;
use super::*;
use bessemer::flow::*;
use std::collections::BTreeMap;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/read.rs"]
mod read;

fn limits() -> FlowIdStateLimits {
    FlowIdStateLimits {
        maximum_payload_bytes: 16_384,
        maximum_entries: 1024,
    }
}

fn carrier(payload: &[u8], identifier: u32) -> FlowCarrier {
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
            maximum_wire_bytes: 32_768,
            maximum_payload_bytes: 16_384,
            maximum_records: 3,
        },
    )
    .unwrap()
}

fn open(raw: &[u8], limits: FlowIdStateLimits) -> bessemer::Result<FlowIdStates> {
    let source = carrier(raw, 410);
    let id = source.latest(410).unwrap().unwrap().id;
    source.record_410_states(&id, limits)
}

fn read_case(case: &StateCase) -> FlowIdStates {
    open(&case.raw, limits()).unwrap()
}

fn map(entries: &[(i32, u8)]) -> FlowIdStateMap {
    FlowIdStateMap::from(entries.iter().copied().collect::<BTreeMap<_, _>>())
}

fn assert_invalid(result: bessemer::Result<FlowIdStates>) {
    let error = result.unwrap_err();
    assert_eq!(error.operation, "Flow legacy tables");
    assert!(error.entity.contains("record 410"));
    assert!(!error.cause.is_empty());
}
