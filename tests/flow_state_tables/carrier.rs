use bessemer::flow::*;
pub fn carrier(identifier: u32, payload: &[u8]) -> FlowCarrier {
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
pub fn check_history(edit: &FlowCarrierEdit) {
    assert_eq!(edit.history().len(), 3);
    assert_eq!(edit.result().records().len(), 3);
    for ((before, after), entry) in edit
        .source()
        .records()
        .zip(edit.result().records())
        .zip(edit.history())
    {
        assert_eq!((&entry.source, &entry.result), (&before.id, &after.id));
        let relation = if before.payload == after.payload {
            FlowPayloadRelation::Copied
        } else {
            FlowPayloadRelation::Derived
        };
        assert_eq!(entry.relation, relation);
        if before.id.ordinal() != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}
