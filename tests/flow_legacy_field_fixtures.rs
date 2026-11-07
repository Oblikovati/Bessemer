//! Frozen legacy105/113/155 framing, pure comparison branches and selection precedence.
#[path = "flow_conditional_items/case_reader.rs"]
mod case_reader;
#[path = "flow_conditional_items/types.rs"]
mod conditional_types;
#[path = "flow_legacy_fields/fixture.rs"]
mod fixture;
#[path = "flow_legacy_fields/oracles.rs"]
mod oracles;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_field_values/stored_fields.rs"]
mod stored_fields;
use conditional_types::*;
use packet::*;
use std::collections::BTreeSet;
#[test]
fn all_1007_native_and_owned_word_payloads_match_complete_raw_framing_and_scalar_queries() {
    let rows = fixture::words();
    assert_eq!(rows.len(), 1007);
    let mut native = [0; 2];
    for row in rows {
        check_words(&row);
        if row.name.starts_with("native-") {
            native[0] += 1;
            native[1] += row.raw.len();
        }
    }
    assert_eq!(native, [3, 32068]);
}
fn check_words(row: &fixture::WordCase) {
    let start = word_start(row);
    assert_eq!(row.end, row.raw.len());
    assert_eq!(row.raw.len(), start + row.patterns.len() * 8);
    for (index, value) in row.patterns.iter().enumerate() {
        assert_eq!(
            u64::from_le_bytes(
                row.raw[start + index * 8..start + index * 8 + 8]
                    .try_into()
                    .unwrap()
            ),
            *value
        );
    }
    assert!(!row.runtime && !row.engineering);
    assert_eq!(row.query.is_some(), row.identifier == 105);
    if let Some(query) = &row.query {
        oracles::scalar(row.patterns[7], row.patterns[10], query);
    }
}
fn word_start(row: &fixture::WordCase) -> usize {
    match row.identifier {
        105 => {
            assert_eq!(row.patterns.len(), 320);
            0
        }
        113 => {
            assert_eq!(
                i32::from_le_bytes(row.raw[..4].try_into().unwrap()) as usize,
                row.patterns.len()
            );
            4
        }
        value => panic!("identifier {value}; expected 105 or 113"),
    }
}

#[test]
fn all_90601_scalar_domain_pairs_match_independent_integer_bit_classification() {
    let rows = fixture::queries();
    assert_eq!(rows.len(), 301 * 301);
    let domain: BTreeSet<_> = rows.iter().map(|row| row.slot7).collect();
    assert_eq!(domain.len(), 301);
    for row in rows {
        oracles::scalar(row.slot7, row.slot10, &row.query);
    }
    for boundary in [0x406fa00000000000u64, 0x4081a80000000000] {
        for offset in -2i64..=2 {
            assert!(domain.contains(&boundary.wrapping_add_signed(offset)));
        }
    }
}
#[test]
fn all_1986_raw_fallback_groups_rebuild_shared_fields_scope_intervals_and_count_locations() {
    let rows = fixture::groups();
    assert_eq!(rows.len(), 1986);
    for row in &rows {
        assert!(!row.name.is_empty());
        oracles::group(row);
    }
    let words = stored_fields::patterns(rows.iter().flat_map(|row| row.fields.iter()), 2);
    let wide = stored_fields::patterns(rows.iter().flat_map(|row| row.fields.iter()), 3);
    assert!(words.contains(&0xffffffff));
    assert!(wide.contains(&0x8000000000000000));
    assert!(wide.contains(&0x7ff0000000000001));
}
#[test]
fn all_400_ordered_selection_requests_preserve_latest_preference_empty_success_and_selected_failures()
 {
    let rows = fixture::selections();
    assert_eq!(rows.len(), 400);
    for row in &rows {
        assert!(!row.name.is_empty());
        oracles::selection(row);
    }
    assert!(rows.iter().any(|row| !row.completed));
    assert!(
        rows.iter()
            .any(|row| row.completed && row.identifier.is_none())
    );
    assert!(rows.iter().any(|row| row.identifier == Some(113)));
    assert!(rows.iter().any(|row| row.identifier == Some(155)));
}
