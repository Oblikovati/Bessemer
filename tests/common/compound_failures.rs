//! Complete negative allocation, metadata and budget regressions.
use super::reference::{LIMITS, cases, check_case, owned, word_at};
use bessemer::compound::*;

#[test]
fn impossible_root_mini_sector_count_fails_without_a_giant_payload() {
    let packet = owned();
    let cases = cases(&packet, false);
    let mut raw = cases[3].bytes.to_vec();
    let root = (word_at(&raw, 48) + 1) * 4096;
    raw[root + 120..root + 128].copy_from_slice(&((0xffff_fffa_u64 + 1) * 64).to_le_bytes());
    let limits = CompoundLimits {
        maximum_file_bytes: usize::MAX,
        ..LIMITS
    };
    let error = CompoundFile::from_bytes(&raw, limits).unwrap_err();
    assert!(error.cause.contains("mini-sector identifiers"));
}

#[test]
fn every_short_small_file_prefix_and_bad_header_fails_without_partial_publication() {
    let packet = owned();
    let cases = cases(&packet, false);
    let original = cases[2].bytes;
    for length in 0..original.len() {
        assert!(
            CompoundFile::from_bytes(&original[..length], LIMITS).is_err(),
            "prefix {length}"
        );
    }
    reject_bad_headers(original);
    assert!(CompoundFile::from_bytes(&[original, b"x"].concat(), LIMITS).is_err());
}

fn reject_bad_headers(original: &[u8]) {
    for (offset, value) in [
        (26, 5),
        (28, 0),
        (30, 12),
        (32, 9),
        (34, 1),
        (40, 1),
        (56, 64),
    ] {
        let mut broken = original.to_vec();
        broken[offset] = value;
        assert!(
            CompoundFile::from_bytes(&broken, LIMITS).is_err(),
            "header byte {offset}"
        );
    }
}

#[test]
fn normal_mini_and_directory_cycles_aliases_and_dangling_links_are_rejected() {
    let packet = owned();
    let cases = cases(&packet, false);
    let original = cases[2].bytes;
    for (offset, value) in chain_corruptions(original) {
        reject_word(original, offset, value);
    }
    let directory = (word_at(original, 48) + 1) * 512;
    reject_small_alias(original, directory);
}

fn chain_corruptions(original: &[u8]) -> [(usize, u32); 8] {
    let directory = (word_at(original, 48) + 1) * 512;
    let fat = (word_at(original, 76) + 1) * 512;
    let big = word_at(original, directory + 3 * 128 + 116);
    let mini_fat = (word_at(original, 60) + 1) * 512;
    [
        (fat + big * 4, big as u32),
        (fat + big * 4, u32::MAX),
        (directory + 3 * 128 + 116, word_at(original, 48) as u32),
        (mini_fat, 0),
        (directory + 128 + 76, 1),
        (directory + 3 * 128 + 68, 3),
        (directory + 76, u32::MAX),
        (directory + 76, 99999),
    ]
}

fn reject_word(original: &[u8], offset: usize, value: u32) {
    let mut broken = original.to_vec();
    broken[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    assert!(
        CompoundFile::from_bytes(&broken, LIMITS).is_err(),
        "offset {offset}, value {value:#010x}"
    );
}

fn reject_small_alias(original: &[u8], directory: usize) {
    let mut broken = original.to_vec();
    let empty = directory + 4 * 128;
    broken[empty + 116..empty + 120].copy_from_slice(&0_u32.to_le_bytes());
    broken[empty + 120..empty + 128].copy_from_slice(&39_u64.to_le_bytes());
    assert!(CompoundFile::from_bytes(&broken, LIMITS).is_err());
}

#[test]
fn free_physical_slack_is_retained_but_unowned_allocations_and_fat_padding_are_rejected() {
    let packet = owned();
    let cases = cases(&packet, false);
    let original = &cases[2];
    let mut extended = original.bytes.to_vec();
    extended.extend_from_slice(&[0xa5; 512]);
    let file = CompoundFile::from_bytes(&extended, LIMITS).unwrap();
    assert_eq!(file.to_bytes(), extended);
    assert_eq!(
        file.reconstructed_bytes().unwrap(),
        original.canonical.unwrap()
    );
    assert_eq!(file.coverage().claimed_sectors + 1, file.coverage().sectors);
    let fat = (word_at(&extended, 76) + 1) * 512;
    let previous_sectors = original.bytes.len() / 512 - 1;
    reject_word(&extended, fat + previous_sectors * 4, 0xffff_fffe);
    reject_word(original.bytes, fat + previous_sectors * 4, 0xffff_fffe);
}

#[test]
fn inactive_directory_bytes_remain_exact_and_invalid_active_metadata_fails() {
    let packet = owned();
    let cases = cases(&packet, false);
    let original = cases[2].bytes;
    let directory = (word_at(original, 48) + 1) * 512;
    reject_bad_directory_metadata(original, directory);
    let mut opaque = original.to_vec();
    opaque[directory + 5 * 128 + 96] = 0xa5;
    let file = CompoundFile::from_bytes(&opaque, LIMITS).unwrap();
    let rebuilt = file.reconstructed_bytes().unwrap();
    assert_eq!(
        rebuilt[(word_at(&rebuilt, 48) + 1) * 512 + 5 * 128 + 96],
        0xa5
    );
}

fn reject_bad_directory_metadata(original: &[u8], directory: usize) {
    for (offset, value) in [
        (directory + 128 + 66, 9),
        (directory + 128 + 67, 2),
        (directory + 128 + 64, 3),
        (directory + 128, b'/'),
        (directory + 128 + 2, b'x'),
    ] {
        let mut broken = original.to_vec();
        broken[offset] = value;
        assert!(CompoundFile::from_bytes(&broken, LIMITS).is_err());
    }
}

#[test]
fn every_resource_budget_rejects_construction_and_failed_edits_keep_the_input() {
    let packet = owned();
    let cases = cases(&packet, false);
    let original = cases[2].bytes;
    let baseline = check_case(&cases[2]);
    assert!(baseline.coverage().reconstruction_checked);
    for limits in small_limits(&baseline) {
        assert!(CompoundFile::from_bytes(original, limits).is_err());
    }
    check_failed_edits(original);
}

fn small_limits(file: &CompoundFile) -> [CompoundLimits; 5] {
    let file_limit = CompoundLimits {
        maximum_file_bytes: file.to_bytes().len() - 1,
        ..LIMITS
    };
    let stream_limit = CompoundLimits {
        maximum_stream_bytes: 7000,
        ..LIMITS
    };
    let total_limit = CompoundLimits {
        maximum_total_stream_bytes: file.coverage().logical_stream_bytes - 1,
        ..LIMITS
    };
    let tree = small_tree_limits(file.coverage().directory_slots);
    [file_limit, stream_limit, total_limit, tree[0], tree[1]]
}

fn small_tree_limits(slots: usize) -> [CompoundLimits; 2] {
    [
        CompoundLimits {
            maximum_directory_entries: slots - 1,
            ..LIMITS
        },
        CompoundLimits {
            maximum_storage_depth: 1,
            ..LIMITS
        },
    ]
}

fn check_failed_edits(original: &[u8]) {
    let limits = CompoundLimits {
        maximum_file_bytes: original.len(),
        ..LIMITS
    };
    let file = CompoundFile::from_bytes(original, limits).unwrap();
    let entry = file.entry_at_path("S/B").unwrap();
    assert!(
        file.edit_stream(entry.id(), &vec![7; original.len() * 2])
            .is_err()
    );
    assert!(
        file.edit_stream(entry.id(), &vec![7; original.len()])
            .is_err()
    );
    assert_eq!(file.to_bytes(), original);
    assert_eq!(
        file.coverage().mode,
        CompoundCheckMode::ExactAllocationAndDirectoryLinks
    );
}

#[test]
fn fat_difat_markers_and_table_extent_claims_are_checked_before_publication() {
    let packet = owned();
    let cases = cases(&packet, false);
    let original = cases[2].bytes;
    let fat_id = word_at(original, 76);
    let fat = (fat_id + 1) * 512;
    reject_word(original, fat + fat_id * 4, 0xffff_fffe);
    reject_word(original, 80, 0);
    reject_word(original, 44, u32::MAX);
    reject_word(original, 64, u32::MAX);
    let extended = cases[4].bytes;
    let difat_id = word_at(extended, 68);
    reject_word(extended, (difat_id + 1) * 512 + 508, difat_id as u32);
    reject_word(extended, 80, word_at(extended, 76) as u32);
}

#[test]
fn individual_and_aggregate_replacement_limits_leave_the_input_untouched() {
    let packet = owned();
    let cases = cases(&packet, false);
    let limits = CompoundLimits {
        maximum_stream_bytes: 7001,
        maximum_total_stream_bytes: 7040,
        ..LIMITS
    };
    let file = CompoundFile::from_bytes(cases[2].bytes, limits).unwrap();
    assert!(
        file.edit_stream(file.entry_at_path("S/B").unwrap().id(), &vec![7; 7002])
            .is_err()
    );
    assert!(
        file.edit_stream(file.entry_at_path("S/A").unwrap().id(), &[7; 40])
            .is_err()
    );
    assert_eq!(file.to_bytes(), cases[2].bytes);
}

#[test]
fn canonical_transaction_signature_and_version_four_header_padding_are_explicit() {
    let packet = owned();
    let cases = cases(&packet, false);
    let mut raw = cases[3].bytes.to_vec();
    raw[52..56].copy_from_slice(&0x12345678_u32.to_le_bytes());
    let file = CompoundFile::from_bytes(&raw, LIMITS).unwrap();
    assert_eq!(file.to_bytes(), raw);
    assert_eq!(word_at(&file.reconstructed_bytes().unwrap(), 52), 0);
    for offset in [8, 512, 4095] {
        let mut broken = raw.clone();
        broken[offset] = 0xa5;
        let error = CompoundFile::from_bytes(&broken, LIMITS).unwrap_err();
        assert_eq!(error.operation, "compound file");
        assert!(error.cause.contains("0xa5"));
    }
}
