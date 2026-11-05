use super::packet::{decode_pages, take, word};
use super::*;
#[path = "writer_bucket_census.rs"]
mod bucket_census;
#[path = "writer_bucket_documents.rs"]
mod bucket_documents;
use bessemer::database_hash::*;
use std::{collections::BTreeMap, io::Read};

struct ExpectedCell {
    offset: u16,
    hash: u32,
    next: u16,
    first: u64,
    key: Vec<u8>,
    value: Vec<u8>,
}
struct BucketCase {
    extent: usize,
    rejected: bool,
    raw: Vec<u8>,
    pages: Vec<OverflowPageImage>,
    free: Vec<HashFreeRecord>,
    cells: Vec<ExpectedCell>,
}

fn fixture_cases() -> Vec<BucketCase> {
    let compressed = include_bytes!("../fixtures/database-overflow/writer-buckets.bin.gz");
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(compressed.as_slice())
        .take(4_734_118)
        .read_to_end(&mut raw)
        .unwrap();
    assert_eq!(raw.len(), 4_734_117);
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"BWB1");
    assert_eq!(u32::from_le_bytes(word(&mut cursor)), 122);
    let cases = (0..122)
        .map(|_| {
            let count = u32::from_le_bytes(word(&mut cursor)) as usize;
            decode_bucket(take(&mut cursor, count))
        })
        .collect();
    assert!(cursor.is_empty());
    cases
}

fn decode_bucket(mut cursor: &[u8]) -> BucketCase {
    let extent = u32::from_le_bytes(word(&mut cursor)) as usize;
    let rejected = take(&mut cursor, 1)[0] != 0;
    let page_count = u32::from_le_bytes(word(&mut cursor)) as usize;
    let free_count = u32::from_le_bytes(word(&mut cursor)) as usize;
    let cell_count = u32::from_le_bytes(word(&mut cursor)) as usize;
    let raw = take(&mut cursor, extent).to_vec();
    let pages = decode_pages(&mut cursor, page_count, extent);
    let free = expected_free(&mut cursor, free_count);
    let cells = expected_cells(&mut cursor, cell_count);
    BucketCase {
        extent,
        rejected,
        raw,
        pages,
        free,
        cells,
    }
}

fn expected_free(cursor: &mut &[u8], count: usize) -> Vec<HashFreeRecord> {
    (0..count)
        .map(|_| HashFreeRecord {
            offset: u16::from_le_bytes(word(cursor)),
            next: u16::from_le_bytes(word(cursor)),
            size: u16::from_le_bytes(word(cursor)),
        })
        .collect()
}
fn expected_cells(cursor: &mut &[u8], count: usize) -> Vec<ExpectedCell> {
    let cells = (0..count).map(|_| expected_cell(cursor)).collect();
    assert!(cursor.is_empty());
    cells
}

fn expected_cell(cursor: &mut &[u8]) -> ExpectedCell {
    let offset = u16::from_le_bytes(word(cursor));
    let hash = u32::from_le_bytes(word(cursor));
    let next = u16::from_le_bytes(word(cursor));
    let first = u64::from_le_bytes(word(cursor));
    let key_length = u32::from_le_bytes(word(cursor)) as usize;
    let value_length = u64::from_le_bytes(word(cursor)) as usize;
    ExpectedCell {
        offset,
        hash,
        next,
        first,
        key: take(cursor, key_length).to_vec(),
        value: take(cursor, value_length).to_vec(),
    }
}

fn bucket_context(case: &BucketCase) -> OverflowStorageContext {
    OverflowStorageContext {
        profile: OverflowStorageProfile::InstalledWriter,
        engine_extent: case.extent,
        maximum_bytes: case.extent * (case.pages.len() + 1),
        maximum_pages: case.pages.len() + 1,
        maximum_payload_bytes: case
            .cells
            .iter()
            .map(|cell| cell.key.len() + cell.value.len())
            .sum(),
    }
}

fn parse_bucket(case: &BucketCase) -> bessemer::Result<HashWriterBucket> {
    HashWriterBucket::from_bytes(
        &case.raw,
        &case.pages,
        bucket_context(case),
        case.free.len() + case.cells.len(),
    )
}

fn storage_payload_sizes(case: &BucketCase) -> (usize, usize) {
    let mut inline = 0;
    let mut overflow = 0;
    for cell in &case.cells {
        let payload = cell.key.len() + cell.value.len();
        if cell.first == 0 {
            inline += payload;
        } else {
            overflow += 10 + payload;
        }
    }
    (inline, overflow)
}

fn check_cells(case: &BucketCase, bucket: &HashWriterBucket) {
    assert_eq!(bucket.cells().len(), case.cells.len());
    assert_eq!(bucket.free_records(), case.free);
    assert_eq!(bucket.to_bytes(), case.raw);
    let mut pages = case.pages.clone();
    pages.sort_by_key(|page| page.identifier);
    assert_eq!(bucket.to_overflow_pages(), pages);
    for (actual, expected) in bucket.cells().iter().zip(&case.cells) {
        check_cell(actual, expected);
    }
    check_bucket_coverage(case, bucket);
}

fn check_bucket_coverage(case: &BucketCase, bucket: &HashWriterBucket) {
    assert_eq!(bucket.coverage(), expected_coverage(case));
    assert_eq!(
        bucket.header(),
        HashBufferHeader {
            cell_head: u16::from_be_bytes(case.raw[..2].try_into().unwrap()),
            free_head: u16::from_be_bytes(case.raw[2..4].try_into().unwrap()),
            next_page: u64::from_be_bytes(case.raw[4..12].try_into().unwrap())
        }
    );
}

fn expected_coverage(case: &BucketCase) -> HashWriterCoverage {
    let (inline, overflow) = storage_payload_sizes(case);
    let interpreted =
        12 + 4 * case.free.len() + 26 * case.cells.len() + inline + overflow + 8 * case.pages.len();
    HashWriterCoverage {
        mode: HashWriterCheckMode::ExactSuppliedWriterBucket,
        buffer_bytes: bucket_context(case).maximum_bytes,
        interpreted_bytes: interpreted,
        uninterpreted_bytes: bucket_context(case).maximum_bytes - interpreted,
        pages: case.pages.len() + 1,
        cells: case.cells.len(),
        achieved_byte_error: 0,
    }
}

fn check_cell(actual: &HashWriterCell, expected: &ExpectedCell) {
    assert_eq!(
        (actual.offset(), actual.key_hash(), actual.next_offset()),
        (expected.offset, expected.hash, expected.next)
    );
    assert_eq!(
        actual
            .overflow_identifier()
            .map(NonZeroU64::get)
            .unwrap_or(0),
        expected.first
    );
    assert_eq!(
        (actual.key(), actual.value()),
        (expected.key.as_slice(), expected.value.as_slice())
    );
}
fn check_inline_agreement(case: &BucketCase, bucket: &HashWriterBucket) {
    if !case.pages.is_empty() {
        return;
    }
    let inline = HashPageBuffer::from_bytes(
        &case.raw,
        HashBufferContext {
            profile: HashBufferProfile::InstalledInline,
            engine_extent: case.extent,
            maximum_bytes: case.extent,
            maximum_entries: case.free.len() + case.cells.len(),
        },
    )
    .unwrap();
    assert_eq!(inline.to_bytes(), bucket.to_bytes());
}

#[test]
fn complete_owned_controls_and_native_mixed_buckets_match_every_independent_cell() {
    let mut admitted = 0;
    for case in fixture_cases() {
        if case.rejected {
            assert!(parse_bucket(&case).is_err());
            continue;
        }
        let bucket = parse_bucket(&case).unwrap();
        check_cells(&case, &bucket);
        check_inline_agreement(&case, &bucket);
        admitted += 1;
    }
    assert_eq!(admitted, 116);
}

fn selected_pages(case: &BucketCase, first: u64) -> Vec<OverflowPageImage> {
    let lookup: BTreeMap<_, _> = case
        .pages
        .iter()
        .map(|page| (page.identifier.get(), page))
        .collect();
    let mut result = Vec::new();
    let mut current = first;
    while current != 0 {
        assert!(result.len() < case.pages.len());
        let page = lookup[&current];
        result.push(page.clone());
        current = u64::from_be_bytes(page.bytes[..8].try_into().unwrap());
    }
    result
}

fn expected_bucket_edit(
    case: &BucketCase,
    cell: &ExpectedCell,
    value: &[u8],
) -> (Vec<u8>, Vec<OverflowPageImage>) {
    let mut raw = case.raw.clone();
    let mut pages = case.pages.clone();
    if cell.first == 0 {
        let start = usize::from(cell.offset) + 26 + cell.key.len();
        raw[start..start + value.len()].copy_from_slice(value);
    } else {
        replace_expected_overflow(case, cell, value, &mut pages);
    }
    pages.sort_by_key(|page| page.identifier);
    (raw, pages)
}

fn replace_expected_overflow(
    case: &BucketCase,
    cell: &ExpectedCell,
    value: &[u8],
    pages: &mut [OverflowPageImage],
) {
    let selected = OwnedOverflowCase {
        extent: case.extent,
        pages: selected_pages(case, cell.first),
        key: cell.key.clone(),
        value: cell.value.clone(),
        wrapped: false,
    };
    for expected in expected_pages(&selected, value) {
        let identifier = expected.identifier;
        *pages
            .iter_mut()
            .find(|page| page.identifier == identifier)
            .unwrap() = expected;
    }
}

fn check_bucket_edit(case: &BucketCase, source: &HashWriterBucket, cell: &ExpectedCell) {
    let replacement: Vec<_> = cell.value.iter().map(|byte| byte ^ 0x5a).collect();
    let edit = source.edit_value(cell.offset, &replacement).unwrap();
    let (expected_raw, expected_pages) = expected_bucket_edit(case, cell, &replacement);
    assert_eq!(edit.result().to_bytes(), expected_raw);
    assert_eq!(edit.result().to_overflow_pages(), expected_pages);
    assert_eq!(edit.source(), source);
    assert_eq!(edit.coverage(), source.coverage());
    assert_eq!(edit.diagnostics().len(), 4);
    let fresh = HashWriterBucket::from_bytes(
        &expected_raw,
        &expected_pages,
        bucket_context(case),
        case.free.len() + case.cells.len(),
    )
    .unwrap();
    assert_eq!(
        fresh.edit_value(cell.offset, &cell.value).unwrap().result(),
        source
    );
    check_writer_history(source, &edit);
}

#[test]
fn every_inline_and_overflow_value_edit_matches_complete_images_reopens_and_reverses() {
    let mut count = 0;
    for case in fixture_cases().iter().filter(|case| !case.rejected) {
        let source = parse_bucket(case).unwrap();
        for cell in &case.cells {
            check_bucket_edit(case, &source, cell);
            count += 1;
        }
        assert_eq!(source.to_bytes(), case.raw);
    }
    assert_eq!(count, 259);
}

fn check_writer_history(source: &HashWriterBucket, edit: &HashWriterEdit) {
    let mut spans: BTreeMap<Option<NonZeroU64>, usize> = BTreeMap::new();
    let mut previous = None;
    for row in edit.history() {
        let (id, span) = match row {
            HashWriterHistory::Bucket(span) => (None, span),
            HashWriterHistory::Overflow(row) => (Some(row.identifier), &row.span),
        };
        assert!(previous <= id);
        previous = id;
        assert_eq!(span.parent, span.result);
        check_copied_writer_span(source, edit, id, span);
        assert_eq!(*spans.entry(id).or_default(), span.parent.start);
        spans.insert(id, span.parent.end);
    }
    assert_eq!(spans.len(), source.coverage().pages);
    assert!(spans.values().all(|end| *end == source.to_bytes().len()));
}

fn writer_image(bucket: &HashWriterBucket, id: Option<NonZeroU64>) -> Vec<u8> {
    match id {
        None => bucket.to_bytes(),
        Some(id) => {
            bucket
                .to_overflow_pages()
                .into_iter()
                .find(|page| page.identifier == id)
                .unwrap()
                .bytes
        }
    }
}

fn check_copied_writer_span(
    source: &HashWriterBucket,
    edit: &HashWriterEdit,
    id: Option<NonZeroU64>,
    span: &HashBufferHistory,
) {
    if edit.completion() == HashBufferCompletion::Unchanged {
        assert_eq!(span.relation, HashBufferRelation::Copied);
    }
    if span.relation != HashBufferRelation::Copied {
        return;
    }
    assert_eq!(
        writer_image(source, id)[span.parent.clone()],
        writer_image(edit.result(), id)[span.result.clone()]
    );
}

#[test]
fn malformed_bucket_extents_cycles_overlap_and_count_overflow_reject_without_mutating_inputs() {
    let mut case = fixture_cases().remove(0);
    let original = case.raw.clone();
    for length in 0..512 {
        case.raw = original[..length].to_vec();
        assert!(parse_bucket(&case).is_err());
    }
    case.raw = original.clone();
    case.raw[36..38].copy_from_slice(&20u16.to_be_bytes());
    assert!(parse_bucket(&case).is_err());
    case.raw = original.clone();
    case.raw[2..4].copy_from_slice(&20u16.to_be_bytes());
    assert!(parse_bucket(&case).is_err());
    case.raw = original.clone();
    case.raw[24..28].copy_from_slice(&u32::MAX.to_be_bytes());
    case.raw[28..36].copy_from_slice(&u64::MAX.to_be_bytes());
    assert!(parse_bucket(&case).is_err());
    case.raw = original;
    case.pages[0].bytes.pop();
    assert!(parse_bucket(&case).is_err());
}

#[test]
fn aggregate_byte_page_entry_and_payload_limits_accept_exactly_and_reject_one_below() {
    let cases = fixture_cases();
    let case = &cases[121];
    let exact = bucket_context(case);
    let entries = case.free.len() + case.cells.len();
    assert!(HashWriterBucket::from_bytes(&case.raw, &case.pages, exact, entries).is_ok());
    let mut short = exact;
    short.maximum_bytes -= 1;
    assert!(HashWriterBucket::from_bytes(&case.raw, &case.pages, short, entries).is_err());
    short = exact;
    short.maximum_pages -= 1;
    assert!(HashWriterBucket::from_bytes(&case.raw, &case.pages, short, entries).is_err());
    short = exact;
    short.maximum_payload_bytes -= 1;
    assert!(HashWriterBucket::from_bytes(&case.raw, &case.pages, short, entries).is_err());
    assert!(HashWriterBucket::from_bytes(&case.raw, &case.pages, exact, entries - 1).is_err());
}

#[test]
fn missing_duplicate_unused_and_shared_overflow_ownership_reject_complete_bucket_publication() {
    let mut case = fixture_cases().remove(121);
    let original = case.pages.clone();
    case.pages.pop();
    assert!(parse_bucket(&case).is_err());
    case.pages = original.clone();
    case.pages.push(original[0].clone());
    assert!(parse_bucket(&case).is_err());
    case.pages = original;
    case.pages.push(OverflowPageImage {
        identifier: identity(99),
        bytes: vec![0; 4096],
    });
    assert!(parse_bucket(&case).is_err());
    case.pages.pop();
    case.raw[64 + 18..64 + 26].copy_from_slice(&4u64.to_be_bytes());
    assert!(parse_bucket(&case).is_err());
}

#[test]
fn source_inputs_noops_and_failed_replacements_preserve_all_owned_bytes() {
    let mut case = fixture_cases().remove(121);
    let source = parse_bucket(&case).unwrap();
    let checkpoint = source.clone();
    case.raw.fill(0);
    case.pages[0].bytes.fill(0);
    assert_eq!(source, checkpoint);
    assert!(source.edit_value(65535, &[]).is_err());
    let cell = &source.cells()[0];
    assert!(source.edit_value(cell.offset(), &[]).is_err());
    let noop = source.edit_value(cell.offset(), cell.value()).unwrap();
    assert_eq!(noop.completion(), HashBufferCompletion::Unchanged);
    assert_eq!(noop.result(), &source);
    check_writer_history(&source, &noop);
    assert_eq!(source, checkpoint);
}

#[test]
fn mixed_bucket_public_types_are_send_and_sync() {
    assert_send_sync::<HashWriterBucket>();
    assert_send_sync::<HashWriterCell>();
    assert_send_sync::<HashWriterEdit>();
    assert_send_sync::<HashWriterCoverage>();
    assert_send_sync::<HashWriterCheckMode>();
    assert_send_sync::<HashWriterHistory>();
}

fn mixed_worker(
    source: Arc<HashWriterBucket>,
    index: usize,
) -> std::thread::JoinHandle<HashWriterBucket> {
    std::thread::spawn(move || {
        let cell = &source.cells()[index];
        let value: Vec<_> = cell.value().iter().map(|b| b ^ 0x5a).collect();
        source
            .edit_value(cell.offset(), &value)
            .unwrap()
            .result()
            .clone()
    })
}

#[test]
fn shared_mixed_bucket_requests_match_independent_images() {
    let cases = fixture_cases();
    let case = &cases[121];
    let source = Arc::new(parse_bucket(case).unwrap());
    let workers: Vec<_> = (0..8)
        .map(|index| mixed_worker(source.clone(), index))
        .collect();
    for (index, worker) in workers.into_iter().enumerate() {
        let cell = &case.cells[index];
        let value: Vec<_> = cell.value.iter().map(|b| b ^ 0x5a).collect();
        let actual = worker.join().unwrap();
        assert_eq!(
            (actual.to_bytes(), actual.to_overflow_pages()),
            expected_bucket_edit(case, cell, &value)
        );
    }
}
