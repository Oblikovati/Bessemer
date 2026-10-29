use super::*;
use crate::archive_codec::{ArchiveCursor, ObjectRecord};
use crate::header::ArchiveVersion;
use crate::model_archives::reader::{ModelArchiveContext, ModelFieldLimits};
use crate::model_archives::{ModelField, encode_fields};

const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 100_000,
    maximum_fields: 30_000,
    maximum_entries: 30_000,
    maximum_depth: 128,
    maximum_objects: 256,
};

fn decoder(bytes: &[u8]) -> ModelDecoder<'_> {
    ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(19000),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap()
}

fn decode(bytes: &[u8]) -> Result<(Vec<ModelField>, usize)> {
    let mut reader = decoder(bytes);
    let mut map = ObjectReader::new();
    reader.object("object", &mut map, ModelObjectRole::Any)?;
    assert!(!reader.inline_part_scope);
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}

#[test]
fn complete_tools_match_source_primitives_shared_map_census_and_all_bytes() {
    for (bytes, oracle, slots) in [
        (
            include_bytes!("../fixtures/forming-tools/blank-tool.bin").as_slice(),
            include_str!("../fixtures/forming-tools/blank-tool.tsv"),
            162,
        ),
        (
            include_bytes!("../fixtures/forming-tools/null-part-tool.bin").as_slice(),
            include_str!("../fixtures/forming-tools/null-part-tool.tsv"),
            3,
        ),
        (
            include_bytes!("../fixtures/forming-tools/paired-tools.bin").as_slice(),
            include_str!("../fixtures/forming-tools/paired-tools.tsv"),
            165,
        ),
    ] {
        let (fields, actual) = decode(bytes).unwrap();
        assert_eq!(actual, slots);
        crate::model_archives::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(encode_fields(&fields, LIMITS.maximum_bytes).unwrap(), bytes);
    }
}

#[test]
fn inline_part_factory_is_restored_after_success_and_every_early_tool_failure() {
    let raw = include_bytes!("../fixtures/forming-tools/blank-tool.bin");
    for length in [
        0,
        17,
        18,
        31,
        32,
        33,
        35,
        36,
        100,
        4888,
        raw.len() - 1,
        raw.len(),
    ] {
        let mut reader = decoder(&raw[..length]);
        let mut map = ObjectReader::new();
        let result = reader.object("object", &mut map, ModelObjectRole::Any);
        assert_eq!(result.is_ok(), length == raw.len());
        assert!(!reader.inline_part_scope, "prefix {length}");
        assert!(reader.verified_part_root.is_none());
    }
}

#[test]
fn embedded_part_cannot_be_decoded_using_the_ordinary_external_part_factory() {
    let raw = include_bytes!("../fixtures/forming-tools/blank-tool.bin");
    let mut cursor = ArchiveCursor::new(raw);
    let mut map = ObjectReader::new();
    assert!(matches!(
        map.object(&mut cursor).unwrap(),
        ObjectRecord::New { .. }
    ));
    let part = &raw[18..];
    let mut reader = decoder(part);
    let mut map = ObjectReader::new();
    let error = reader
        .object("object", &mut map, ModelObjectRole::Any)
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("isolated factory"));
}

fn completed_outer_root() -> crate::model_archives::part_archive::VerifiedPartRoot {
    let document =
        crate::Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let bytes = document.read_stream("Contents/Config-0").unwrap();
    let header = std::sync::Arc::new(crate::model_archives::header_load_plan::native_header());
    crate::model_archives::part_archive::read(&bytes, header, ArchiveVersion::new(19000), LIMITS)
        .unwrap()
        .verified_root()
}

#[test]
fn embedded_tool_restores_the_actual_completed_outer_part_proof_after_failure_or_success() {
    let root = completed_outer_root();
    let raw = include_bytes!("../fixtures/forming-tools/blank-tool.bin");
    for length in [raw.len() - 1, raw.len()] {
        let mut reader = decoder(&raw[..length]).with_verified_part_root(root);
        let mut map = ObjectReader::new();
        let result = reader.object("object", &mut map, ModelObjectRole::Any);
        assert_eq!(result.is_ok(), length == raw.len());
        assert_eq!(
            reader.verified_part_root.map(|value| value.index()),
            Some(root.index())
        );
        assert!(!reader.inline_part_scope);
    }
}

#[test]
fn both_embedded_section_counts_reject_negative_overruns_and_inexact_ends() {
    let raw = include_bytes!("../fixtures/forming-tools/blank-tool.bin");
    for (offset, size) in [(32, 4856_i32), (4892, 21103_i32)] {
        for count in [-1, 0, size - 1, size + 1, i32::MAX] {
            let mut malformed = raw.to_vec();
            malformed[offset..offset + 4].copy_from_slice(&count.to_le_bytes());
            assert!(decode(&malformed).is_err(), "count {count} at {offset}");
        }
    }
    let mut trailing = raw.to_vec();
    trailing.push(0);
    assert!(decode(&trailing).is_err());
}

#[test]
fn cached_tool_object_cannot_replace_the_declared_embedded_part_pointer() {
    let mut raw = include_bytes!("../fixtures/forming-tools/paired-tools.bin").to_vec();
    let offset = raw.len() - 10;
    assert_eq!(&raw[offset..offset + 2], &6_u16.to_le_bytes());
    raw[offset..offset + 2].copy_from_slice(&4_u16.to_le_bytes());
    let Err(error) = decode(&raw) else {
        panic!("expected a typed part-role rejection");
    };
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("moPart_c"));
}
