//! Original complete fresh selection contexts and sparse raw-wide projections.
#[path = "flow_selected_values/api.rs"]
mod api;
#[path = "flow_selected_values/fixture.rs"]
mod fixture;
#[path = "flow_selected_values/input_bindings.rs"]
mod input_bindings;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
#[path = "flow_state_tables/process.rs"]
mod state_process;
#[path = "flow_state_tables/streams.rs"]
mod state_streams;
use fixture::{SelectionCase, SelectionEntry};
use packet::*;
use std::collections::BTreeSet;
const ACCEPTED: &[u8] = &[
    0, 2, 3, 4, 6, 7, 10, 11, 12, 13, 14, 15, 30, 31, 32, 50, 51, 52, 60, 61, 62, 70, 72, 73, 74,
    76, 77, 80, 82, 83, 84, 88, 90, 91, 92,
];

fn converted_code(code: u8, version: i32) -> u8 {
    if version != 0 {
        return code;
    }
    for first in [30, 40, 50, 60] {
        if (first..first + 6).contains(&code) {
            return first + (code - first) / 2;
        }
    }
    code
}
#[test]
fn every_complete_original_context_and_projection_matches_independent_dense_selection() {
    let rows = fixture::cases();
    assert_eq!(rows.len(), 2729);
    for row in rows {
        check_inputs(&row);
        check_selection(&row);
        assert!(row.expected.fresh && row.expected.complete);
        assert!(
            !row.expected.context_owner
                && !row.expected.value_owner
                && !row.expected.scatter
                && !row.expected.engineering
        );
    }
}
fn check_inputs(row: &SelectionCase) {
    let input = &row.inputs;
    let decoded = if input.compressed_links {
        state_streams::inflate_exact(&input.links_stored)
    } else {
        input.links_stored.clone()
    };
    assert_eq!(decoded, input.links_decoded, "{}", row.name);
    assert_eq!(
        i32::from_le_bytes(decoded[..4].try_into().unwrap()),
        input.version
    );
    assert_eq!(
        u32::from_le_bytes(decoded[4..8].try_into().unwrap()) as usize,
        input.entries.len()
    );
    check_state_frame(row);
    for (ordinal, entry) in input.entries.iter().enumerate() {
        check_entry(row, ordinal, entry);
    }
}
fn check_entry(row: &SelectionCase, ordinal: usize, entry: &SelectionEntry) {
    assert_eq!(entry.ordinal as usize, ordinal);
    let start = entry.start as usize;
    assert_eq!(row.inputs.links_decoded[start + 64], entry.code);
    assert_eq!(
        entry.native_code,
        converted_code(entry.code, row.inputs.version)
    );
    let count = row.inputs.links_decoded[start + 65] as usize;
    assert_eq!(entry.end as usize, start + 66 + 4 * count);
}
fn expected_ordinals(row: &SelectionCase) -> Vec<u32> {
    let flags = &row.inputs.states_decoded[8..];
    row.inputs
        .entries
        .iter()
        .enumerate()
        .filter(|(index, entry)| {
            !flags.is_empty()
                && flags[*index] >= 128
                && ACCEPTED.contains(&converted_code(entry.code, row.inputs.version))
        })
        .map(|(index, _)| index as u32)
        .collect()
}
fn check_selection(row: &SelectionCase) {
    let expected = expected_ordinals(row);
    let mut mapping = vec![-1; row.inputs.entries.len()];
    for (dense, ordinal) in expected.iter().enumerate() {
        mapping[*ordinal as usize] = dense as i32;
    }
    assert_eq!(row.expected.link_count as usize, mapping.len());
    assert_eq!(row.expected.selected_ordinals, expected, "{}", row.name);
    assert_eq!(row.expected.dense_indices, mapping, "{}", row.name);
    assert_eq!(row.expected.selected_count as usize, expected.len());
    assert_eq!(row.expected.parsed_count, row.expected.selected_count);
    assert_eq!(row.expected.values.len(), expected.len() * 8);
    assert_eq!(row.expected.value_end as usize, row.expected.values.len());
    assert_eq!(
        row.expected.entry_patterns,
        projected(&row.expected.values, &mapping)
    );
}
fn projected(values: &[u8], mapping: &[i32]) -> Vec<Option<u64>> {
    mapping
        .iter()
        .map(|index| {
            if *index < 0 {
                return None;
            }
            let start = *index as usize * 8;
            Some(u64::from_le_bytes(
                values[start..start + 8].try_into().unwrap(),
            ))
        })
        .collect()
}
#[test]
fn versions_zero_and_nine_cover_every_byte_pair_with_real_serialized_input_records() {
    let rows = fixture::cases();
    let mut pairs = BTreeSet::new();
    for row in rows
        .iter()
        .filter(|row| row.name.starts_with("grid-") && [0, 9].contains(&row.inputs.version))
    {
        for (entry, flag) in row
            .inputs
            .entries
            .iter()
            .zip(&row.inputs.states_decoded[8..])
        {
            pairs.insert((row.inputs.version, entry.code, *flag));
        }
    }
    assert_eq!(pairs.len(), 2 * 256 * 256);
}
#[test]
fn remaining_eight_versions_cover_every_code_at_all_six_high_bit_boundaries() {
    let rows = fixture::cases();
    let mut pairs = BTreeSet::new();
    for row in rows
        .iter()
        .filter(|row| row.name.starts_with("grid-") && (1..9).contains(&row.inputs.version))
    {
        for (entry, flag) in row
            .inputs
            .entries
            .iter()
            .zip(&row.inputs.states_decoded[8..])
        {
            pairs.insert((row.inputs.version, entry.code, *flag));
        }
    }
    assert_eq!(pairs.len(), 8 * 256 * 6);
    let flags = [0, 1, 127, 128, 129, 255];
    assert!(pairs.iter().all(|(_, _, flag)| flags.contains(flag)));
}
#[test]
fn every_raw_wide_single_bit_complement_and_nonfinite_pattern_is_retained() {
    let rows = fixture::cases();
    let bits: BTreeSet<_> = rows
        .iter()
        .flat_map(|row| row.expected.values.as_chunks::<8>().0.iter())
        .map(|bytes| u64::from_le_bytes(*bytes))
        .collect();
    for bit in 0..64 {
        assert!(bits.contains(&(1u64 << bit)) && bits.contains(&!(1u64 << bit)));
    }
    for pattern in [
        0,
        u64::MAX,
        0x8000000000000000,
        0x7ff0000000000001,
        0x7ff8000000000042,
    ] {
        assert!(bits.contains(&pattern));
    }
}
#[test]
fn selected_zero_values_and_empty_state_arrays_remain_distinct_from_missing_entries() {
    let rows = fixture::cases();
    let zero = rows
        .iter()
        .find(|row| row.name == "selected-zero-distinct-from-absent")
        .unwrap();
    assert_eq!(
        zero.expected.entry_patterns,
        vec![Some(0), None, Some(0), None]
    );
    let absent: Vec<_> = rows
        .iter()
        .filter(|row| row.name.starts_with("absent-states-"))
        .collect();
    assert_eq!(absent.len(), 10);
    for row in absent {
        assert!(row.expected.values.is_empty());
        assert_eq!(row.expected.entry_patterns, vec![None; 4]);
    }
}
#[test]
fn native_three_record_inputs_reproduce_the_complete_3876_value_projection_without_pairing_claims()
{
    let rows = fixture::cases();
    let native: Vec<_> = rows
        .iter()
        .filter(|row| row.native_count.is_some())
        .collect();
    assert_eq!(native.len(), 1);
    let row = native[0];
    assert_eq!(row.native_count, Some(3876));
    assert_eq!(row.expected.link_count, 5532);
    assert_eq!(row.expected.selected_count, 3876);
    assert_eq!(row.expected.value_end, 31008);
    check_inputs(row);
    check_selection(row);
}

fn check_state_frame(row: &SelectionCase) {
    let input = &row.inputs;
    assert_eq!(
        state_streams::inflate_exact(&input.states_stored),
        input.states_decoded
    );
    let flags = &input.states_decoded[8..];
    assert_eq!(
        u32::from_le_bytes(input.states_decoded[4..8].try_into().unwrap()) as usize,
        flags.len()
    );
    assert!(flags.is_empty() || flags.len() == input.entries.len());
}
