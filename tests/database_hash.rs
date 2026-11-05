//! Complete inline buffer bounds, independent bytes and actual native bucket edits (#224).
use bessemer::database_hash::*;
use bessemer::{Document, FailureClass};

const NATIVE: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
const STREAMS: [&str; 2] = [
    "Contents/3DExperienceExchange2",
    "Contents/3DExperienceExchange_ConfigHeader",
];

fn context(extent: usize, entries: usize) -> HashBufferContext {
    HashBufferContext {
        profile: HashBufferProfile::InstalledInline,
        engine_extent: extent,
        maximum_bytes: extent,
        maximum_entries: entries,
    }
}

fn literal_page(hash: u32, size: u16, next: u64, value: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0xa5; 96];
    bytes[..2].copy_from_slice(&20u16.to_be_bytes());
    bytes[2..4].copy_from_slice(&12u16.to_be_bytes());
    bytes[4..12].copy_from_slice(&next.to_be_bytes());
    bytes[12..14].fill(0);
    bytes[14..16].copy_from_slice(&size.to_be_bytes());
    bytes[20..24].copy_from_slice(&hash.to_be_bytes());
    bytes[24..28].copy_from_slice(&3u32.to_be_bytes());
    bytes[28..36].copy_from_slice(&(value.len() as u64).to_be_bytes());
    bytes[36..46].fill(0);
    bytes[46..49].copy_from_slice(b"k\0\xff");
    bytes[49..49 + value.len()].copy_from_slice(value);
    bytes
}

#[test]
fn literal_signed_boundaries_raw_free_sizes_and_opaque_values_reconstruct_exactly() {
    let packet = include_bytes!("fixtures/database-hash/cases.bin");
    assert_eq!(&packet[..8], b"HSH1\x3c\0\0\0");
    assert_eq!(packet.len(), 8 + 60 * 96);
    for (index, bytes) in packet[8..].as_chunks::<96>().0.iter().enumerate() {
        let hash = [0, 1, 0x80000000, u32::MAX][index / 15];
        let size = [0, 1, 3, 4, u16::MAX][(index / 3) % 5];
        let next = [0, 1, u64::MAX][index % 3];
        check_literal(bytes.to_vec(), hash, size, next);
        assert_eq!(
            bytes.as_slice(),
            literal_page(hash, size, next, b"\0\xff\x80k")
        );
    }
}

fn check_literal(bytes: Vec<u8>, hash: u32, size: u16, next: u64) {
    let buffer = HashPageBuffer::from_bytes(&bytes, context(96, 2)).unwrap();
    assert_literal_metadata(&buffer, size, next);
    let cell = &buffer.cells()[0];
    assert_eq!(
        (cell.offset(), cell.key_hash(), cell.next_offset()),
        (20, hash, 0)
    );
    assert_eq!(cell.key(), b"k\0\xff");
    assert_eq!(cell.value(), b"\0\xff\x80k");
    assert_eq!(buffer.to_bytes(), bytes);
    assert_eq!(buffer.coverage().interpreted_bytes, 49);
    assert_eq!(buffer.coverage().uninterpreted_bytes, 47);
}

fn assert_literal_metadata(buffer: &HashPageBuffer, size: u16, next: u64) {
    assert_eq!(
        buffer.header(),
        HashBufferHeader {
            cell_head: 20,
            free_head: 12,
            next_page: next
        }
    );
    assert_eq!(
        buffer.free_records(),
        &[HashFreeRecord {
            offset: 12,
            next: 0,
            size
        }]
    );
    assert_eq!(buffer.free_size_sum(), size);
}

#[test]
fn free_chain_totals_wrap_and_do_not_apply_the_update_writer_size_rule() {
    let mut bytes = literal_page(0, 65535, 0, b"x");
    bytes[12..14].copy_from_slice(&16u16.to_be_bytes());
    bytes[16..18].fill(0);
    bytes[18..20].copy_from_slice(&2u16.to_be_bytes());
    let buffer = HashPageBuffer::from_bytes(&bytes, context(96, 3)).unwrap();
    assert_eq!(buffer.free_records().len(), 2);
    assert_eq!(buffer.free_size_sum(), 1);
    assert_eq!(buffer.to_bytes(), bytes);
}

#[test]
fn empty_buffers_and_empty_values_are_completed_exact_noops() {
    let empty = HashPageBuffer::from_bytes(&[0; 12], context(12, 0)).unwrap();
    assert!(empty.cells().is_empty());
    assert!(empty.free_records().is_empty());
    assert_eq!(empty.free_size_sum(), 0);
    assert_eq!(empty.to_bytes(), [0; 12]);
    let buffer = HashPageBuffer::from_bytes(&literal_page(0, 0, 0, b""), context(96, 2)).unwrap();
    let edit = buffer.edit_value(20, b"").unwrap();
    assert_eq!(edit.completion(), HashBufferCompletion::Unchanged);
    assert_eq!(edit.history()[1].parent, 49..49);
    assert_eq!(edit.history()[1].relation, HashBufferRelation::Copied);
    assert_eq!(edit.buffer(), &buffer);
}

#[test]
fn empty_payloads_allow_an_adjacent_cell_at_the_same_boundary() {
    let mut bytes = literal_page(0, 4, 0, b"");
    bytes[24..28].fill(0);
    bytes[36..38].copy_from_slice(&46u16.to_be_bytes());
    bytes[46..72].fill(0);
    let buffer = HashPageBuffer::from_bytes(&bytes, context(96, 3)).unwrap();
    assert_eq!(buffer.cells().len(), 2);
    assert!(
        buffer
            .cells()
            .iter()
            .all(|cell| cell.key().is_empty() && cell.value().is_empty())
    );
    assert_eq!(buffer.coverage().interpreted_bytes, 68);
    assert_eq!(
        buffer.edit_value(20, b"").unwrap().buffer().to_bytes(),
        bytes
    );
}

#[test]
fn every_truncation_extra_byte_and_exact_budget_boundary_is_checked() {
    let raw = literal_page(0, 4, 0, b"word");
    for end in 0..96 {
        assert!(HashPageBuffer::from_bytes(&raw[..end], context(96, 2)).is_err());
    }
    assert!(HashPageBuffer::from_bytes(&[raw.clone(), vec![0]].concat(), context(96, 2)).is_err());
    assert!(HashPageBuffer::from_bytes(&raw, context(96, 1)).is_err());
    let mut limits = context(96, 2);
    limits.maximum_bytes = 95;
    assert!(HashPageBuffer::from_bytes(&raw, limits).is_err());
    assert!(HashPageBuffer::from_bytes(&raw, context(96, 2)).is_ok());
}

#[test]
fn cycles_overlaps_incomplete_records_and_impossible_lengths_never_publish() {
    for (range, replacement) in [
        (0..2, 95u16.to_be_bytes().to_vec()),
        (2..4, 1u16.to_be_bytes().to_vec()),
        (12..14, 12u16.to_be_bytes().to_vec()),
        (36..38, 20u16.to_be_bytes().to_vec()),
        (24..28, u32::MAX.to_be_bytes().to_vec()),
        (28..36, u64::MAX.to_be_bytes().to_vec()),
        (36..38, 12u16.to_be_bytes().to_vec()),
    ] {
        let mut raw = literal_page(0, 4, 0, b"word");
        raw[range].copy_from_slice(&replacement);
        let error = HashPageBuffer::from_bytes(&raw, context(96, 20)).unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
    }
}

#[test]
fn overflow_cells_are_explicitly_unsupported_without_partial_inline_results() {
    for overflow in [1u64, 0x8000000000000000, u64::MAX] {
        let mut raw = literal_page(0, 4, 0, b"word");
        raw[38..46].copy_from_slice(&overflow.to_be_bytes());
        let error = HashPageBuffer::from_bytes(&raw, context(96, 2)).unwrap_err();
        assert_eq!(error.class, FailureClass::Unsupported);
        assert!(error.cause.contains(&overflow.to_string()));
    }
}

fn check_edit(buffer: &HashPageBuffer, offset: u16, start: usize, replacement: &[u8]) {
    let source = buffer.to_bytes();
    let mut expected = source.clone();
    expected[start..start + replacement.len()].copy_from_slice(replacement);
    let edit = buffer.edit_value(offset, replacement).unwrap();
    assert_eq!(edit.source(), buffer);
    assert_edit_contract(&edit, &expected, start..start + replacement.len());
    let fresh = HashPageBuffer::from_bytes(&expected, context(source.len(), 100)).unwrap();
    let reverse = fresh
        .edit_value(offset, &source[start..start + replacement.len()])
        .unwrap();
    assert_eq!(reverse.buffer().to_bytes(), source);
    assert_eq!(buffer.to_bytes(), source);
}

fn assert_edit_contract(edit: &HashBufferEdit, expected: &[u8], range: std::ops::Range<usize>) {
    assert_eq!(edit.buffer().to_bytes(), expected);
    assert_eq!(edit.coverage().mode, HashBufferCheckMode::ExactInlineBuffer);
    assert_eq!(edit.coverage().achieved_byte_error, 0);
    assert_eq!(
        edit.completion(),
        if edit.source().to_bytes() == expected {
            HashBufferCompletion::Unchanged
        } else {
            HashBufferCompletion::Changed
        }
    );
    assert_history(edit, range);
}

fn assert_history(edit: &HashBufferEdit, value: std::ops::Range<usize>) {
    let end = edit.coverage().buffer_bytes;
    let spans = [0..value.start, value.clone(), value.end..end];
    for (index, span) in spans.into_iter().enumerate() {
        assert_eq!(edit.history()[index].parent, span);
        assert_eq!(edit.history()[index].result, span);
        let derived = index == 1 && edit.completion() == HashBufferCompletion::Changed;
        assert_eq!(
            edit.history()[index].relation,
            if derived {
                HashBufferRelation::Derived
            } else {
                HashBufferRelation::Copied
            }
        );
    }
    assert_diagnostics(edit);
}

fn assert_diagnostics(edit: &HashBufferEdit) {
    assert_eq!(
        edit.diagnostics(),
        [
            HashBufferDiagnostic::OpaqueValues,
            HashBufferDiagnostic::FreeSizesNotAllocatorValidation,
            HashBufferDiagnostic::PageGraphNotValidated,
            HashBufferDiagnostic::ProviderAssumed
        ]
    );
}

#[test]
fn value_edits_have_exact_bytes_history_reversals_and_failure_isolation() {
    let raw = literal_page(u32::MAX, 3, u64::MAX, b"word");
    let buffer = HashPageBuffer::from_bytes(&raw, context(96, 2)).unwrap();
    check_edit(&buffer, 20, 49, b"\0\xff\x80k");
    check_edit(&buffer, 20, 49, b"word");
    assert!(buffer.edit_value(0, b"word").is_err());
    assert!(buffer.edit_value(20, b"longer").is_err());
    assert_eq!(buffer.to_bytes(), raw);
}

#[test]
fn both_native_buckets_match_independent_cell_order_and_payload_spans() {
    let source = Document::from_bytes(NATIVE).unwrap();
    for (index, stream) in STREAMS.into_iter().enumerate() {
        let raw = source.read_stream(stream).unwrap();
        let page = &raw[8192..12288];
        let buffer = HashPageBuffer::from_bytes(page, context(4096, 8)).unwrap();
        let rows = native_rows(index);
        assert_eq!(buffer.cells().len(), rows.len());
        check_native_edits(&buffer, page, rows);
        assert_eq!(buffer.to_bytes(), page);
        assert_eq!(buffer.free_size_sum(), [1814, 3930][index]);
        assert_eq!(buffer.coverage().interpreted_bytes, [2290, 170][index]);
    }
}

type NativeCellOracle = (usize, u32, usize, usize, u16);

fn check_native_edits(buffer: &HashPageBuffer, page: &[u8], rows: Vec<NativeCellOracle>) {
    for (cell, row) in buffer.cells().iter().zip(rows) {
        check_native_cell(cell, page, row);
        check_edit(
            buffer,
            cell.offset(),
            usize::from(cell.offset()) + 26 + cell.key().len(),
            &vec![0x5a; cell.value().len()],
        );
    }
}

fn check_native_cell(cell: &HashInlineCell, page: &[u8], row: NativeCellOracle) {
    let (offset, hash, key, value, next) = row;
    assert_eq!(
        (cell.offset(), cell.key_hash(), cell.next_offset()),
        (offset as u16, hash, next)
    );
    assert_eq!(cell.key(), &page[offset + 26..offset + 26 + key]);
    assert_eq!(
        cell.value(),
        &page[offset + 26 + key..offset + 26 + key + value]
    );
}

fn native_rows(index: usize) -> Vec<NativeCellOracle> {
    if index == 1 {
        return vec![(12, 0x767f1f97, 36, 92, 0)];
    }
    vec![
        (2239, 0x5c01cd10, 36, 155, 1821),
        (1821, 0x0b8822ab, 3, 389, 1360),
        (1360, 0x1c071df4, 36, 399, 903),
        (903, 0x749eb64b, 36, 395, 442),
        (442, 0x9532ccdc, 36, 399, 186),
        (186, 0x520cd583, 36, 194, 0),
    ]
}

#[test]
fn native_value_replacements_reopen_complete_documents_and_preserve_unrelated_streams() {
    let source = Document::from_bytes(NATIVE).unwrap();
    for stream in STREAMS {
        let raw = source.read_stream(stream).unwrap();
        let replacement = native_replacement(&raw);
        let mut document = source.clone();
        document
            .replace_stream(stream, replacement.clone())
            .unwrap();
        let fresh = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
        assert_eq!(fresh.read_stream(stream).unwrap(), replacement);
        for entry in source.streams().filter(|entry| entry.name != stream) {
            assert_eq!(
                fresh.read_stream(entry.name).unwrap(),
                source.read_stream(entry.name).unwrap()
            );
        }
    }
    assert_eq!(source.to_bytes().unwrap(), NATIVE);
}

fn native_replacement(raw: &[u8]) -> Vec<u8> {
    let buffer = HashPageBuffer::from_bytes(&raw[8192..12288], context(4096, 8)).unwrap();
    let cell = &buffer.cells()[0];
    let edit = buffer
        .edit_value(cell.offset(), &vec![0x5a; cell.value().len()])
        .unwrap();
    let mut replacement = raw.to_vec();
    replacement[8192..12288].copy_from_slice(&edit.buffer().to_bytes());
    replacement
}

#[test]
fn immutable_results_own_sources_and_support_eight_shared_concurrent_edits() {
    assert_hash_types_send_sync();
    let mut raw = literal_page(1, 4, 0, b"word");
    let buffer = HashPageBuffer::from_bytes(&raw, context(96, 2)).unwrap();
    raw.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| check_edit(&buffer, 20, 49, b"test")))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert_eq!(buffer.cells()[0].value(), b"word");
}

fn assert_hash_types_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<HashPageBuffer>();
    check::<HashBufferEdit>();
    check::<HashBufferProfile>();
    check::<HashBufferContext>();
    check::<HashBufferHeader>();
    check::<HashFreeRecord>();
    check::<HashInlineCell>();
    check::<HashBufferCheckMode>();
    check::<HashBufferCoverage>();
    check::<HashBufferCompletion>();
    check::<HashBufferRelation>();
    check::<HashBufferHistory>();
    check::<HashBufferDiagnostic>();
}
