use super::{Cursor, assert_record_metadata, inflate, limits};
use bessemer::plastics::*;

pub fn assert_archive(archive: &UsdArchive, expected: &[u8]) {
    let mut cursor = Cursor(expected);
    assert_eq!(archive.record_count(), cursor.word() as usize);
    for index in 0..archive.record_count() {
        let record = archive.record(&archive.record_id(index).unwrap()).unwrap();
        assert_record_metadata(&record, &mut Cursor(cursor.span()));
    }
    assert!(cursor.0.is_empty());
    let coverage = archive.coverage();
    assert_eq!(coverage.checked_records, archive.record_count());
    assert_eq!(coverage.checked_bytes, archive.to_bytes().len());
    assert_eq!(
        coverage.check_mode,
        UsdCheckMode::ExactCompleteCountedRecords
    );
}

pub fn records(archive: &UsdArchive) -> Vec<UsdStringRecord> {
    (0..archive.record_count())
        .map(|index| archive.record(&archive.record_id(index).unwrap()).unwrap())
        .collect()
}

pub fn layout(archive: &UsdArchive, expected: &[u8]) -> UsdWriterLayout {
    let result = archive.writer_layout().unwrap();
    let mut cursor = Cursor(expected);
    assert_eq!(archive.record_count(), cursor.word() as usize);
    assert_eq!(result.adapters.len(), cursor.word() as usize);
    let ids = layout_ids(&result);
    assert_eq!(ids.len(), cursor.word() as usize);
    for id in ids {
        assert_eq!(id, &archive.record_id(cursor.word() as usize).unwrap());
    }
    for value in result.profile.iter() {
        let units: Vec<_> = value
            .label
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        assert_eq!(units, cursor.span());
    }
    assert!(cursor.0.is_empty());
    assert_layout_contract(archive, &result);
    result
}

fn layout_ids(layout: &UsdWriterLayout) -> Vec<&UsdRecordId> {
    std::iter::once(&layout.host)
        .chain(std::iter::once(&layout.volume_serial))
        .chain(layout.adapters.iter().flatten())
        .chain(layout.customer.iter())
        .chain(layout.profile.iter().map(|value| &value.record))
        .collect()
}

fn assert_layout_contract(archive: &UsdArchive, result: &UsdWriterLayout) {
    assert_eq!(&result.parent, archive.revision());
    assert_eq!(result.source.to_bytes(), archive.to_bytes());
    assert_eq!(result.source.revision(), archive.revision());
    let coverage = result.coverage;
    assert_eq!(
        coverage.check_mode,
        UsdCheckMode::ExactUnicodeWriterGrammarAndLiterals
    );
    assert_eq!(coverage.checked_records, archive.record_count());
    assert_eq!(coverage.checked_bytes, archive.to_bytes().len());
    assert_eq!(
        coverage.checked_literal_records,
        56 + result.adapters.len() * 3
    );
    assert_eq!(coverage.diagnostics, WRITER_DIAGNOSTICS);
}

pub fn edit_contract(edit: &UsdRecordEdit, selected: usize, changed: bool) {
    assert_eq!(edit.parent, edit.source.record_id(selected).unwrap());
    assert_eq!(edit.result, edit.archive.record_id(selected).unwrap());
    assert_eq!(&edit.parent_revision, edit.source.revision());
    assert_eq!(&edit.result_revision, edit.archive.revision());
    assert_eq!(edit.requested_pattern_error_bits, 0);
    assert_eq!(
        edit.coverage.check_mode,
        UsdCheckMode::ExactCompleteRecordsAndUnchangedRecordBytes
    );
    assert_eq!(edit.history.len(), edit.source.record_count());
    assert_edit_completion(edit, changed);
    assert_eq!(edit.parent_revision == edit.result_revision, !changed);
    for (index, relation) in edit.history.iter().enumerate() {
        assert_history_record(edit, relation, index, selected, changed);
    }
}

fn assert_history_record(
    edit: &UsdRecordEdit,
    relation: &UsdRecordHistory,
    index: usize,
    selected: usize,
    changed: bool,
) {
    assert_eq!(relation.parent, edit.source.record_id(index).unwrap());
    assert_eq!(relation.result, edit.archive.record_id(index).unwrap());
    let expected = if changed && index == selected {
        UsdRecordRelation::Derived
    } else {
        UsdRecordRelation::Copied
    };
    assert_eq!(relation.relation, expected);
    let source = if relation.relation == UsdRecordRelation::Derived {
        edit.replacement.clone()
    } else {
        edit.source.record(&relation.parent).unwrap()
    };
    assert_eq!(
        edit.archive.record(&relation.result).unwrap().to_bytes(),
        source.to_bytes()
    );
}

pub fn fixed_indices(adapters: usize, count: usize) -> Vec<usize> {
    let customer = 6 + adapters * 10;
    let profile = customer + 9;
    let mut indices = vec![
        0,
        2,
        3,
        5,
        customer,
        customer + 1,
        customer + 2,
        customer + 8,
        count - 1,
    ];
    append_fixed_groups(&mut indices, adapters, profile);
    indices.sort();
    assert_eq!(indices.len(), 56 + adapters * 3);
    indices
}

pub fn check_every_layout_record(archive: &UsdArchive, adapters: usize) {
    let fixed = fixed_indices(adapters, archive.record_count());
    let replacement = UsdStringRecord::from_units(&[0, 0xd800, 0xffff], 6).unwrap();
    let original = archive.to_bytes();
    for index in 0..archive.record_count() {
        let edit = archive
            .edit_record(&archive.record_id(index).unwrap(), &replacement)
            .unwrap();
        assert_eq!(
            edit.archive.writer_layout().is_err(),
            fixed.binary_search(&index).is_ok()
        );
        edit_contract(&edit, index, true);
    }
    let replayed = UsdArchive::from_records(&records(archive), limits()).unwrap();
    assert_eq!(replayed.to_bytes(), original);
    assert_ne!(replayed.revision(), archive.revision());
    assert_eq!(archive.to_bytes(), original);
}

pub fn append_words(output: &mut Vec<u8>, words: &[u32]) {
    output.extend(words.iter().flat_map(|word| word.to_le_bytes()));
}

pub fn append_span(output: &mut Vec<u8>, bytes: &[u8]) {
    append_words(output, &[bytes.len() as u32]);
    output.extend_from_slice(bytes);
}

pub fn record_bytes(record: &UsdStringRecord) -> Vec<u8> {
    let mut output = Vec::new();
    let (offset, payload, end) = record.source_offsets();
    for value in [offset, payload, end] {
        output.extend_from_slice(&(value as u64).to_le_bytes());
    }
    append_words(&mut output, &[record.character_width().byte_width() as u32]);
    output.extend_from_slice(&record.declared_count().to_le_bytes());
    append_span(&mut output, record.header());
    append_span(&mut output, record.payload());
    output
}

pub fn archive_bytes(archive: &UsdArchive) -> Vec<u8> {
    let mut output = Vec::new();
    append_words(&mut output, &[archive.record_count() as u32]);
    for record in records(archive) {
        append_span(&mut output, &record_bytes(&record));
    }
    output
}

pub fn layout_bytes(layout: &UsdWriterLayout) -> Vec<u8> {
    let mut output = Vec::new();
    append_words(&mut output, &layout_identity_words(layout));
    for value in layout.profile.iter() {
        let units: Vec<_> = value
            .label
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        append_span(&mut output, &units);
    }
    output
}

const WRITER_DIAGNOSTICS: [UsdWriterDiagnostic; 2] = [
    UsdWriterDiagnostic::ValuesNotSemanticallyValidated,
    UsdWriterDiagnostic::NativeLoaderNotExecuted,
];

pub fn first_layout() -> UsdArchive {
    let raw = inflate(include_bytes!("../fixtures/plastics-usd/layouts.bin"));
    let mut cursor = Cursor(&raw);
    cursor.take(12);
    UsdArchive::from_bytes(cursor.span(), limits()).unwrap()
}

fn assert_edit_completion(edit: &UsdRecordEdit, changed: bool) {
    let expected = if changed {
        UsdEditCompletion::Changed
    } else {
        UsdEditCompletion::Unchanged
    };
    assert_eq!(edit.completion, expected);
    assert_eq!(edit.coverage.checked_bytes, edit.archive.to_bytes().len());
    assert_eq!(edit.coverage.checked_records, edit.archive.record_count());
}

fn append_fixed_groups(indices: &mut Vec<usize>, adapters: usize, profile: usize) {
    for number in 0..adapters {
        indices.extend([6 + number * 10, 8 + number * 10, 15 + number * 10]);
    }
    for number in 0..12 {
        indices.extend([
            profile + number * 5,
            profile + number * 5 + 1,
            profile + number * 5 + 2,
        ]);
        if number < 11 {
            indices.push(profile + number * 5 + 4);
        }
    }
}

fn layout_identity_words(layout: &UsdWriterLayout) -> Vec<u32> {
    let ids = layout_ids(layout);
    let archive = &layout.source;
    let mut words = vec![
        archive.record_count() as u32,
        layout.adapters.len() as u32,
        ids.len() as u32,
    ];
    for id in ids {
        let index = (0..archive.record_count())
            .find(|index| archive.record_id(*index).unwrap() == *id)
            .unwrap();
        words.push(index as u32);
    }
    words
}
