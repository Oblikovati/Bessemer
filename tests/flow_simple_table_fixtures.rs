//! Independent complete wire reconstruction before the typed simple-table reader/editor.
#[path = "flow_simple_tables/api.rs"]
mod api;
#[path = "flow_simple_tables/fixture.rs"]
mod fixture;
use fixture::*;
use std::io::Read;

#[test]
fn every_simple_table_field_reconstructs_wire_and_preserves_context_presence() {
    let cases = cases();
    assert_eq!(cases.len(), 118);
    for case in cases {
        let mut raw = case.version.to_le_bytes().to_vec();
        match &case.expected {
            ExpectedSimpleTable::Parallel(groups) => append_parallel(&mut raw, groups, &case),
            ExpectedSimpleTable::LeafPairs(rows) => append_leaf_pairs(&mut raw, rows, &case),
            ExpectedSimpleTable::FiveWords(table) => append_five_words(&mut raw, table, &case),
            ExpectedSimpleTable::Bytes(table) => append_bytes(&mut raw, table, &case),
        }
        assert_eq!(raw, case.frame.decoded, "{}", case.frame.name);
        assert!(!case.runtime && !case.roles);
        assert_eq!(
            case.tree_order,
            (case.frame.identifier == 437).then_some(false)
        );
        check_transport(&case.frame);
    }
}

fn append_parallel(raw: &mut Vec<u8>, groups: &[ParallelGroup; 2], case: &SimpleCase) {
    assert_eq!(case.frame.identifier, 407);
    assert!(case.frame.leaf.is_none() && case.frame.legacy.is_none());
    if case.version == 0 {
        for group in groups {
            raw.extend_from_slice(&group.count.to_le_bytes());
        }
    }
    for (ordinal, group) in groups.iter().enumerate() {
        assert_eq!(group.ordinal, ordinal as u32);
        assert_eq!(raw.len(), group.start as usize);
        if case.version == 0 {
            append_parallel_group(raw, group);
        } else {
            assert_eq!(group.count, 0);
            assert!(group.presence.is_none() && group.arrays.is_empty());
            raw.extend_from_slice(&0_i32.to_le_bytes());
        }
        assert_eq!(raw.len(), group.end as usize);
    }
}

fn append_parallel_group(raw: &mut Vec<u8>, group: &ParallelGroup) {
    assert!(group.count <= i32::MAX as u32);
    let presence = group.presence.unwrap();
    raw.extend_from_slice(&presence.to_le_bytes());
    assert_eq!(group.arrays.len(), if presence == 0 { 0 } else { 4 });
    for array in &group.arrays {
        assert_eq!(array.bits.len(), group.count as usize);
        assert_eq!(raw.len(), array.start as usize);
        raw.extend(array.bits.iter().flat_map(|value| value.to_le_bytes()));
        assert_eq!(raw.len(), array.end as usize);
    }
}

fn append_leaf_pairs(raw: &mut Vec<u8>, rows: &[LeafPair], case: &SimpleCase) {
    assert_eq!((case.frame.identifier, case.version), (437, 0));
    assert_eq!(case.frame.leaf, Some(rows.len() as u32));
    assert!(case.frame.legacy.is_none());
    for (ordinal, row) in rows.iter().enumerate() {
        assert_eq!(row.ordinal, ordinal as u32);
        assert_eq!(raw.len(), row.start as usize);
        raw.extend(row.bits.into_iter().flat_map(u64::to_le_bytes));
        assert_eq!(raw.len(), row.end as usize);
    }
}

fn append_five_words(raw: &mut Vec<u8>, table: &FiveWords, case: &SimpleCase) {
    assert_eq!(case.frame.identifier, 411);
    assert!(case.frame.leaf.is_none());
    assert_eq!(table.stored, case.version >= 4);
    if table.stored {
        assert!(case.frame.legacy.is_none());
        raw.extend_from_slice(&table.count.unwrap().to_le_bytes());
    } else {
        assert_eq!(table.count, case.frame.legacy);
    }
    assert_eq!(raw.len(), table.start as usize);
    check_five_layout(table, case.version);
    raw.extend(
        table
            .rows
            .iter()
            .flatten()
            .flat_map(|word| word.to_le_bytes()),
    );
}

fn append_bytes(raw: &mut Vec<u8>, table: &ByteTable, case: &SimpleCase) {
    assert_eq!(case.frame.identifier, 413);
    assert!(case.frame.leaf.is_none());
    assert_eq!(table.stored, case.version > 0);
    if table.stored {
        assert!(case.frame.legacy.is_none());
        raw.extend_from_slice(&table.count.to_le_bytes());
    } else {
        assert_eq!(case.frame.legacy, Some(table.count));
    }
    assert_eq!(raw.len(), table.start as usize);
    assert_eq!(table.bytes.len(), table.count as usize);
    if case.version >= 2 {
        assert_eq!(table.count, 0);
    }
    raw.extend_from_slice(&table.bytes);
}

fn check_transport(frame: &SimpleFrame) {
    if !frame.compressed {
        assert_eq!(frame.stored, frame.decoded);
        return;
    }
    let mut decoded = Vec::new();
    flate2::read::ZlibDecoder::new(frame.stored.as_slice())
        .read_to_end(&mut decoded)
        .unwrap();
    assert_eq!(decoded, frame.decoded);
}

#[test]
fn native_tables_pin_complete_arrays_rows_and_empty_byte_count() {
    let cases = cases();
    let decoded: Vec<usize> = cases[..4]
        .iter()
        .map(|case| case.frame.decoded.len())
        .collect();
    assert_eq!(decoded, [124948, 63748, 81128, 8]);
    let ExpectedSimpleTable::Parallel(groups) = &cases[0].expected else {
        panic!("expected parallel native table")
    };
    assert_eq!([groups[0].count, groups[1].count], [1944, 1960]);
    let ExpectedSimpleTable::LeafPairs(rows) = &cases[1].expected else {
        panic!("expected native leaf pairs")
    };
    assert_eq!(rows.len(), 3984);
    check_native_raw_tables(&cases);
}

fn check_five_layout(table: &FiveWords, version: i32) {
    let rows = if (3..=4).contains(&version) {
        table.count.unwrap()
    } else {
        0
    };
    assert_eq!(table.rows.len(), rows as usize);
    assert_eq!(table.wire_present, rows > 0);
    if version >= 5 {
        assert_eq!(table.count, Some(0));
    }
}

fn check_native_raw_tables(cases: &[SimpleCase]) {
    let ExpectedSimpleTable::FiveWords(table) = &cases[2].expected else {
        panic!("expected native five-word rows")
    };
    assert_eq!((table.count, table.rows.len()), (Some(4056), 4056));
    let ExpectedSimpleTable::Bytes(table) = &cases[3].expected else {
        panic!("expected native byte table")
    };
    assert_eq!((table.count, table.bytes.len()), (0, 0));
}
