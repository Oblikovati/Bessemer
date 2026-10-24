//! Bounded parsing and transactional validation of standalone history graphs.

use bessemer::FailureClass;
use bessemer::history::{DateCodeId, HistoryTail, StateDateCodes, VersionHistory};

const NATIVE: &[u8] = include_bytes!("fixtures/history/blank-part.bin");
const MIXED: &[u8] = include_bytes!("fixtures/history/mixed.bin");

#[test]
fn every_native_truncation_and_trailing_byte_is_rejected() {
    for bytes in [NATIVE, MIXED] {
        for end in 0..bytes.len() {
            let error = VersionHistory::from_bytes(&bytes[..end]).unwrap_err();
            assert_eq!(error.operation, "decode version history", "end {end}");
            assert!(error.entity.starts_with("History"));
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(VersionHistory::from_bytes(&trailing).is_err());
    }
}

#[test]
fn invalid_counts_are_rejected_before_allocating_state_arrays() {
    for offset in [24, 71] {
        for count in [0_i32, -1, i32::MAX] {
            let mut bytes = NATIVE.to_vec();
            bytes[offset..offset + 4].copy_from_slice(&count.to_le_bytes());
            let error = VersionHistory::from_bytes(&bytes).unwrap_err();
            assert_eq!(error.class, FailureClass::Invalid);
            assert!(error.cause.contains("expected"));
            assert!(error.cause.contains(&count.to_string()));
        }
    }
}

#[test]
fn object_pointer_kinds_and_unmapped_indices_are_checked() {
    for tag in [0_u16, 2, 3, 0x8002, 0x8000, 0x8063, 0x0063] {
        let mut bytes = NATIVE.to_vec();
        bytes[46..48].copy_from_slice(&tag.to_le_bytes());
        assert!(VersionHistory::from_bytes(&bytes).is_err(), "tag {tag:#x}");
    }
    let mut bad_name = NATIVE.to_vec();
    bad_name[6] = 0xff;
    assert_eq!(
        VersionHistory::from_bytes(&bad_name).unwrap_err().class,
        FailureClass::Invalid
    );
    bad_name[6] = b'x';
    assert_eq!(
        VersionHistory::from_bytes(&bad_name).unwrap_err().class,
        FailureClass::Unsupported
    );
}

#[test]
fn malformed_wide_string_is_not_replaced_lossily() {
    let mut bytes = NATIVE.to_vec();
    // The native empty comment occupies bytes 38..42; insert one unpaired surrogate.
    bytes[41] = 1;
    bytes.splice(42..42, [0x00, 0xd8]);
    let error = VersionHistory::from_bytes(&bytes).unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("UTF-16LE"));
    assert!(error.cause.contains("d800"));
}

#[test]
fn invalid_typed_graphs_leave_published_input_unchanged() {
    let source = VersionHistory::from_bytes(NATIVE).unwrap();
    let mut empty = source.contents().clone();
    empty.states.clear();
    assert!(source.with_contents(empty).is_err());
    let mut branch = source.contents().clone();
    branch.states[0].date_codes = StateDateCodes::Absent;
    assert!(source.with_contents(branch).is_err());
    let mut tail = source.contents().clone();
    tail.tail = HistoryTail::default();
    assert!(source.with_contents(tail).is_err());
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
}

#[test]
fn missing_empty_and_orphan_date_objects_are_rejected() {
    let source = VersionHistory::from_bytes(NATIVE).unwrap();
    let mut missing = source.contents().clone();
    missing.date_histories.clear();
    assert!(source.with_contents(missing).is_err());
    let mut empty = source.contents().clone();
    empty
        .date_histories
        .values_mut()
        .next()
        .unwrap()
        .codes
        .clear();
    assert!(source.with_contents(empty).is_err());
    let mut orphan = source.contents().clone();
    let record = orphan.date_histories.values().next().unwrap().clone();
    orphan
        .date_histories
        .insert(DateCodeId::new(99).unwrap(), record);
    assert!(source.with_contents(orphan).is_err());
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
}

#[test]
fn graph_identities_reject_reserved_and_out_of_range_values() {
    for identity in [0, 0x8000_0000, u32::MAX] {
        assert!(DateCodeId::new(identity).is_err());
    }
    assert_eq!(DateCodeId::new(0x7fff_ffff).unwrap().value(), 0x7fff_ffff);
}

#[test]
fn generated_mutations_never_panic_or_return_partial_histories() {
    for offset in 0..NATIVE.len() {
        for value in [0, 1, 127, 128, 255] {
            let mut bytes = NATIVE.to_vec();
            bytes[offset] = value;
            if let Ok(history) = VersionHistory::from_bytes(&bytes) {
                assert!(!history.contents().states.is_empty());
                assert_eq!(history.to_bytes().unwrap(), bytes);
            }
        }
    }
}
