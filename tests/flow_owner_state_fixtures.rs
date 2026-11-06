//! Complete raw owner-state fields and documented latest-preferred update projections.
#[path = "flow_owner_state/api.rs"]
mod api;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
#[path = "flow_state_tables/process.rs"]
mod state_process;
use bessemer::flow::*;
use packet::*;
use std::collections::BTreeSet;
type Updates = [Option<i32>; 7];
struct OwnerCase {
    name: String,
    native_identifier: Option<i32>,
    identifier: u32,
    raw: Vec<u8>,
    version: Option<i32>,
    gate: bool,
    values: Vec<u8>,
    state: Option<i32>,
    updates: Updates,
    end: u32,
}
enum SelectionOutcome {
    Empty,
    Selected {
        identifier: u32,
        ordinal: u32,
        updates: Updates,
    },
    Rejected,
}
struct SelectionCase {
    name: String,
    raw: Vec<u8>,
    outcome: SelectionOutcome,
}
fn updates(cursor: &mut PacketCursor) -> Updates {
    std::array::from_fn(|_| optional(cursor, signed))
}
fn case(cursor: &mut PacketCursor) -> OwnerCase {
    OwnerCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native_identifier: optional(cursor, signed),
        identifier: word(cursor),
        raw: blob(cursor),
        version: optional(cursor, signed),
        gate: boolean(cursor),
        values: blob(cursor),
        state: optional(cursor, signed),
        updates: updates(cursor),
        end: word(cursor),
    }
}
fn selection(cursor: &mut PacketCursor) -> SelectionCase {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let raw = blob(cursor);
    let outcome = match byte(cursor) {
        0 => SelectionOutcome::Empty,
        1 => SelectionOutcome::Selected {
            identifier: word(cursor),
            ordinal: word(cursor),
            updates: updates(cursor),
        },
        2 => SelectionOutcome::Rejected,
        value => panic!("selection status {value}; expected zero through two"),
    };
    SelectionCase { name, raw, outcome }
}
fn cases() -> (Vec<OwnerCase>, Vec<SelectionCase>) {
    let mut cursor = cursor(
        include_bytes!("fixtures/flow-owner-state/records.bin.gz"),
        *b"FOS1",
    );
    let result = (counted(&mut cursor, case), counted(&mut cursor, selection));
    finish(&cursor);
    result
}
fn signed_word(raw: &[u8], index: usize) -> i32 {
    i32::from_le_bytes(raw[index * 4..index * 4 + 4].try_into().unwrap())
}
fn expected_updates(identifier: u32, raw: &[u8]) -> Updates {
    if identifier == 434 {
        return modern_updates(raw);
    }
    if raw[..4] == [255; 4] {
        return [None; 7];
    }
    [
        Some(signed_word(raw, 5)),
        Some(signed_word(raw, 7)),
        Some(signed_word(raw, 1)),
        Some(signed_word(raw, 8)),
        Some(signed_word(raw, 2)),
        Some(-1),
        Some(0),
    ]
}
fn modern_updates(raw: &[u8]) -> Updates {
    std::array::from_fn(|index| {
        Some(if index == 6 {
            raw[28] as i32
        } else {
            signed_word(raw, index + 1)
        })
    })
}

fn rebuilt(row: &OwnerCase) -> Vec<u8> {
    let mut raw = Vec::new();
    if let Some(version) = row.version {
        raw.extend_from_slice(&version.to_le_bytes());
    }
    raw.extend_from_slice(&row.values);
    if let Some(state) = row.state {
        raw.push(u8::try_from(state).unwrap());
    }
    raw
}
#[test]
fn all_owned_raw_fields_and_native_updates_match_independent_member_equations() {
    let (rows, selections) = cases();
    assert_eq!(rows.len(), 4613);
    assert_eq!(selections.len(), 27);
    for row in rows {
        assert_eq!(row.end as usize, row.raw.len());
        assert_eq!(rebuilt(&row), row.raw, "{}", row.name);
        assert_eq!(
            row.updates,
            expected_updates(row.identifier, &row.raw),
            "{}",
            row.name
        );
        check_shape(&row);
    }
}
fn check_shape(row: &OwnerCase) {
    if row.identifier == 122 {
        assert_eq!(
            (row.raw.len(), row.values.len(), row.version, row.state),
            (200, 200, None, None)
        );
        assert_eq!(row.gate, row.raw[..4] != [255; 4]);
        return;
    }
    assert_eq!(row.identifier, 434);
    assert!(row.gate);
    assert_eq!((row.raw.len(), row.values.len()), (29, 24));
    assert!(matches!(row.version, Some(0 | 1)));
    assert_eq!(row.state, Some(row.raw[28] as i32));
}
#[test]
fn every_legacy_word_and_modern_member_retains_each_bit_and_complement() {
    let (rows, _) = cases();
    for (identifier, count) in [(122, 50), (434, 6)] {
        for index in 0..count {
            let values: BTreeSet<_> = rows
                .iter()
                .filter(|row| row.identifier == identifier)
                .map(|row| signed_word(&row.values, index) as u32)
                .collect();
            check_word_bits(&values);
        }
    }
    for version in [0, 1] {
        let states: BTreeSet<_> = rows
            .iter()
            .filter(|row| row.version == Some(version))
            .map(|row| row.state.unwrap())
            .collect();
        assert_eq!(states, (0..256).collect());
    }
}
fn check_word_bits(values: &BTreeSet<u32>) {
    for bit in 0..32 {
        assert!(values.contains(&(1u32 << bit)) && values.contains(&!(1u32 << bit)));
    }
}

fn carrier(raw: &[u8]) -> FlowCarrier {
    FlowCarrier::from_bytes(
        raw,
        FlowLimits {
            maximum_wire_bytes: 1_000_000,
            maximum_payload_bytes: 1_000_000,
            maximum_records: 16,
        },
    )
    .unwrap()
}
#[test]
fn all_selection_outcomes_preserve_priority_latest_occurrence_and_no_error_fallback() {
    for row in cases().1 {
        let source = carrier(&row.raw);
        let preferred = source
            .latest(434)
            .unwrap()
            .or_else(|| source.latest(122).unwrap());
        check_selection(&row, preferred);
    }
}
fn check_selection(row: &SelectionCase, preferred: Option<FlowRecord<'_>>) {
    match &row.outcome {
        SelectionOutcome::Empty => assert!(preferred.is_none(), "{}", row.name),
        SelectionOutcome::Rejected => assert!(malformed_record(preferred.unwrap()), "{}", row.name),
        SelectionOutcome::Selected {
            identifier,
            ordinal,
            updates,
        } => {
            check_selected(
                preferred.unwrap(),
                *identifier,
                *ordinal,
                updates,
                &row.name,
            );
        }
    }
}
fn check_selected(
    record: FlowRecord<'_>,
    identifier: u32,
    ordinal: u32,
    updates: &Updates,
    name: &str,
) {
    assert_eq!(
        (record.identifier, record.id.ordinal()),
        (identifier, ordinal as usize)
    );
    assert_eq!(
        *updates,
        expected_updates(record.identifier, record.payload),
        "{}",
        name
    );
}

fn malformed_record(record: FlowRecord<'_>) -> bool {
    match record.identifier {
        122 => record.payload.len() != 200,
        434 => record.payload.len() != 29 || !matches!(signed_word(record.payload, 0), 0 | 1),
        _ => true,
    }
}
#[test]
fn native_sources_preserve_three_physical_records_and_two_distinct_payloads() {
    let (rows, _) = cases();
    let native: Vec<_> = rows
        .iter()
        .filter(|row| row.native_identifier.is_some())
        .collect();
    assert_eq!(native.len(), 3);
    assert_eq!(native[0].raw, native[2].raw);
    for row in &native {
        assert_eq!(row.native_identifier, Some(row.identifier as i32));
    }
    assert_eq!(native[1].identifier, 434);
    assert_eq!(native[1].state, Some(1));
    let values: Vec<_> = (0..6)
        .map(|index| signed_word(&native[1].values, index))
        .collect();
    assert_eq!(values, [3368, 2584, 5952, 0, 1936, 0]);
}
