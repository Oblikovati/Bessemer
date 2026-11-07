//! Independent reconstruction of every raw legacy table field before the typed reader/editor.
#[path = "flow_legacy_tables/api.rs"]
mod api;
#[path = "flow_legacy_tables/fixture.rs"]
mod fixture;
use fixture::*;
use std::io::Read;

#[test]
fn all_173_tables_rebuild_exact_wire_without_losing_context_presence_or_discarded_fields() {
    for case in cases() {
        let mut raw = case.version.to_le_bytes().to_vec();
        match &case.expected {
            ExpectedTable::Pointers(tables) => append_pointer_table(&mut raw, &case, tables),
            ExpectedTable::Words(words) => append_word_table(&mut raw, &case, words),
        }
        assert_eq!(raw, case.frame.decoded, "{}", case.frame.name);
        check_transport(&case.frame);
        api::check_case(&case);
    }
}

fn append_pointer_table(raw: &mut Vec<u8>, case: &TableCase, tables: &ExpectedPointers) {
    assert_eq!(case.frame.identifier, 401);
    assert!(case.frame.legacy.is_none());
    assert!((0..=3).contains(&case.version));
    assert_eq!(tables.groups.len(), 2);
    assert_eq!(tables.indices_checked, case.frame.pointer_count.is_some());
    assert!(!tables.pointer_population);
    for (ordinal, group) in tables.groups.iter().enumerate() {
        assert_eq!(group.ordinal, ordinal as u32);
        append_group(raw, group, case.version);
        check_pair_bounds(group, case.frame.pointer_count);
    }
}

fn append_group(raw: &mut Vec<u8>, group: &ExpectedGroup, version: i32) {
    assert_eq!(raw.len(), group.start as usize);
    assert!(group.count <= i32::MAX as u32);
    raw.extend_from_slice(&group.count.to_le_bytes());
    append_optional(raw, group.discarded_count, version == 1);
    assert_eq!(group.body.is_some(), group.count > 0);
    if let Some(body) = &group.body {
        append_group_body(raw, group, body, version);
    }
    assert_eq!(raw.len(), group.end as usize);
}

fn append_group_body(
    raw: &mut Vec<u8>,
    group: &ExpectedGroup,
    body: &ExpectedGroupBody,
    version: i32,
) {
    raw.extend(body.members.into_iter().flat_map(i32::to_le_bytes));
    raw.extend(body.lengths.into_iter().flat_map(i32::to_le_bytes));
    let discarded =
        usize::from(group.ordinal == 1 && version < 2) + if version == 1 { 2 } else { 0 };
    assert_eq!(body.discarded.len(), discarded);
    raw.extend(
        body.discarded
            .iter()
            .flatten()
            .flat_map(|word| word.to_le_bytes()),
    );
    append_group_arrays(raw, body);
    append_optional(raw, body.pre_pairs, version == 1);
    assert_eq!(body.pairs.len(), group.count as usize);
    raw.extend(
        body.pairs
            .iter()
            .flatten()
            .flat_map(|word| word.to_le_bytes()),
    );
}

fn append_array(raw: &mut Vec<u8>, array: &ExpectedWideArray) {
    assert_eq!(raw.len(), array.start as usize);
    assert_eq!(array.end - array.start, array.bits.len() as u32 * 8);
    for bits in &array.bits {
        assert_eq!(f64::from_bits(*bits).to_bits(), *bits);
        raw.extend_from_slice(&bits.to_le_bytes());
    }
    assert_eq!(raw.len(), array.end as usize);
}

fn append_optional(raw: &mut Vec<u8>, value: Option<i32>, present: bool) {
    assert_eq!(value.is_some(), present);
    if let Some(word) = value {
        raw.extend_from_slice(&word.to_le_bytes());
    }
}

fn check_pair_bounds(group: &ExpectedGroup, context: Option<u32>) {
    let Some(count) = context else {
        return;
    };
    if let Some(body) = &group.body {
        for index in body.pairs.iter().flatten() {
            assert!(*index >= 0 && (*index as u32) < count);
        }
    }
}

fn append_word_table(raw: &mut Vec<u8>, case: &TableCase, words: &ExpectedWords) {
    assert_eq!(case.frame.identifier, 405);
    assert!(case.frame.pointer_count.is_none());
    assert_eq!(words.count_stored, case.version == 1);
    if case.version == 0 {
        check_legacy_context(case, words.count);
    } else {
        assert_eq!(case.version, 1);
        assert!(case.frame.legacy.is_none());
        raw.extend_from_slice(&words.count.to_le_bytes());
    }
    assert_eq!(words.count as usize, words.words.len());
    assert_eq!(raw.len(), words.words_start as usize);
    raw.extend(words.words.iter().flat_map(|word| word.to_le_bytes()));
    assert!(!words.roles_resolved);
}

fn check_transport(frame: &TableFrame) {
    if !frame.compressed {
        assert_eq!(frame.stored, frame.decoded);
        return;
    }
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(frame.stored.as_slice())
        .take(500_001)
        .read_to_end(&mut raw)
        .unwrap();
    assert_eq!(raw, frame.decoded);
}

#[test]
fn native_tables_keep_pair_multiplicity_and_range_checks_separate_from_population() {
    let cases = cases();
    assert_eq!(cases[0].frame.decoded, cases[1].frame.decoded);
    assert_eq!(cases[0].frame.pointer_count, None);
    assert_eq!(cases[1].frame.pointer_count, Some(5952));
    let ExpectedTable::Pointers(pointers) = &cases[1].expected else {
        panic!("expected native pointer table")
    };
    check_native_pointer_totals(pointers, &cases[1]);
    check_native_word_totals(&cases[2]);
}

fn check_native_pointer_totals(pointers: &ExpectedPointers, case: &TableCase) {
    assert_eq!(
        pointers.groups.iter().map(|g| g.count).collect::<Vec<_>>(),
        [9338, 6542]
    );
    assert_eq!(
        pointers
            .groups
            .iter()
            .flat_map(|g| &g.body.as_ref().unwrap().arrays)
            .map(|a| a.bits.len())
            .sum::<usize>(),
        7930
    );
    assert_eq!(case.frame.decoded.len(), 190540);
    assert!(pointers.indices_checked && !pointers.pointer_population);
}

fn check_native_word_totals(case: &TableCase) {
    let ExpectedTable::Words(words) = &case.expected else {
        panic!("expected native word table")
    };
    assert_eq!(words.count, 5952);
    assert_eq!(case.frame.decoded.len(), 23816);
    assert_eq!(
        words
            .words
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>(),
        [0, 1].into_iter().collect()
    );
}

fn append_group_arrays(raw: &mut Vec<u8>, body: &ExpectedGroupBody) {
    assert_eq!(body.arrays.len(), 3);
    for (array, length) in body.arrays.iter().zip(body.lengths) {
        assert!(length >= 0);
        assert_eq!(array.bits.len(), length as usize);
        append_array(raw, array);
    }
}

fn check_legacy_context(case: &TableCase, count: u32) {
    let context = case.frame.legacy.unwrap();
    assert!(context.into_iter().all(|n| n <= i32::MAX as u32));
    let total: u64 = context.into_iter().map(u64::from).sum();
    assert!(total <= i32::MAX as u64);
    assert_eq!(total, u64::from(count));
}
