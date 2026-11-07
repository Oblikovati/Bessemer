//! Rebuild complete legacy block payloads independently before introducing their typed API.
#[path = "flow_block_data/fixture.rs"]
mod fixture;
use fixture::*;
use std::io::Read;

#[test]
fn all_273_profiles_rebuild_exact_bytes_and_preserve_signed_count_branches() {
    for case in cases() {
        let mut rebuilt = prefix_bytes(&case);
        let mut totals = [0_u32; 3];
        assert_eq!(case.blocks.len(), case.leaf_count as usize);
        for (ordinal, block) in case.blocks.iter().enumerate() {
            assert_eq!(block.ordinal, ordinal as u32);
            append_block(&mut rebuilt, case.prefix.version, block, &mut totals);
        }
        assert_eq!(totals, case.totals, "{}", case.name);
        assert_eq!(
            case.entries_equal_total,
            totals[0] as u64 == case.prefix.mathematical
        );
        assert_eq!(rebuilt, case.decoded, "{}", case.name);
        check_transport(&case);
    }
}

fn prefix_bytes(case: &BlockCase) -> Vec<u8> {
    let prefix = &case.prefix;
    assert!((1..=17).contains(&prefix.version));
    assert!(prefix.fluid >= 0 && prefix.solid >= 0);
    let total = prefix.fluid as u64 + prefix.solid as u64;
    assert_eq!(prefix.mathematical, total);
    assert_eq!(prefix.native, total as u32 as i32);
    let bitmap = if prefix.version >= 8 && prefix.native > 0 {
        prefix.native as usize / 8 + 1
    } else {
        0
    };
    assert_eq!(case.bitmap.len(), bitmap);
    let mut raw: Vec<u8> = [prefix.version, prefix.fluid, prefix.solid]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    raw.extend_from_slice(&case.bitmap);
    raw
}

fn append_block(raw: &mut Vec<u8>, version: i32, block: &ExpectedBlock, totals: &mut [u32; 3]) {
    assert_eq!(raw.len(), block.start as usize);
    assert_eq!(block.signed_count, block.raw_count as i16);
    assert_eq!(block.entries.len(), block.signed_count.max(0) as usize);
    assert_eq!(block.native_assign, version >= 13);
    raw.push(block.byte);
    raw.extend_from_slice(&block.word_10.to_le_bytes());
    raw.extend_from_slice(&block.raw_count.to_le_bytes());
    for entry in &block.entries {
        append_entry(raw, version, entry);
        totals[0] += 1;
        totals[1] += entry.tail.first.len() as u32;
        totals[2] += entry.tail.second.len() as u32;
    }
    assert_eq!(raw.len(), block.end as usize);
}

fn append_entry(raw: &mut Vec<u8>, version: i32, entry: &ExpectedEntry) {
    assert_eq!(raw.len(), entry.start as usize);
    raw.extend_from_slice(&entry.identifier.to_le_bytes());
    append_optional_word(raw, entry.legacy_index, version < 12);
    if version == 1 {
        assert_eq!(entry.member_20, entry.legacy_index);
    } else {
        append_optional_word(raw, entry.member_20, version < 12);
    }
    raw.extend_from_slice(&entry.member_28.to_le_bytes());
    append_optional_byte(raw, entry.discarded_prefix, (11..=13).contains(&version));
    raw.push(entry.state);
    append_optional_byte(raw, entry.flag_2, version >= 5);
    append_optional_byte(raw, entry.flag_4, version >= 17);
    append_tail(raw, version, &entry.tail);
    assert_eq!(raw.len(), entry.end as usize);
}

fn append_optional_word(raw: &mut Vec<u8>, value: Option<i32>, required: bool) {
    assert_eq!(value.is_some(), required);
    if let Some(word) = value {
        raw.extend_from_slice(&word.to_le_bytes());
    }
}

fn append_optional_byte(raw: &mut Vec<u8>, value: Option<u8>, required: bool) {
    assert_eq!(value.is_some(), required);
    if let Some(byte) = value {
        raw.push(byte);
    }
}

fn append_tail(raw: &mut Vec<u8>, version: i32, tail: &ExpectedTail) {
    assert_eq!(tail.first.len(), tail.descriptors[6] as usize);
    for descriptor in tail.descriptors {
        if version == 1 {
            raw.push(u8::try_from(descriptor).unwrap());
        } else {
            raw.extend_from_slice(&descriptor.to_le_bytes());
        }
    }
    raw.extend(tail.first.iter().flat_map(|word| word.to_le_bytes()));
    append_second_count(raw, version, tail);
    assert_eq!(tail.second.len(), tail.second_count.max(0) as usize);
    raw.extend(tail.second.iter().flat_map(|word| word.to_le_bytes()));
    assert_eq!(tail.discarded_suffix.is_some(), matches!(version, 16 | 17));
    if let Some(suffix) = tail.discarded_suffix {
        raw.extend_from_slice(&suffix.to_le_bytes());
    }
}

fn append_second_count(raw: &mut Vec<u8>, version: i32, tail: &ExpectedTail) {
    if version == 1 {
        let byte = u8::try_from(tail.raw_second.unwrap()).unwrap();
        assert_eq!(tail.second_count, i32::from(byte as i8));
        raw.push(byte);
        return;
    }
    if version < 10 {
        let word = tail.raw_second.unwrap();
        assert_eq!(tail.second_count, i32::from(word));
        raw.extend_from_slice(&word.to_le_bytes());
        return;
    }
    assert_eq!(tail.raw_second, None);
    raw.extend_from_slice(&tail.second_count.to_le_bytes());
}

fn check_transport(case: &BlockCase) {
    if !case.compressed {
        assert_eq!(case.stored, case.decoded);
        return;
    }
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(case.stored.as_slice())
        .take(400_001)
        .read_to_end(&mut raw)
        .unwrap();
    assert_eq!(raw, case.decoded);
}

#[test]
fn native_cpt_totals_cover_every_identifier_without_inventing_a_tree_assignment() {
    let native = cases().remove(0);
    assert_eq!(native.name, "native-cpt");
    assert_eq!(native.prefix.version, 14);
    assert_eq!((native.leaf_count, native.bitmap.len()), (3984, 745));
    assert_eq!(native.totals, [5952, 31760, 9588]);
    assert_eq!(native.decoded.len(), 352725);
    check_native_block_totals(&native);
}

fn check_native_block_totals(native: &BlockCase) {
    let identifiers: std::collections::BTreeSet<_> = native
        .blocks
        .iter()
        .flat_map(|block| &block.entries)
        .map(|entry| entry.identifier)
        .collect();
    assert_eq!(identifiers, (0..5952).collect());
    assert_eq!(
        native
            .blocks
            .iter()
            .map(|block| u64::from(block.word_10))
            .sum::<u64>(),
        3368
    );
    assert_eq!(native.blocks.first().unwrap().start, 757);
    assert_eq!(native.blocks.last().unwrap().end, 352725);
}
