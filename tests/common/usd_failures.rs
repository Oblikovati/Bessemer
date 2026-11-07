use super::{Cursor, checks, inflate, limits};
use bessemer::plastics::*;

#[test]
fn every_small_record_prefix_and_all_large_header_or_terminal_truncations_fail() {
    let raw = inflate(include_bytes!("../fixtures/plastics-usd/records.bin"));
    let mut cursor = Cursor(&raw);
    cursor.take(4);
    assert_eq!(cursor.word(), 49);
    for _ in 0..49 {
        let encoded = cursor.span();
        cursor.span();
        assert_truncated_record(encoded);
    }
    assert!(cursor.0.is_empty());
}

fn assert_truncated_record(encoded: &[u8]) {
    let record = UsdStringRecord::from_bytes(encoded, limits().maximum_payload_bytes).unwrap();
    let lengths: Vec<_> = if encoded.len() < 1024 {
        (0..encoded.len()).collect()
    } else {
        (0..=record.header().len())
            .chain(std::iter::once(encoded.len() - 1))
            .collect()
    };
    for length in lengths {
        let error = UsdStringRecord::from_bytes(&encoded[..length], limits().maximum_payload_bytes)
            .unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert_eq!(error.operation, "decode Plastics USD");
    }
    if !record.payload().is_empty() {
        assert!(UsdStringRecord::from_bytes(encoded, record.payload().len() - 1).is_err());
    }
}

#[test]
fn escaped_u64_counts_reject_overflow_or_missing_payload_without_narrowing_or_allocation() {
    for count in [0xffffffffu64, 0x100000000, u64::MAX] {
        for wide in [false, true] {
            let mut raw = if wide {
                vec![0xff, 0xfe, 0xff]
            } else {
                Vec::new()
            };
            raw.extend_from_slice(&[0xff; 7]);
            raw.extend_from_slice(&count.to_le_bytes());
            for maximum in [0, 64 * 1024 * 1024, usize::MAX] {
                let error = UsdStringRecord::from_bytes(&raw, maximum).unwrap_err();
                assert_eq!(error.class, bessemer::FailureClass::Invalid);
                assert!(error.cause.contains("expected"));
            }
        }
    }
}

#[test]
fn standalone_records_construction_and_edits_enforce_their_own_explicit_limits() {
    let empty = UsdStringRecord::from_narrow_bytes(&[], 0).unwrap();
    let one = UsdStringRecord::from_narrow_bytes(&[0x80], 1).unwrap();
    let strict = UsdLimits {
        maximum_records: 1,
        maximum_payload_bytes: 0,
    };
    let archive = UsdArchive::from_records(std::slice::from_ref(&empty), strict).unwrap();
    assert!(UsdStringRecord::from_bytes(&[], 0).is_err());
    assert!(UsdStringRecord::from_bytes(&[0, 0], 0).is_err());
    assert!(UsdStringRecord::from_units(&[0], 1).is_err());
    assert!(UsdStringRecord::from_narrow_bytes(&[0], 0).is_err());
    assert!(UsdArchive::from_records(&[empty.clone(), empty.clone()], strict).is_err());
    assert!(UsdArchive::from_records(std::slice::from_ref(&one), strict).is_err());
    let id = archive.record_id(0).unwrap();
    assert!(archive.edit_record(&id, &one).is_err());
    let noop = archive.edit_record(&id, &empty).unwrap();
    checks::edit_contract(&noop, 0, false);
    assert_eq!(archive.to_bytes(), [0]);
}

#[test]
fn same_byte_foreign_revisions_and_bad_positions_do_not_alias_source_record_identities() {
    let first = UsdArchive::from_bytes(&[0], limits()).unwrap();
    let second = UsdArchive::from_bytes(&[0], limits()).unwrap();
    let id = first.record_id(0).unwrap();
    let replacement = UsdStringRecord::from_narrow_bytes(&[1], 1).unwrap();
    assert_ne!(first.revision(), second.revision());
    assert!(second.record(&id).is_err());
    assert!(second.edit_record(&id, &replacement).is_err());
    assert!(first.record_id(1).is_err());
    assert!(first.record_id(usize::MAX).is_err());
    let changed = first.edit_record(&id, &replacement).unwrap();
    assert!(changed.archive.record(&id).is_err());
    assert_eq!(first.to_bytes(), [0]);
    assert_eq!(second.to_bytes(), [0]);
}

#[test]
fn writer_layout_rejects_narrow_values_missing_tail_and_nul_extended_eof_without_relabeling() {
    let archive = checks::first_layout();
    let narrow = UsdStringRecord::from_narrow_bytes(&[0x80], 1).unwrap();
    let mixed = archive
        .edit_record(&archive.record_id(1).unwrap(), &narrow)
        .unwrap();
    assert!(mixed.archive.writer_layout().is_err());
    let extended = UsdStringRecord::from_units(&[35, 69, 79, 70, 0], 10).unwrap();
    let terminal = archive.record_id(archive.record_count() - 1).unwrap();
    assert!(
        archive
            .edit_record(&terminal, &extended)
            .unwrap()
            .archive
            .writer_layout()
            .is_err()
    );
    assert_missing_writer_tail(&archive);
    assert!(archive.writer_layout().is_ok());
}

#[test]
fn published_record_bytes_survive_input_mutation_source_drop_and_failed_followup_operations() {
    let edit = retained_edit();
    let expected = edit.archive.to_bytes();
    let source = edit.source.to_bytes();
    let mut returned = edit.archive.to_bytes();
    returned.fill(0);
    let foreign = UsdArchive::from_bytes(&[0], limits())
        .unwrap()
        .record_id(0)
        .unwrap();
    assert!(
        edit.archive
            .edit_record(&foreign, &edit.replacement)
            .is_err()
    );
    assert_eq!(edit.archive.to_bytes(), expected);
    assert_eq!(edit.source.to_bytes(), source);
    checks::edit_contract(&edit, 0, true);
}

fn retained_edit() -> UsdRecordEdit {
    let mut raw = vec![0];
    let source = UsdArchive::from_bytes(&raw, limits()).unwrap();
    let mut units = vec![0xd800, 0, 0x4e2d];
    let replacement = UsdStringRecord::from_units(&units, 6).unwrap();
    let edit = source
        .edit_record(&source.record_id(0).unwrap(), &replacement)
        .unwrap();
    raw.fill(255);
    units.fill(0);
    assert_eq!(source.to_bytes(), [0]);
    assert_eq!(edit.replacement.utf16_units().unwrap(), [0xd800, 0, 0x4e2d]);
    edit
}

#[test]
fn all_usd_public_types_are_send_sync_and_parallel_complete_edits_equal_serial_bytes() {
    assert_send_sync_types();
    let edit = retained_edit();
    let serial = edit.archive.to_bytes();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (source, replacement, serial) = (&edit.source, &edit.replacement, &serial);
            scope.spawn(move || repeat_edit(source, replacement, serial));
        }
    });
}

fn repeat_edit(source: &UsdArchive, replacement: &UsdStringRecord, serial: &[u8]) {
    for _ in 0..32 {
        let edit = source
            .edit_record(&source.record_id(0).unwrap(), replacement)
            .unwrap();
        assert_eq!(edit.archive.to_bytes(), serial);
        checks::edit_contract(&edit, 0, true);
    }
}

fn assert_send_sync_types() {
    fn check<T: Send + Sync>() {}
    check::<UsdArchive>();
    check::<UsdRevision>();
    check::<UsdRecordId>();
    check::<UsdStringRecord>();
    check::<UsdCharacterWidth>();
    check::<UsdLimits>();
    check::<UsdFramingCoverage>();
    check::<UsdReadCompletion>();
    check::<UsdCheckMode>();
    check::<UsdDiagnostic>();
    check::<UsdRecordEdit>();
    check::<UsdEditCompletion>();
    check::<UsdRecordRelation>();
    check::<UsdRecordHistory>();
    check::<UsdWriterLayout>();
    check::<UsdProfileRecord>();
    check::<UsdWriterCoverage>();
    check::<UsdWriterDiagnostic>();
}

fn assert_missing_writer_tail(archive: &UsdArchive) {
    let mut records = checks::records(archive);
    records.pop();
    assert!(
        UsdArchive::from_records(&records, limits())
            .unwrap()
            .writer_layout()
            .is_err()
    );
}

#[test]
fn parallel_writer_layouts_keep_all_record_identities_and_complete_projection_bytes() {
    let archive = checks::first_layout();
    let serial = checks::layout_bytes(&archive.writer_layout().unwrap());
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (archive, serial) = (&archive, &serial);
            scope.spawn(move || repeat_layout(archive, serial));
        }
    });
}

fn repeat_layout(archive: &UsdArchive, serial: &[u8]) {
    for _ in 0..32 {
        let layout = archive.writer_layout().unwrap();
        assert_eq!(checks::layout_bytes(&layout), serial);
        assert_eq!(&layout.parent, archive.revision());
        assert_eq!(layout.source.to_bytes(), archive.to_bytes());
    }
}

#[test]
fn borrowed_nonzero_offset_replacements_keep_noop_identity_and_exact_changed_bytes() {
    let source = UsdArchive::from_bytes(&[0, 1, 0x80, 0], limits()).unwrap();
    let id = source.record_id(1).unwrap();
    let replacement = source.record(&id).unwrap();
    assert_eq!(replacement.source_offsets(), (1, 2, 3));
    let noop = source.edit_record(&id, &replacement).unwrap();
    checks::edit_contract(&noop, 1, false);
    let changed = source
        .edit_record(&source.record_id(0).unwrap(), &replacement)
        .unwrap();
    assert_eq!(changed.archive.to_bytes(), [1, 0x80, 1, 0x80, 0]);
    checks::edit_contract(&changed, 0, true);
    assert_eq!(source.to_bytes(), [0, 1, 0x80, 0]);
}
