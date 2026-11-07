//! Opaque revision identity diagnostics must stay bounded independently of carrier contents.
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
use bessemer::flow::*;
fn identity_carrier(bytes: usize) -> FlowCarrier {
    let payload = vec![0xa5; bytes];
    let carrier = state_carrier::carrier(301, &payload);
    let id = carrier.records().nth(1).unwrap().id;
    let noop = carrier.replace_payload(&id, &payload).unwrap();
    state_carrier::check_history(&noop);
    noop.result().clone()
}
#[test]
fn record_identity_debug_never_expands_large_carrier_buffers_or_span_lists() {
    for bytes in [0, 1, 65_536, 524_288] {
        let carrier = identity_carrier(bytes);
        let id = carrier.records().nth(1).unwrap().id;
        let compact = format!("{id:?}");
        let pretty = format!("{id:#?}");
        assert!(
            compact.len() < 256,
            "identity debug length {} for {bytes} payload bytes",
            compact.len()
        );
        assert!(
            pretty.len() < 512,
            "pretty identity debug length {} for {bytes} payload bytes",
            pretty.len()
        );
        assert!(!compact.contains("wire: [") && !compact.contains("spans: ["));
    }
}
#[test]
fn compact_debug_preserves_clone_identity_and_foreign_reparse_semantics() {
    let carrier = identity_carrier(4096);
    let id = carrier.records().nth(1).unwrap().id;
    let cloned = carrier.clone().records().nth(1).unwrap().id;
    assert_eq!(id, cloned);
    let limits = FlowLimits {
        maximum_wire_bytes: 4_000_000,
        maximum_payload_bytes: 2_000_000,
        maximum_records: 3,
    };
    let reparsed = FlowCarrier::from_bytes(carrier.as_bytes(), limits).unwrap();
    let foreign = reparsed.records().nth(1).unwrap().id;
    assert_ne!(id, foreign);
    assert_eq!(format!("{id:?}"), format!("{foreign:?}"));
    assert!(carrier.replace_payload(&foreign, b"changed").is_err());
    assert_eq!(carrier.as_bytes(), reparsed.as_bytes());
}
