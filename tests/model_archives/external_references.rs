use super::annotation_view_tests::{LIMITS, decode};
use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, encode_fields};
use crate::archive_codec::ArchiveCursor;
use crate::header::{ArchiveVersion, WireKind};

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/external-reference-cases.bin");

#[test]
fn external_references_reuse_header_rules_and_match_independent_fields_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (24, 396));
}

#[test]
fn external_reference_prefixes_and_trailing_bytes_reject_partial_objects() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..length]).is_err(),
                "version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(version, &trailing).is_err());
    }
}

#[test]
fn external_reference_snapshots_own_input_strings_and_parallel_writes_keep_sharing() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        assert_owned_case(version, bytes);
    }
}

fn assert_owned_case(version: u32, bytes: &[u8]) {
    let mut source = bytes.to_vec();
    let fields = decode(version, &source).unwrap();
    source.fill(0xa5);
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), bytes);
        }
    });
}

#[test]
fn unimplemented_header_field_kinds_fail_and_preserve_the_first_failure() {
    let mut reader = ModelDecoder::new_in_context(
        &[],
        ArchiveVersion::new(19000),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap();
    let error = super::external_references::unsupported_kind(
        &mut reader,
        "future.external",
        WireKind::Bounds,
    )
    .unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("Bounds"));
    assert_eq!(reader.finish().unwrap_err(), error);
}
