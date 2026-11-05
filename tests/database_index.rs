//! Complete conditional hash file graphs, independent edits and fresh reversals.
use bessemer::database_hash::{HashBufferCompletion, HashBufferRelation};
use bessemer::database_index::*;
use bessemer::{FailureClass, Result};
use std::{io::Read, num::NonZeroU64};
#[path = "database_index/documents.rs"]
mod documents;
#[path = "database_index/hash_vectors.rs"]
mod hash_vectors;
#[path = "database_index/native_corpus.rs"]
mod native_corpus;
#[path = "database_index/typed_errors.rs"]
mod typed_errors;

const OWNED_ROLES: [HashFilePageRole; 10] = [
    HashFilePageRole::Header,
    HashFilePageRole::Index,
    HashFilePageRole::Bucket(0),
    HashFilePageRole::Index,
    HashFilePageRole::Free,
    HashFilePageRole::Bucket(1),
    HashFilePageRole::Bucket(0),
    HashFilePageRole::Free,
    HashFilePageRole::Overflow,
    HashFilePageRole::Overflow,
];
const OWNED_MAPPINGS: [HashFileMapEntry; 2] = [
    HashFileMapEntry {
        logical: 0,
        physical: 2,
    },
    HashFileMapEntry {
        logical: 1,
        physical: 5,
    },
];
const CORRUPT_OFFSETS: [usize; 11] = [
    0,
    15,
    19,
    23,
    25,
    4096,
    4100,
    4096 + 40,
    3 * 4096 + 8,
    2 * 4096 + 12,
    2 * 4096 + 38,
];

struct FileCase {
    page: NonZeroU64,
    offset: u16,
    key: Vec<u8>,
    value: Vec<u8>,
    replacement: Vec<u8>,
    spans: Vec<std::ops::Range<usize>>,
    expected: Vec<u8>,
}

fn take<'a>(cursor: &mut &'a [u8], count: usize) -> &'a [u8] {
    let (head, tail) = cursor.split_at(count);
    *cursor = tail;
    head
}

fn word<const N: usize>(cursor: &mut &[u8]) -> [u8; N] {
    take(cursor, N).try_into().unwrap()
}

fn cases() -> (Vec<u8>, Vec<FileCase>) {
    let compressed = include_bytes!("fixtures/database-index/complete-file.bin.gz");
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(compressed.as_slice())
        .take(300_000)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"BFI1");
    let length = u32::from_le_bytes(word(&mut cursor)) as usize;
    assert_eq!(u32::from_le_bytes(word(&mut cursor)), 4);
    let original = take(&mut cursor, length).to_vec();
    let cases = (0..4)
        .map(|_| {
            let length = u32::from_le_bytes(word(&mut cursor)) as usize;
            decode_case(take(&mut cursor, length), original.len())
        })
        .collect();
    assert!(cursor.is_empty());
    (original, cases)
}

fn decode_case(mut cursor: &[u8], length: usize) -> FileCase {
    let page = NonZeroU64::new(u64::from_le_bytes(word(&mut cursor))).unwrap();
    let offset = u16::from_le_bytes(word(&mut cursor));
    let key_length = u32::from_le_bytes(word(&mut cursor)) as usize;
    let value_length = u32::from_le_bytes(word(&mut cursor)) as usize;
    let key = take(&mut cursor, key_length).to_vec();
    let value = take(&mut cursor, value_length).to_vec();
    let replacement = take(&mut cursor, value_length).to_vec();
    let spans = decode_spans(&mut cursor);
    let expected = take(&mut cursor, length).to_vec();
    assert!(cursor.is_empty());
    FileCase {
        page,
        offset,
        key,
        value,
        replacement,
        spans,
        expected,
    }
}

fn decode_spans(cursor: &mut &[u8]) -> Vec<std::ops::Range<usize>> {
    let count = u32::from_le_bytes(word(cursor));
    (0..count)
        .map(|_| {
            let start = u64::from_le_bytes(word(cursor)) as usize;
            start..u64::from_le_bytes(word(cursor)) as usize
        })
        .collect()
}

fn context(length: usize, extent: usize) -> HashFileContext {
    HashFileContext {
        profile: HashFileProfile::DefaultWindowsWriter,
        engine_extent: extent,
        pager_extent: extent,
        maximum_bytes: length,
        maximum_pages: length / extent,
        maximum_records: length / 4,
        maximum_payload_bytes: length,
    }
}

fn parse(raw: &[u8]) -> HashFileGraph {
    HashFileGraph::from_bytes(raw, context(raw.len(), 4096)).unwrap()
}

#[test]
fn complete_owned_file_checks_all_roles_hashes_routing_and_opaque_accounting() {
    let (raw, _) = cases();
    let graph = parse(&raw);
    assert_eq!(graph.to_bytes(), raw);
    assert_eq!(graph.mappings(), &OWNED_MAPPINGS);
    assert_eq!(graph.page_roles(), &OWNED_ROLES);
    assert!(graph.bucket(4).is_none());
    let coverage = graph.coverage();
    assert_eq!(coverage.mode, HashFileCheckMode::ExactSuppliedFileGraph);
    assert_eq!(
        (coverage.pages, coverage.cells, coverage.interpreted_bytes),
        (10, 4, 5326)
    );
    assert_eq!(coverage.buffer_bytes, raw.len());
    assert_eq!(coverage.uninterpreted_bytes, raw.len() - 5326);
    assert_eq!(coverage.achieved_byte_error, 0);
}

#[test]
fn each_complete_file_edit_matches_independent_bytes_reparses_and_reverses() {
    let (raw, cases) = cases();
    let graph = parse(&raw);
    for case in cases {
        verify_case(&graph, &raw, &case);
    }
}

fn verify_case(graph: &HashFileGraph, raw: &[u8], case: &FileCase) {
    assert_cell(graph, case);
    let edit = graph
        .edit_value(case.page, case.offset, &case.replacement)
        .unwrap();
    assert_eq!(edit.result().to_bytes(), case.expected);
    assert_eq!(parse(&case.expected), *edit.result());
    assert_eq!(edit.source(), graph);
    assert_eq!(graph.to_bytes(), raw);
    assert_eq!(edit.coverage(), graph.coverage());
    assert_history(&edit, case);
    let reverse = edit
        .result()
        .edit_value(case.page, case.offset, &case.value)
        .unwrap();
    assert_eq!(reverse.result().to_bytes(), raw);
    assert_eq!(edit.result().to_bytes(), case.expected);
}

fn assert_cell(graph: &HashFileGraph, case: &FileCase) {
    let bucket = graph.bucket(case.page.get()).unwrap();
    let cell = bucket
        .cells()
        .iter()
        .find(|cell| cell.offset() == case.offset)
        .unwrap();
    assert_eq!(cell.key(), case.key);
    assert_eq!(cell.value(), case.value);
}

fn assert_history(edit: &HashFileEdit, case: &FileCase) {
    let source = edit.source().to_bytes();
    let result = edit.result().to_bytes();
    let mut end = 0;
    let mut derived = Vec::new();
    for span in edit.history() {
        assert_eq!(span.parent, span.result);
        assert_eq!(span.parent.start, end);
        end = span.parent.end;
        match span.relation {
            HashBufferRelation::Copied => {
                assert_eq!(source[span.parent.clone()], result[span.result.clone()])
            }
            HashBufferRelation::Derived => derived.push(span.result.clone()),
        }
    }
    assert_eq!(end, source.len());
    assert_eq!(derived, nonempty_spans(case));
}

fn nonempty_spans(case: &FileCase) -> Vec<std::ops::Range<usize>> {
    case.spans
        .iter()
        .filter(|span| !span.is_empty())
        .cloned()
        .collect()
}

#[test]
fn noops_and_empty_values_return_completed_unchanged_with_complete_history() {
    let (raw, cases) = cases();
    let graph = parse(&raw);
    for case in cases {
        let edit = graph
            .edit_value(case.page, case.offset, &case.value)
            .unwrap();
        assert_eq!(edit.completion(), HashBufferCompletion::Unchanged);
        assert_eq!(edit.result(), &graph);
        assert!(
            edit.history()
                .iter()
                .all(|span| span.relation == HashBufferRelation::Copied)
        );
        assert_diagnostics(&edit);
    }
}

fn assert_diagnostics(edit: &HashFileEdit) {
    assert_eq!(
        edit.diagnostics(),
        [
            HashFileDiagnostic::ProviderAssumed,
            HashFileDiagnostic::OpaqueValues,
            HashFileDiagnostic::NativeKeyReaderDiffers,
            HashFileDiagnostic::AllocationTransactionAndApplicationNotValidated
        ]
    );
}

#[test]
fn edits_reject_missing_buckets_cells_and_length_changes_without_mutating_source() {
    let (raw, cases) = cases();
    let graph = parse(&raw);
    let missing = graph
        .edit_value(NonZeroU64::new(4).unwrap(), 12, &[])
        .unwrap_err();
    assert_eq!(missing.class, FailureClass::Missing);
    assert!(missing.cause.contains('4'));
    assert!(graph.edit_value(cases[0].page, 999, &[]).is_err());
    for case in cases {
        let mut replacement = case.value;
        replacement.push(0);
        assert!(
            graph
                .edit_value(case.page, case.offset, &replacement)
                .is_err()
        );
    }
    assert_eq!(graph.to_bytes(), raw);
}

#[test]
fn all_aggregate_budgets_are_enforced_and_exact_limits_pass() {
    let (raw, cases) = cases();
    let mut exact = context(raw.len(), 4096);
    exact.maximum_records = 6;
    exact.maximum_payload_bytes = cases
        .iter()
        .map(|case| case.key.len() + case.value.len())
        .sum();
    HashFileGraph::from_bytes(&raw, exact).unwrap();
    let mut byte_limit = exact;
    byte_limit.maximum_bytes -= 1;
    let mut page_limit = exact;
    page_limit.maximum_pages -= 1;
    let mut record_limit = exact;
    record_limit.maximum_records -= 1;
    let mut payload_limit = exact;
    payload_limit.maximum_payload_bytes -= 1;
    for limits in [byte_limit, page_limit, record_limit, payload_limit] {
        assert!(HashFileGraph::from_bytes(&raw, limits).is_err());
    }
}

fn write_word(raw: &mut [u8], offset: usize, value: u64) {
    raw[offset..offset + 8].copy_from_slice(&value.to_be_bytes());
}

#[test]
fn cycles_cross_role_aliases_and_out_of_bounds_links_fail_without_partial_results() {
    let (raw, _) = cases();
    let changes = [
        (3 * 4096, 3),
        (7 * 4096, 4),
        (6 * 4096 + 4, 2),
        (9 * 4096, 8),
        (4096 + 8, 8),
        (3 * 4096 + 20, 2),
        (4096 + 32, 10),
        (2 * 4096 + 4, u64::MAX),
        (4096 + 8, 0),
    ];
    for (offset, value) in changes {
        let mut changed = raw.clone();
        write_word(&mut changed, offset, value);
        assert!(
            HashFileGraph::from_bytes(&changed, context(changed.len(), 4096)).is_err(),
            "{offset}/{value}"
        );
    }
}

#[test]
fn malformed_headers_maps_cells_keys_and_hashes_are_rejected() {
    let (raw, _) = cases();
    for offset in CORRUPT_OFFSETS {
        let mut changed = raw.clone();
        changed[offset] ^= 1;
        assert!(
            HashFileGraph::from_bytes(&changed, context(changed.len(), 4096)).is_err(),
            "{offset}"
        );
    }
    let mut missing = raw.clone();
    write_word(&mut missing, 3 * 4096 + 12, 0);
    assert!(HashFileGraph::from_bytes(&missing, context(missing.len(), 4096)).is_err());
}

#[test]
fn partial_extents_mismatched_context_and_extra_pages_are_rejected() {
    let (raw, _) = cases();
    for extent in [0, 511, 4095, 65537] {
        let mut selected = context(raw.len(), 4096);
        selected.pager_extent = extent;
        assert!(HashFileGraph::from_bytes(&raw, selected).is_err());
    }
    let mut selected = context(raw.len(), 4096);
    selected.engine_extent = 512;
    assert!(HashFileGraph::from_bytes(&raw, selected).is_err());
    assert!(HashFileGraph::from_bytes(&raw[..raw.len() - 1], context(raw.len(), 4096)).is_err());
    let mut extra = raw;
    extra.extend_from_slice(&[0; 4096]);
    assert!(HashFileGraph::from_bytes(&extra, context(extra.len(), 4096)).is_err());
}

fn initialized_empty(extent: usize) -> Vec<u8> {
    let mut raw = vec![0; extent * 2];
    raw[..11].copy_from_slice(b"unqlite\xdb\x7c\x27\x12");
    raw[15..19].copy_from_slice(&512u32.to_be_bytes());
    raw[19..23].copy_from_slice(&(extent as u32).to_be_bytes());
    raw[23..25].copy_from_slice(&4u16.to_be_bytes());
    raw[25..29].copy_from_slice(b"hash");
    raw[extent..extent + 4].copy_from_slice(&0xfa782dcbu32.to_be_bytes());
    raw[extent + 4..extent + 8].copy_from_slice(&0x03f523f5u32.to_be_bytes());
    write_word(&mut raw, extent + 24, 1);
    raw
}

#[test]
fn initialized_empty_results_and_non_power_of_two_physical_extents_are_explicit() {
    for extent in [512, 513, 4096, 65536] {
        let raw = initialized_empty(extent);
        let graph = HashFileGraph::from_bytes(&raw, context(raw.len(), extent)).unwrap();
        assert_eq!(graph.to_bytes(), raw);
        assert_eq!((graph.coverage().pages, graph.coverage().cells), (2, 0));
        assert!(graph.mappings().is_empty());
        assert_eq!(
            graph.page_roles(),
            &[HashFilePageRole::Header, HashFilePageRole::Index]
        );
    }
}

#[test]
fn snapshots_are_isolated_from_input_changes_and_concurrent_edits() {
    let (mut raw, mut cases) = cases();
    let graph = parse(&raw);
    let case = cases.remove(3);
    raw.fill(0);
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_bytes(&graph, &case)))
            .collect();
        for job in jobs {
            assert_eq!(job.join().unwrap(), case.expected);
        }
    });
    assert_ne!(graph.to_bytes(), raw);
    assert_eq!(
        graph.bucket(case.page.get()).unwrap().cells()[1].value(),
        case.value
    );
}

fn edited_bytes(graph: &HashFileGraph, case: &FileCase) -> Vec<u8> {
    graph
        .edit_value(case.page, case.offset, &case.replacement)
        .unwrap()
        .result()
        .to_bytes()
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn public_graph_types_are_send_and_sync() {
    assert_send_sync::<HashFileGraph>();
    assert_send_sync::<HashFileEdit>();
    assert_send_sync::<HashFileContext>();
    assert_send_sync::<HashFileProfile>();
    assert_send_sync::<HashFileMapEntry>();
    assert_send_sync::<HashFilePageRole>();
    assert_send_sync::<HashFileCoverage>();
    assert_send_sync::<HashFileCheckMode>();
    assert_send_sync::<HashFileDiagnostic>();
}

#[test]
fn retained_native_files_have_complete_graphs_and_editable_cells() -> Result<()> {
    for raw in [
        include_bytes!("fixtures/database-overflow/native-0.db").as_slice(),
        include_bytes!("fixtures/database-overflow/native-1.db").as_slice(),
        include_bytes!("fixtures/database-overflow/native-2.db").as_slice(),
    ] {
        let graph = HashFileGraph::from_bytes(raw, context(raw.len(), 4096))?;
        assert_eq!(graph.to_bytes(), raw);
        assert!(graph.coverage().cells > 0);
        for (physical, role) in graph.page_roles().iter().enumerate() {
            if matches!(role, HashFilePageRole::Bucket(_)) {
                check_native_bucket(&graph, physical as u64)?;
            }
        }
    }
    Ok(())
}

fn check_native_bucket(graph: &HashFileGraph, physical: u64) -> Result<()> {
    for cell in graph.bucket(physical).unwrap().cells() {
        let replacement: Vec<_> = cell.value().iter().map(|byte| byte ^ 0xa5).collect();
        let identity = NonZeroU64::new(physical).unwrap();
        let edit = graph.edit_value(identity, cell.offset(), &replacement)?;
        let reverse = edit
            .result()
            .edit_value(identity, cell.offset(), cell.value())?;
        assert_eq!(reverse.result(), graph);
    }
    Ok(())
}
