use super::*;
pub(super) fn carrier(row: &ReplayCase) -> FlowCarrier {
    let limits = FlowLimits {
        maximum_wire_bytes: 4_000_000,
        maximum_records: 100_000,
        maximum_payload_bytes: 2_000_000,
    };
    let carrier = FlowCarrier::from_bytes(&row.raw, limits).unwrap();
    assert_eq!(carrier.as_bytes(), row.raw);
    assert_eq!(carrier.version(), row.version);
    assert_eq!(carrier.records().len(), row.physical.len());
    for (record, expected) in carrier.records().zip(&row.physical) {
        assert_eq!(record.identifier, expected.identifier);
        assert_eq!(record.header_offset as u64, expected.start);
        assert_eq!(12 + record.payload.len() as u64, expected.span);
    }
    carrier
}
pub(super) fn physical_maps(row: &ReplayCase) -> (BTreeMap<u32, RecordSpan>, BTreeMap<u64, u64>) {
    let mut active = BTreeMap::new();
    let mut saved = BTreeMap::new();
    for span in &row.physical {
        if span.identifier == DELETED {
            saved.insert(span.start, span.span);
            continue;
        }
        if let Some(old) = active.insert(span.identifier, *span) {
            saved.insert(old.start, old.span);
        }
    }
    (active, saved)
}
pub(super) fn assert_maps(
    projection: &ReplayProjection,
    active: &BTreeMap<u32, RecordSpan>,
    saved: &BTreeMap<u64, u64>,
) {
    let active_rows = active.values().copied().collect::<Vec<_>>();
    let saved_rows = saved
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect::<Vec<_>>();
    assert_eq!(projection.active, active_rows);
    assert_eq!(projection.saved, saved_rows);
    assert!(
        projection
            .active
            .windows(2)
            .all(|pair| pair[0].identifier < pair[1].identifier)
    );
    assert!(
        projection
            .saved
            .windows(2)
            .all(|pair| pair[0].0 < pair[1].0)
    );
}
pub(super) fn terminal_projection(
    row: &ReplayCase,
    expected: &TerminalProjection,
) -> FlowTerminalIndex {
    let carrier = carrier(row);
    let record = carrier.records().nth(expected.ordinal as usize).unwrap();
    let limits = control_options();
    let index = carrier
        .terminal_index_writer(&record.id, FlowControlLengthWidth::EightBytes, limits)
        .unwrap();
    assert_eq!(index.decoded_bytes(), expected.decoded);
    assert_eq!(index.stored_count(), expected.count as usize);
    assert_eq!(index.decoded_bytes().len(), expected.end as usize);
    check_terminal_transport(&index, expected);
    check_terminal_entries(&index, &expected.entries);
    index
}
fn check_terminal_transport(index: &FlowTerminalIndex, expected: &TerminalProjection) {
    assert_eq!(
        index.compressed_interval(),
        (8, expected.footer_start as usize)
    );
    assert_eq!(
        index.footer_interval(),
        (expected.footer_start as usize, expected.stored as usize)
    );
    assert_eq!(
        expected.compressed as usize,
        expected.footer_start as usize - 8
    );
    assert_eq!(index.backward_distance(), expected.distance);
    assert_eq!(index.footer_identifier(), expected.identifier);
    assert_eq!(expected.width, 8);
    assert!(expected.complete);
}
fn check_terminal_entries(index: &FlowTerminalIndex, expected: &[u8]) {
    let (entries, remainder) = expected.as_chunks::<20>();
    assert!(remainder.is_empty());
    assert_eq!(entries.len(), index.stored_count());
    for (ordinal, raw) in entries.iter().enumerate() {
        let entry = index.entry(ordinal).unwrap();
        assert_eq!(
            entry.identifier(),
            u32::from_le_bytes(raw[..4].try_into().unwrap())
        );
        assert_eq!(
            entry.record_start(),
            u64::from_le_bytes(raw[4..12].try_into().unwrap())
        );
        assert_eq!(
            entry.record_span(),
            u64::from_le_bytes(raw[12..].try_into().unwrap())
        );
    }
}
pub(super) fn terminal_maps(
    row: &ReplayCase,
    index: &FlowTerminalIndex,
) -> (BTreeMap<u32, RecordSpan>, BTreeMap<u64, u64>) {
    let mut active = BTreeMap::new();
    for ordinal in 0..index.stored_count() {
        let entry = index.entry(ordinal).unwrap();
        active.insert(
            entry.identifier(),
            RecordSpan {
                identifier: entry.identifier(),
                start: entry.record_start(),
                span: entry.record_span(),
            },
        );
    }
    let self_span = row.physical.last().unwrap();
    active.insert(INDEX, *self_span);
    let saved = saved_map(row, active.get(&SAVED));
    (active, saved)
}
fn saved_map(row: &ReplayCase, selected: Option<&RecordSpan>) -> BTreeMap<u64, u64> {
    let mut saved = BTreeMap::new();
    let Some(selected) = selected else {
        return saved;
    };
    let carrier = carrier(row);
    let record = carrier
        .records()
        .find(|record| record.header_offset as u64 == selected.start)
        .unwrap();
    assert_eq!(record.identifier, SAVED);
    assert_eq!(12 + record.payload.len() as u64, selected.span);
    let limits = control_options();
    let view = carrier.saved_span_writer(&record.id, limits).unwrap();
    for ordinal in 0..view.stored_count() {
        let entry = view.entry(ordinal).unwrap();
        saved.insert(entry.key_bits(), entry.value_bits());
    }
    saved
}

fn control_options() -> FlowControlOptions {
    FlowControlOptions {
        maximum_decoded_bytes: 2_000_000,
        maximum_entries: 100_000,
    }
}
