//! Complete supplied writer chains, independent payload spans and isolated edits (#229).
use bessemer::FailureClass;
use bessemer::database_hash::{HashBufferCompletion, HashBufferRelation};
use bessemer::database_overflow::*;
use std::{num::NonZeroU64, sync::Arc};
#[path = "database_overflow/packet.rs"]
mod packet;
use packet::packet_cases;
#[path = "database_overflow/documents.rs"]
mod documents;
#[path = "database_overflow/native_census.rs"]
mod native_census;

#[derive(Clone)]
struct OwnedOverflowCase {
    extent: usize,
    pages: Vec<OverflowPageImage>,
    key: Vec<u8>,
    value: Vec<u8>,
    wrapped: bool,
}

fn identity(raw: u64) -> NonZeroU64 {
    NonZeroU64::new(raw).unwrap()
}

fn context(case: &OwnedOverflowCase) -> OverflowStorageContext {
    OverflowStorageContext {
        profile: OverflowStorageProfile::InstalledWriter,
        engine_extent: case.extent,
        maximum_bytes: case.extent * case.pages.len(),
        maximum_pages: case.pages.len(),
        maximum_payload_bytes: case.key.len() + case.value.len(),
    }
}

fn parse_case(case: &OwnedOverflowCase) -> bessemer::Result<OverflowPayload> {
    parse_limited(case, context(case))
}

fn parse_limited(
    case: &OwnedOverflowCase,
    limits: OverflowStorageContext,
) -> bessemer::Result<OverflowPayload> {
    OverflowPayload::from_pages(
        &case.pages,
        case.pages[0].identifier,
        case.key.len() as u32,
        case.value.len() as u64,
        limits,
    )
}

fn expected_pages(case: &OwnedOverflowCase, replacement: &[u8]) -> Vec<OverflowPageImage> {
    let mut pages = case.pages.clone();
    for (index, byte) in replacement.iter().enumerate() {
        let position = case.key.len() + index;
        let (page, offset) = if position < case.extent - 18 {
            (0, 18 + position)
        } else {
            let relative = position - (case.extent - 18);
            (
                1 + relative / (case.extent - 8),
                8 + relative % (case.extent - 8),
            )
        };
        pages[page].bytes[offset] = *byte;
    }
    pages.sort_by_key(|page| page.identifier);
    pages
}

fn assert_coverage(case: &OwnedOverflowCase, payload: &OverflowPayload) {
    let coverage = payload.coverage();
    let total = case.extent * case.pages.len();
    let interpreted = case.pages.len() * 8 + 10 + case.key.len() + case.value.len();
    assert_eq!(
        coverage,
        OverflowCoverage {
            mode: OverflowCheckMode::ExactSuppliedWriterChain,
            buffer_bytes: total,
            interpreted_bytes: interpreted,
            uninterpreted_bytes: total - interpreted,
            pages: case.pages.len(),
            achieved_byte_error: 0
        }
    );
}

#[test]
fn all_independent_writer_images_reconstruct_or_reject_exact_cursor_truncation() {
    let mut accepted = 0;
    for case in packet_cases() {
        if case.wrapped {
            let error = parse_case(&case).unwrap_err();
            assert_eq!(error.class, FailureClass::Invalid);
            assert!(error.cause.contains("without cursor truncation"));
            continue;
        }
        let payload = parse_case(&case).unwrap();
        assert_eq!(
            (payload.key(), payload.value()),
            (case.key.as_slice(), case.value.as_slice())
        );
        assert_eq!(payload.pages(), expected_pages(&case, &case.value));
        assert_coverage(&case, &payload);
        accepted += 1;
    }
    assert_eq!(accepted, 111);
}

fn check_value_edit(case: &OwnedOverflowCase) {
    let source = parse_case(case).unwrap();
    let replacement: Vec<_> = case.value.iter().map(|byte| byte ^ 0x5a).collect();
    let edit = source.edit_value(&replacement).unwrap();
    assert_eq!(edit.result().value(), replacement);
    assert_eq!(edit.result().key(), case.key);
    assert_eq!(edit.result().pages(), expected_pages(case, &replacement));
    assert_eq!(edit.source(), &source);
    assert_eq!(edit.coverage(), source.coverage());
    assert_eq!(edit.diagnostics().len(), 4);
    let fresh = OverflowPayload::from_pages(
        edit.result().pages(),
        case.pages[0].identifier,
        case.key.len() as u32,
        case.value.len() as u64,
        context(case),
    )
    .unwrap();
    assert_eq!(fresh.edit_value(&case.value).unwrap().result(), &source);
    assert_eq!(source.pages(), expected_pages(case, &case.value));
    check_history(&edit);
}

fn check_history(edit: &OverflowPayloadEdit) {
    for page in edit.source().pages() {
        check_page_history(edit, page);
    }
    assert!(
        edit.history()
            .windows(2)
            .all(|pair| pair[0].identifier <= pair[1].identifier)
    );
    if edit.completion() == HashBufferCompletion::Unchanged {
        assert!(
            edit.history()
                .iter()
                .all(|row| row.span.relation == HashBufferRelation::Copied)
        );
    }
}

#[test]
fn every_admitted_owned_value_edit_matches_closed_form_pages_reopens_and_reverses() {
    for case in packet_cases().iter().filter(|case| !case.wrapped) {
        check_value_edit(case);
    }
}

fn check_page_history(edit: &OverflowPayloadEdit, page: &OverflowPageImage) {
    let mut cursor = 0;
    for row in edit
        .history()
        .iter()
        .filter(|row| row.identifier == page.identifier)
    {
        assert_eq!(row.span.parent, row.span.result);
        assert_eq!(row.span.parent.start, cursor);
        assert_copied_span(edit, page, row);
        cursor = row.span.parent.end;
    }
    assert_eq!(cursor, page.bytes.len());
}

fn assert_copied_span(
    edit: &OverflowPayloadEdit,
    page: &OverflowPageImage,
    row: &OverflowPageHistory,
) {
    if row.span.relation != HashBufferRelation::Copied {
        return;
    }
    let result = edit
        .result()
        .pages()
        .iter()
        .find(|item| item.identifier == page.identifier)
        .unwrap();
    assert_eq!(
        page.bytes[row.span.parent.clone()],
        result.bytes[row.span.result.clone()]
    );
}

fn parse_permutation(case: &OwnedOverflowCase, pages: &[OverflowPageImage]) -> OverflowPayload {
    OverflowPayload::from_pages(
        pages,
        case.pages[0].identifier,
        case.key.len() as u32,
        case.value.len() as u64,
        context(case),
    )
    .unwrap()
}

#[test]
fn supplied_page_permutations_have_identical_normalized_results_and_history() {
    let case = packet_cases().remove(71);
    let expected = parse_case(&case).unwrap();
    let expected_edit = expected.edit_value(&case.value).unwrap();
    let mut pages = case.pages.clone();
    for _ in 0..pages.len() {
        pages.rotate_left(1);
        let actual = parse_permutation(&case, &pages);
        assert_eq!(actual, expected);
        assert_eq!(
            actual.edit_value(&case.value).unwrap().history(),
            expected_edit.history()
        );
    }
}

#[test]
fn all_short_page_prefixes_appended_bytes_and_unused_tail_links_reject() {
    let original = packet_cases().remove(0);
    for length in 0..original.extent {
        let mut bad = original.clone();
        bad.pages[0].bytes.truncate(length);
        assert_bad(&bad);
    }
    let mut bad = original.clone();
    bad.pages[0].bytes.push(0);
    assert_bad(&bad);
    bad = original.clone();
    bad.pages[0].bytes[..8].copy_from_slice(&1u64.to_be_bytes());
    assert_bad(&bad);
}

#[test]
fn exact_noops_own_inputs_and_rejected_lengths_preserve_all_source_bytes() {
    let mut case = packet_cases().remove(8);
    let source = parse_case(&case).unwrap();
    let original = source.clone();
    case.pages[0].bytes.fill(0);
    case.value.fill(0);
    assert_eq!(source, original);
    let noop = source.edit_value(source.value()).unwrap();
    assert_eq!(noop.completion(), HashBufferCompletion::Unchanged);
    assert_eq!(noop.result(), &source);
    check_history(&noop);
    assert!(source.edit_value(&[]).is_err());
    assert_eq!(source, original);
}

#[test]
fn byte_page_and_payload_budgets_accept_exactly_and_reject_one_below() {
    for case in packet_cases().iter().filter(|case| !case.wrapped) {
        let exact = context(case);
        assert!(parse_limited(case, exact).is_ok());
        let mut short = exact;
        short.maximum_bytes -= 1;
        assert!(parse_limited(case, short).is_err());
        short = exact;
        short.maximum_pages -= 1;
        assert!(parse_limited(case, short).is_err());
        if exact.maximum_payload_bytes == 0 {
            continue;
        }
        short = exact;
        short.maximum_payload_bytes -= 1;
        assert!(parse_limited(case, short).is_err());
    }
}

fn assert_bad(case: &OwnedOverflowCase) {
    let original = case.pages.clone();
    assert!(parse_case(case).is_err());
    assert_eq!(case.pages, original);
}

#[test]
fn missing_duplicate_extra_cycle_and_early_end_pages_never_publish_partial_payloads() {
    let case = packet_cases().remove(71);
    let mut bad = case.clone();
    bad.pages.pop();
    assert_bad(&bad);
    bad = case.clone();
    bad.pages.push(case.pages[0].clone());
    assert_bad(&bad);
    bad = case.clone();
    bad.pages.push(OverflowPageImage {
        identifier: identity(1),
        bytes: vec![0; 512],
    });
    assert_bad(&bad);
}

#[test]
fn cyclic_missing_and_early_terminated_continuations_reject_partial_results() {
    let case = packet_cases().remove(71);
    for next in [0, 1, case.pages[0].identifier.get()] {
        let mut bad = case.clone();
        bad.pages[0].bytes[..8].copy_from_slice(&next.to_be_bytes());
        assert_bad(&bad);
    }
}

#[test]
fn checkpoints_extents_and_oversized_counts_reject_before_allocation() {
    let case = packet_cases().remove(71);
    let mut bad = case.clone();
    bad.pages[0].bytes[8..18].fill(0);
    assert_bad(&bad);
    bad = case.clone();
    bad.pages[0].bytes.pop();
    assert_bad(&bad);
    let mut limits = context(&case);
    limits.engine_extent = 511;
    assert!(parse_limited(&case, limits).is_err());
    limits.engine_extent = 65537;
    assert!(parse_limited(&case, limits).is_err());
}

#[test]
fn oversized_and_missing_payloads_reject_before_large_allocation() {
    let case = packet_cases().remove(71);
    let mut limits = context(&case);
    limits.maximum_payload_bytes = usize::MAX;
    assert!(
        OverflowPayload::from_pages(
            &case.pages,
            case.pages[0].identifier,
            u32::MAX,
            u64::MAX,
            limits
        )
        .is_err()
    );
    assert!(OverflowPayload::from_pages(&[], identity(1), 0, 0, limits).is_err());
    assert_eq!(
        OverflowPayload::from_pages(&case.pages, identity(1), 0, 0, limits)
            .unwrap_err()
            .class,
        FailureClass::Missing
    );
}

const NATIVE_STREAMS: [&[u8]; 3] = [
    include_bytes!("fixtures/database-overflow/native-0.db"),
    include_bytes!("fixtures/database-overflow/native-1.db"),
    include_bytes!("fixtures/database-overflow/native-2.db"),
];
const NATIVE_KEYS: [&[u8]; 3] = [
    include_bytes!("fixtures/database-overflow/native-0.key.bin"),
    include_bytes!("fixtures/database-overflow/native-1.key.bin"),
    include_bytes!("fixtures/database-overflow/native-2.key.bin"),
];
const NATIVE_VALUES: [&[u8]; 3] = [
    include_bytes!("fixtures/database-overflow/native-0.value.bin"),
    include_bytes!("fixtures/database-overflow/native-1.value.bin"),
    include_bytes!("fixtures/database-overflow/native-2.value.bin"),
];
fn native_case(index: usize) -> OwnedOverflowCase {
    let identifiers: &[u64] = [&[16][..], &[3][..], &[4, 5][..]][index];
    let pages = identifiers
        .iter()
        .map(|id| OverflowPageImage {
            identifier: identity(*id),
            bytes: NATIVE_STREAMS[index][*id as usize * 4096..(*id as usize + 1) * 4096].to_vec(),
        })
        .collect();
    OwnedOverflowCase {
        extent: 4096,
        pages,
        key: NATIVE_KEYS[index].to_vec(),
        value: NATIVE_VALUES[index].to_vec(),
        wrapped: false,
    }
}

#[test]
fn native_single_and_multiple_page_values_match_literal_spans_and_reverse_edits() {
    for index in 0..3 {
        let case = native_case(index);
        let payload = parse_case(&case).unwrap();
        assert_eq!(payload.key(), case.key);
        assert_eq!(payload.value(), case.value);
        check_value_edit(&case);
    }
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn all_public_overflow_types_are_send_and_sync() {
    assert_send_sync::<OverflowPayload>();
    assert_send_sync::<OverflowPayloadEdit>();
    assert_send_sync::<OverflowStorageProfile>();
    assert_send_sync::<OverflowStorageContext>();
    assert_send_sync::<OverflowPageImage>();
    assert_send_sync::<OverflowCoverage>();
    assert_send_sync::<OverflowCheckMode>();
    assert_send_sync::<OverflowPageHistory>();
    assert_send_sync::<OverflowDiagnostic>();
}

#[test]
fn concurrent_mixed_overflow_edits_equal_single_thread_results() {
    let case = native_case(2);
    let shared = Arc::new(parse_case(&case).unwrap());
    let workers: Vec<_> = (0..8)
        .map(|salt| {
            let source = shared.clone();
            std::thread::spawn(move || {
                let value: Vec<_> = source.value().iter().map(|byte| byte ^ salt).collect();
                source.edit_value(&value).unwrap().result().pages().to_vec()
            })
        })
        .collect();
    for (salt, worker) in workers.into_iter().enumerate() {
        let value: Vec<_> = case.value.iter().map(|byte| byte ^ salt as u8).collect();
        assert_eq!(worker.join().unwrap(), expected_pages(&case, &value));
    }
    assert_eq!(shared.value(), case.value);
}
