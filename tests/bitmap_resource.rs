//! Independent count/payload boundaries and immutable resource edits (#190).
#[path = "bitmap_resource/concurrency.rs"]
mod concurrency;
#[path = "bitmap_resource/native.rs"]
mod native;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;
use bessemer::bitmap_resource::*;
use bessemer::{Document, FailureClass};

const PACKET: &[u8] = include_bytes!("fixtures/bitmap-resource-arrays/arrays.bin");
const MAXIMUM: usize = 1_048_576;
const STREAM: &str = "Contents/Config-0-ResolvedFeatures";

struct ResourceLiteral {
    prefix: usize,
    count: usize,
    raw: &'static [u8],
}

fn resource_literals() -> Vec<ResourceLiteral> {
    assert_eq!(&PACKET[..4], b"BRA1");
    assert_eq!(u32::from_le_bytes(PACKET[4..8].try_into().unwrap()), 20);
    let mut cursor = &PACKET[8..];
    let mut cases = Vec::new();
    while !cursor.is_empty() {
        let count = u32::from_le_bytes(cursor[1..5].try_into().unwrap()) as usize;
        let length = u32::from_le_bytes(cursor[5..9].try_into().unwrap()) as usize;
        cases.push(ResourceLiteral {
            prefix: usize::from(cursor[0]),
            count,
            raw: &cursor[9..9 + length],
        });
        cursor = &cursor[9 + length..];
    }
    assert_eq!(cases.len(), 20);
    cases
}

fn literal_array(payload: &[u8], prefix: usize) -> Vec<u8> {
    let mut expected = if prefix == 2 {
        u16::try_from(payload.len()).unwrap().to_le_bytes().to_vec()
    } else {
        [
            u16::MAX.to_le_bytes().as_slice(),
            &u32::try_from(payload.len()).unwrap().to_le_bytes(),
        ]
        .concat()
    };
    expected.extend_from_slice(payload);
    expected
}

fn verify_resource_coverage(edit: &BitmapResourceEdit, expected: &[u8], prefix: usize) {
    let coverage = edit.coverage();
    assert_eq!(coverage.mode, BitmapResourceCheckMode::ExactCountAndPayload);
    assert_eq!(coverage.wire_bytes, expected.len());
    assert_eq!(coverage.count_prefix_bytes, prefix);
    assert_eq!(coverage.payload_bytes, expected.len() - prefix);
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(coverage.object_map_slots_added, 0);
    assert_eq!(edit.resource().to_bytes().unwrap(), expected);
    assert_eq!(
        edit.diagnostics(),
        &[
            BitmapResourceDiagnostic::OpaquePayloadNotInterpreted,
            BitmapResourceDiagnostic::RuntimeResizingNotSimulated,
            BitmapResourceDiagnostic::StorageOwnerNotInferred,
        ]
    );
}

fn verify_resource_history(
    edit: &BitmapResourceEdit,
    old: &[u8],
    new: &[u8],
    prefixes: [usize; 2],
) {
    let [old_prefix, new_prefix] = prefixes;
    let relations = edit.history();
    assert_eq!(relations[0].field, BitmapResourceField::Count);
    assert_eq!(relations[1].field, BitmapResourceField::Payload);
    assert_eq!(relations[0].parents, vec![0..old_prefix]);
    assert_eq!(relations[1].parents, vec![old_prefix..old.len()]);
    assert_eq!(relations[0].result, 0..new_prefix);
    assert_eq!(relations[1].result, new_prefix..new.len());
    for (record, equal) in relations.iter().zip([
        old[..old_prefix] == new[..new_prefix],
        old[old_prefix..] == new[new_prefix..],
    ]) {
        verify_resource_relation(record, equal);
    }
    assert_eq!(edit.source().to_bytes().unwrap(), old);
}

#[test]
fn every_literal_roundtrips_preserves_noncanonical_headers_and_constructs_minimal_headers() {
    for case in resource_literals() {
        let resource = BitmapResourceArray::from_bytes(case.raw, MAXIMUM).unwrap();
        assert_eq!(resource.payload(), &case.raw[case.prefix..]);
        assert_eq!(resource.payload().len(), case.count);
        assert_eq!(resource.to_bytes().unwrap(), case.raw);
        let unchanged = resource.edit_payload(resource.payload()).unwrap();
        assert_eq!(unchanged.completion(), BitmapResourceCompletion::Unchanged);
        verify_resource_coverage(&unchanged, case.raw, case.prefix);
        verify_resource_history(&unchanged, case.raw, case.raw, [case.prefix; 2]);
        let fresh = BitmapResourceArray::new(resource.payload().to_vec(), MAXIMUM).unwrap();
        let prefix = if case.count < 65535 { 2 } else { 6 };
        assert_eq!(
            fresh.to_bytes().unwrap(),
            literal_array(resource.payload(), prefix)
        );
    }
}

fn verify_resource_replacement(case: &ResourceLiteral, length: usize) {
    let source = BitmapResourceArray::from_bytes(case.raw, MAXIMUM).unwrap();
    let mut requested: Vec<u8> = (0..length)
        .map(|index| (index.wrapping_mul(29) ^ length) as u8)
        .collect();
    let prefix = if case.prefix == 6 || length >= 65535 {
        6
    } else {
        2
    };
    let expected = literal_array(&requested, prefix);
    let edit = source.edit_payload(&requested).unwrap();
    verify_resource_coverage(&edit, &expected, prefix);
    verify_resource_history(&edit, case.raw, &expected, [case.prefix, prefix]);
    verify_resource_completion(&edit, case.raw != expected);
    verify_resource_reverse_and_isolation(&source, &edit, case.raw, &expected, &mut requested);
}

#[test]
fn all_payload_replacements_preserve_inputs_and_reverse_across_short_extended_boundary() {
    for case in resource_literals() {
        for length in [0, 1, 2, 255, 65534, 65535, 65536] {
            verify_resource_replacement(&case, length);
        }
    }
}

#[test]
fn every_proper_prefix_and_appended_remainder_is_invalid() {
    let mut rejected = 0;
    for case in resource_literals() {
        for end in 0..case.raw.len() {
            let error = BitmapResourceArray::from_bytes(&case.raw[..end], MAXIMUM).unwrap_err();
            assert_eq!(error.class, FailureClass::Invalid);
            assert_eq!(error.operation, "bitmap resource array");
            rejected += 1;
        }
        let mut appended = case.raw.to_vec();
        appended.push(0x5a);
        assert!(BitmapResourceArray::from_bytes(&appended, MAXIMUM).is_err());
    }
    assert_eq!(rejected, 1_513_405);
}

#[test]
fn exact_payload_budgets_empty_arrays_and_oversized_counts_remain_distinct() {
    for case in resource_literals() {
        let source = BitmapResourceArray::from_bytes(case.raw, case.count).unwrap();
        let over = vec![0; case.count + 1];
        assert!(source.edit_payload(&over).is_err());
        assert_eq!(source.to_bytes().unwrap(), case.raw);
        if case.count > 0 {
            assert!(BitmapResourceArray::from_bytes(case.raw, case.count - 1).is_err());
            assert!(BitmapResourceArray::new(vec![0; case.count], case.count - 1).is_err());
        }
    }
    let empty = BitmapResourceArray::from_bytes(&[0, 0], 0).unwrap();
    assert!(empty.payload().is_empty());
    assert!(BitmapResourceArray::from_bytes(&[], 0).is_err());
    assert!(BitmapResourceArray::from_bytes(&[255; 6], MAXIMUM).is_err());
}

struct ResourceDocumentFixture {
    document: Document,
}

impl ResourceDocumentFixture {
    fn new(raw: &[u8]) -> Self {
        let mut document =
            Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
        document.replace_stream(STREAM, raw.to_vec()).unwrap();
        Self {
            document: Document::from_bytes(&document.to_bytes().unwrap()).unwrap(),
        }
    }
}

#[test]
fn explicit_stream_reads_apply_payload_budgets_and_do_not_infer_storage_ownership() {
    for case in resource_literals() {
        let fixture = ResourceDocumentFixture::new(case.raw);
        let resource =
            BitmapResourceArray::from_document(&fixture.document, STREAM, case.count).unwrap();
        assert_eq!(resource.to_bytes().unwrap(), case.raw);
        if case.count > 0 {
            assert!(
                BitmapResourceArray::from_document(&fixture.document, STREAM, case.count - 1)
                    .is_err()
            );
        }
        let missing =
            BitmapResourceArray::from_document(&fixture.document, "missing-resource", MAXIMUM)
                .unwrap_err();
        assert_eq!(missing.class, FailureClass::Missing);
    }
}

#[test]
fn published_resource_types_are_send_sync() {
    fn require<T: Send + Sync>() {}
    require::<BitmapResourceArray>();
    require::<BitmapResourceEdit>();
    require::<BitmapResourceHistory>();
    require::<BitmapResourceCoverage>();
    require::<BitmapResourceCompletion>();
    require::<BitmapResourceDiagnostic>();
    require::<BitmapResourceCheckMode>();
    require::<BitmapResourceField>();
    require::<BitmapResourceRelation>();
}

fn verify_resource_relation(record: &BitmapResourceHistory, equal: bool) {
    let expected = if equal {
        BitmapResourceRelation::Copied
    } else {
        BitmapResourceRelation::Derived
    };
    assert_eq!(record.relation, expected);
}

fn verify_resource_reverse_and_isolation(
    source: &BitmapResourceArray,
    edit: &BitmapResourceEdit,
    original: &[u8],
    expected: &[u8],
    requested: &mut [u8],
) {
    let reverse = edit.resource().edit_payload(source.payload()).unwrap();
    assert_eq!(reverse.resource().to_bytes().unwrap(), original);
    requested.fill(0x5a);
    assert_eq!(edit.resource().to_bytes().unwrap(), expected);
    assert_eq!(source.to_bytes().unwrap(), original);
}

fn verify_resource_completion(edit: &BitmapResourceEdit, changed: bool) {
    let expected = if changed {
        BitmapResourceCompletion::Changed
    } else {
        BitmapResourceCompletion::Unchanged
    };
    assert_eq!(edit.completion(), expected);
}
