//! Complete counted USD framing with opaque narrow bytes and raw UTF-16 units.
use bessemer::plastics::*;
use std::io::Read;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod packet;
use packet::Cursor;
#[path = "common/usd_packets.rs"]
mod checks;
#[path = "common/usd_export.rs"]
mod export;
#[path = "common/usd_failures.rs"]
mod failures;

fn inflate(bytes: &[u8]) -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(bytes)
        .read_to_end(&mut raw)
        .unwrap();
    raw
}

fn limits() -> UsdLimits {
    UsdLimits {
        maximum_records: 1000,
        maximum_payload_bytes: 1_000_000,
    }
}

fn quad_word(cursor: &mut Cursor<'_>) -> u64 {
    let low = u64::from(cursor.word());
    low | (u64::from(cursor.word()) << 32)
}

#[test]
fn all_literal_record_forms_match_actual_reader_metadata_and_complete_bytes() {
    let raw = inflate(include_bytes!("fixtures/plastics-usd/records.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"USD1");
    assert_eq!(cursor.word(), 49);
    for _ in 0..49 {
        assert_owned_record(&mut cursor);
    }
    assert!(cursor.0.is_empty());
}

fn assert_owned_record(cursor: &mut Cursor<'_>) {
    let encoded = cursor.span();
    let record = UsdStringRecord::from_bytes(encoded, limits().maximum_payload_bytes).unwrap();
    assert_eq!(record.to_bytes(), encoded);
    let mut expected = Cursor(cursor.span());
    assert_record_metadata(&record, &mut expected);
    let archive = UsdArchive::from_bytes(encoded, limits()).unwrap();
    assert_eq!(archive.record_count(), 1);
    assert_eq!(archive.to_bytes(), encoded);
    let id = archive.record_id(0).unwrap();
    assert_eq!(archive.record(&id).unwrap().to_bytes(), encoded);
    assert_eq!(archive.revision(), archive.clone().revision());
    assert!(archive.record_id(1).is_err());
    assert!(archive.record_id(usize::MAX).is_err());
    assert_raw_constructors(&record);
}

fn assert_record_metadata(record: &UsdStringRecord, cursor: &mut Cursor<'_>) {
    let offset = quad_word(cursor) as usize;
    let payload_offset = quad_word(cursor) as usize;
    let end = quad_word(cursor) as usize;
    let width = cursor.word() as usize;
    let count = quad_word(cursor);
    assert_eq!(record.source_offsets(), (offset, payload_offset, end));
    assert_eq!(record.character_width().byte_width(), width);
    assert_eq!(record.declared_count(), count);
    assert_eq!(record.header(), cursor.span());
    assert_eq!(record.payload(), cursor.span());
    assert!(cursor.0.is_empty());
}

fn assert_raw_constructors(record: &UsdStringRecord) {
    let created = match record.utf16_units() {
        Some(units) => UsdStringRecord::from_units(&units, limits().maximum_payload_bytes).unwrap(),
        None => {
            UsdStringRecord::from_narrow_bytes(record.payload(), limits().maximum_payload_bytes)
                .unwrap()
        }
    };
    assert_eq!(created.payload(), record.payload());
    assert_eq!(created.character_width(), record.character_width());
    assert_eq!(created.declared_count(), record.declared_count());
}

#[test]
fn all_complete_writer_layouts_match_actual_reader_identities_labels_and_source_bytes() {
    let raw = inflate(include_bytes!("fixtures/plastics-usd/layouts.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"USDL");
    assert_eq!(cursor.word(), 4);
    for _ in 0..4 {
        let adapters = cursor.word() as usize;
        let encoded = cursor.span();
        let archive = UsdArchive::from_bytes(encoded, limits()).unwrap();
        let layout = checks::layout(&archive, cursor.span());
        assert_eq!(layout.adapters.len(), adapters);
        assert_eq!(archive.to_bytes(), encoded);
        checks::check_every_layout_record(&archive, adapters);
    }
    assert!(cursor.0.is_empty());
}

#[test]
fn every_exact_record_edit_matches_independent_complete_bytes_and_actual_reader_boundaries() {
    let raw = inflate(include_bytes!("fixtures/plastics-usd/edits.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"USDE");
    assert_eq!(cursor.word(), 50);
    for _ in 0..50 {
        assert_owned_edit(&mut cursor);
    }
    assert!(cursor.0.is_empty());
}

fn assert_owned_edit(cursor: &mut Cursor<'_>) {
    let encoded = cursor.span();
    let selected = cursor.word() as usize;
    let replacement =
        UsdStringRecord::from_bytes(cursor.span(), limits().maximum_payload_bytes).unwrap();
    let expected = cursor.span();
    let archive = UsdArchive::from_bytes(encoded, limits()).unwrap();
    let edit = archive
        .edit_record(&archive.record_id(selected).unwrap(), &replacement)
        .unwrap();
    assert_eq!(edit.archive.to_bytes(), expected);
    checks::assert_archive(&edit.archive, cursor.span());
    checks::edit_contract(&edit, selected, expected != encoded);
    assert_eq!(archive.to_bytes(), encoded);
    assert_eq!(edit.source.to_bytes(), encoded);
}

#[test]
fn counted_eof_text_never_stops_complete_framing() {
    let raw = inflate(include_bytes!("fixtures/plastics-usd/after-eof.bin"));
    let archive = UsdArchive::from_bytes(&raw, limits()).unwrap();
    assert_eq!(archive.record_count(), 3);
    assert_eq!(archive.to_bytes(), raw);
    assert_eq!(
        archive.coverage().completion,
        UsdReadCompletion::CompletedRecords
    );
    let strict = UsdLimits {
        maximum_records: 2,
        ..limits()
    };
    assert!(UsdArchive::from_bytes(&raw, strict).is_err());
    let mut invalid = raw.clone();
    invalid.push(255);
    assert!(UsdArchive::from_bytes(&invalid, limits()).is_err());
}

#[test]
fn empty_input_completes_exactly_with_zero_limits_and_does_not_imply_writer_layout() {
    let strict = UsdLimits {
        maximum_records: 0,
        maximum_payload_bytes: 0,
    };
    let empty = UsdArchive::from_bytes(&[], strict).unwrap();
    assert_eq!(
        empty.coverage().completion,
        UsdReadCompletion::CompletedEmpty
    );
    assert_eq!(empty.coverage().checked_bytes, 0);
    assert_eq!(empty.coverage().checked_records, 0);
    assert!(empty.writer_layout().is_err());
}
