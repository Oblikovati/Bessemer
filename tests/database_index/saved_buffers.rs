use super::*;
#[path = "saved_corpus.rs"]
mod saved_corpus;
pub(super) use saved_corpus::verify_native_saved_index;

const SAVED_DIAGNOSTICS: [HashIndexDiagnostic; 4] = [
    HashIndexDiagnostic::ProviderAssumed,
    HashIndexDiagnostic::OpaqueRemainder,
    HashIndexDiagnostic::DuplicatePrecedenceNotInferred,
    HashIndexDiagnostic::FileGraphAndApplicationNotValidated,
];
const OUT_OF_FILE_PAIR: HashFileMapEntry = HashFileMapEntry {
    logical: u64::MAX,
    physical: 0,
};
const DUPLICATE_MAPPINGS: [HashFileMapEntry; 2] = [HashFileMapEntry {
    logical: 0,
    physical: 2,
}; 2];

fn noncanonical_header_words() -> HashIndexHeader {
    HashIndexHeader {
        free_head: u64::MAX,
        split: u64::MAX,
        maximum: 0,
    }
}

fn changed_header_bytes(raw: &[u8], extent: usize, words: HashIndexHeader) -> Vec<u8> {
    let mut expected = raw.to_vec();
    write_word(&mut expected, extent + 8, words.free_head);
    write_word(&mut expected, extent + 16, words.split);
    write_word(&mut expected, extent + 24, words.maximum);
    expected
}

#[test]
fn saved_index_edits_use_the_explicit_extent_including_non_power_of_two_pages() {
    for extent in [512, 513, 4096, 65536] {
        check_saved_extent(extent);
    }
}

fn check_saved_extent(extent: usize) {
    let raw = initialized_empty(extent);
    let limits = context(raw.len(), extent);
    let source = HashFileIndexBuffer::from_bytes(&raw, limits).unwrap();
    let words = noncanonical_header_words();
    let edit = source.edit_header(words).unwrap();
    assert_eq!(
        edit.result().to_bytes(),
        changed_header_bytes(&raw, extent, words)
    );
    assert_eq!(
        (edit.coverage().index_pages, edit.coverage().mappings),
        (1, 0)
    );
    let fresh = HashFileIndexBuffer::from_bytes(&edit.result().to_bytes(), limits).unwrap();
    assert_eq!(
        fresh.edit_header(source.header()).unwrap().result(),
        &source
    );
}

#[derive(Clone, Copy)]
enum SavedRequest {
    Header(HashIndexHeader),
    Mapping(usize, HashFileMapEntry),
}

struct SavedMutation {
    request: SavedRequest,
    span: std::ops::Range<usize>,
    expected: Vec<u8>,
}

fn saved_packet() -> Vec<u8> {
    let compressed = include_bytes!("../fixtures/database-index/saved-indexes.bin.gz");
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(compressed.as_slice())
        .take(1_717_659)
        .read_to_end(&mut raw)
        .unwrap();
    assert_eq!(raw.len(), 1_717_658);
    raw
}

fn saved_mutation(cursor: &mut &[u8], length: usize) -> SavedMutation {
    let kind = take(cursor, 1)[0];
    let ordinal = u32::from_le_bytes(word(cursor)) as usize;
    let request = saved_request(cursor, kind, ordinal);
    let start = u64::from_le_bytes(word(cursor)) as usize;
    let end = u64::from_le_bytes(word(cursor)) as usize;
    SavedMutation {
        request,
        span: start..end,
        expected: take(cursor, length).to_vec(),
    }
}

fn saved_request(cursor: &mut &[u8], kind: u8, ordinal: usize) -> SavedRequest {
    if kind == 0 {
        return SavedRequest::Header(HashIndexHeader {
            free_head: u64::from_le_bytes(word(cursor)),
            split: u64::from_le_bytes(word(cursor)),
            maximum: u64::from_le_bytes(word(cursor)),
        });
    }
    assert_eq!(kind, 1);
    SavedRequest::Mapping(
        ordinal,
        HashFileMapEntry {
            logical: u64::from_le_bytes(word(cursor)),
            physical: u64::from_le_bytes(word(cursor)),
        },
    )
}

fn parse_saved(raw: &[u8]) -> HashFileIndexBuffer {
    HashFileIndexBuffer::from_bytes(raw, context(raw.len(), 4096)).unwrap()
}

fn apply_saved(source: &HashFileIndexBuffer, request: SavedRequest) -> HashFileIndexEdit {
    match request {
        SavedRequest::Header(header) => source.edit_header(header).unwrap(),
        SavedRequest::Mapping(ordinal, entry) => source.edit_mapping(ordinal, entry).unwrap(),
    }
}

fn original_request(source: &HashFileIndexBuffer, request: SavedRequest) -> SavedRequest {
    match request {
        SavedRequest::Header(_) => SavedRequest::Header(source.header()),
        SavedRequest::Mapping(ordinal, _) => {
            SavedRequest::Mapping(ordinal, source.mappings()[ordinal])
        }
    }
}

fn verify_saved_mutation(source: &HashFileIndexBuffer, change: &SavedMutation) {
    let raw = source.to_bytes();
    let edit = apply_saved(source, change.request);
    assert_eq!(edit.source(), source);
    assert_eq!(edit.result().to_bytes(), change.expected);
    assert_eq!(edit.result(), &parse_saved(&change.expected));
    assert_eq!(edit.coverage(), source.coverage());
    assert_saved_history(&edit, change);
    let restored = apply_saved(
        &parse_saved(&change.expected),
        original_request(source, change.request),
    );
    assert_eq!(restored.result(), source);
    assert_eq!(source.to_bytes(), raw);
    assert_eq!(edit.result().to_bytes(), change.expected);
}

fn assert_saved_history(edit: &HashFileIndexEdit, change: &SavedMutation) {
    let raw = edit.source().to_bytes();
    let changed = raw != change.expected;
    let expected = [
        0..change.span.start,
        change.span.clone(),
        change.span.end..raw.len(),
    ];
    for (index, span) in edit.history().iter().enumerate() {
        assert_eq!(span.parent, expected[index]);
        assert_eq!(span.parent, span.result);
        let relation = if index == 1 && changed {
            HashBufferRelation::Derived
        } else {
            HashBufferRelation::Copied
        };
        assert_eq!(span.relation, relation);
    }
    assert_eq!(edit.completion() == HashBufferCompletion::Changed, changed);
}

#[test]
fn every_saved_header_and_pair_edit_matches_frozen_complete_bytes_and_fresh_reversal() {
    let packet = saved_packet();
    let mut cursor = packet.as_slice();
    assert_eq!(take(&mut cursor, 4), b"BSI1");
    assert_eq!(u32::from_le_bytes(word(&mut cursor)), 7);
    let mut edits = 0;
    for _ in 0..7 {
        let length = u32::from_le_bytes(word(&mut cursor)) as usize;
        let count = u32::from_le_bytes(word(&mut cursor));
        let source = parse_saved(take(&mut cursor, length));
        for _ in 0..count {
            verify_saved_mutation(&source, &saved_mutation(&mut cursor, length));
            edits += 1;
        }
    }
    assert_eq!(edits, 34);
    assert!(cursor.is_empty());
}

#[test]
fn duplicate_and_noncanonical_saved_indexes_do_not_claim_complete_graph_validity() {
    let (mut raw, _) = cases();
    write_word(&mut raw, 3 * 4096 + 12, 0);
    write_word(&mut raw, 3 * 4096 + 20, 2);
    let source = parse_saved(&raw);
    assert_eq!(source.mappings(), &DUPLICATE_MAPPINGS);
    assert!(HashFileGraph::from_bytes(&raw, context(raw.len(), 4096)).is_err());
    let header = noncanonical_header_words();
    let edit = source.edit_header(header).unwrap();
    assert_eq!(edit.result().header(), header);
    assert_eq!(edit.result().mappings(), source.mappings());
    assert_index_only_coverage(&edit, raw.len());
    assert!(
        HashFileGraph::from_bytes(&edit.result().to_bytes(), context(raw.len(), 4096)).is_err()
    );
}

fn assert_index_only_coverage(edit: &HashFileIndexEdit, length: usize) {
    assert_eq!(
        edit.coverage().mode,
        HashIndexCheckMode::ExactSavedIndexChain
    );
    assert_eq!(
        (edit.coverage().index_pages, edit.coverage().mappings),
        (2, 2)
    );
    assert_eq!(edit.coverage().interpreted_bytes, 117);
    assert_eq!(edit.coverage().uninterpreted_bytes, length - 117);
    assert_eq!(edit.coverage().achieved_byte_error, 0);
}

#[test]
fn saved_index_diagnostics_exclude_lookup_and_graph_checks_even_for_canonical_inputs() {
    let (raw, _) = cases();
    let source = parse_saved(&raw);
    let edit = source.edit_header(source.header()).unwrap();
    assert_eq!(edit.completion(), HashBufferCompletion::Unchanged);
    assert_eq!(edit.result(), &source);
    assert_eq!(edit.diagnostics(), SAVED_DIAGNOSTICS);
    let mut limits = context(raw.len(), 4096);
    limits.maximum_payload_bytes = 0;
    assert_eq!(
        HashFileIndexBuffer::from_bytes(&raw, limits)
            .unwrap()
            .to_bytes(),
        raw
    );
}

#[test]
fn saved_mapping_selectors_and_budgets_fail_without_mutating_source() {
    let (raw, _) = cases();
    let source = parse_saved(&raw);
    let error = source
        .edit_mapping(usize::MAX, source.mappings()[0])
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains(&usize::MAX.to_string()));
    let mut limits = context(raw.len(), 4096);
    limits.maximum_records = 1;
    assert!(HashFileIndexBuffer::from_bytes(&raw, limits).is_err());
    assert_eq!(source.to_bytes(), raw);
}

#[test]
fn saved_index_rejects_partial_map_pairs_cycles_and_bad_profile_headers() {
    let (raw, _) = cases();
    for offset in [0, 25, 4096, 4100, 4096 + 40, 3 * 4096 + 8] {
        let mut broken = raw.clone();
        broken[offset] ^= 1;
        let error = HashFileIndexBuffer::from_bytes(&broken, context(raw.len(), 4096)).unwrap_err();
        assert_eq!(error.operation, "saved index read");
        assert_eq!(error.class, FailureClass::Invalid);
    }
    let mut cyclic = raw;
    write_word(&mut cyclic, 3 * 4096, 3);
    assert!(HashFileIndexBuffer::from_bytes(&cyclic, context(cyclic.len(), 4096)).is_err());
}

#[test]
fn saved_index_edits_are_isolated_from_source_mutation_and_eight_concurrent_requests() {
    let (mut raw, _) = cases();
    let source = parse_saved(&raw);
    let request = SavedRequest::Mapping(1, OUT_OF_FILE_PAIR);
    let expected = apply_saved(&source, request).result().to_bytes();
    let original = source.to_bytes();
    raw.fill(0);
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| apply_saved(&source, request).result().to_bytes()))
            .collect();
        for job in jobs {
            assert_eq!(job.join().unwrap(), expected);
        }
    });
    assert_eq!(source.to_bytes(), original);
}
