use super::header_imports::ModelHeaderLoadPlan;
use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, ModelObjectRole, encode_fields,
};
use super::{lightweight, model_lists, resolved_managers, resolved_start};
use crate::archive_codec::{ArchiveCursor, ObjectReader, ObjectRecord};
use crate::header::ArchiveVersion;
use crate::{Document, Result};
use std::sync::Arc;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/resolved-manager-cases.bin");
const NATIVE: &[u8] = include_bytes!("../fixtures/model-archives/native-resolved-start.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 30000,
    maximum_fields: 6000,
    maximum_entries: 6000,
    maximum_depth: 32,
    maximum_objects: 128,
};
const VERSION: ArchiveVersion = ArchiveVersion::new(19000);

fn decoder(bytes: &[u8], version: u32) -> ModelDecoder<'_> {
    ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap()
}

fn read_case(kind: u8, reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    match kind {
        0 => model_lists::nodes(reader, map),
        1 => resolved_managers::atom_manager(reader, map),
        2 => resolved_managers::ghost_manager(reader, map),
        3 | 5 => reader.object("object", map, ModelObjectRole::Any),
        _ => panic!("unexpected resolved case {kind}"),
    }
}

fn decode(kind: u8, version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    let mut reader = decoder(bytes, version);
    read_case(kind, &mut reader, &mut ObjectReader::new())?;
    reader.finish()
}

#[test]
fn resolved_manager_cases_match_independent_values_and_exact_reconstruction() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut count, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(kind, version, bytes).unwrap();
        fields += super::node_units::assert_oracle(&decoded, oracle);
        assert_eq!(
            encode_fields(&decoded, LIMITS.maximum_bytes).unwrap(),
            bytes
        );
        count += 1;
    }
    assert_eq!((count, fields), (26, 400));
}

#[test]
fn every_resolved_manager_prefix_and_trailing_byte_is_rejected() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(kind, version, &bytes[..length]).is_err(),
                "kind {kind} version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(kind, version, &trailing).is_err());
    }
}

pub(super) fn inherited_map() -> ObjectReader {
    let document = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let bytes = document.read_stream("Contents/Config-0").unwrap();
    let mut map = ObjectReader::new();
    let reader = loaded_native_builder(&bytes, &mut map);
    assert_eq!(reader.remaining(), bytes.len() - 2921);
    map
}

fn loaded_native_builder<'a>(bytes: &'a [u8], map: &mut ObjectReader) -> ModelDecoder<'a> {
    let mut reader = decoder(bytes, 19000);
    let header = Arc::new(super::header_load_plan::native_header());
    let plan = ModelHeaderLoadPlan::new(header, VERSION, 6000).unwrap();
    lightweight::read(&mut reader, map, plan).unwrap();
    reader
}

#[test]
fn native_resolved_start_keeps_the_inherited_map_and_exact_fields() {
    let mut reader = decoder(NATIVE, 19000);
    resolved_start::read(&mut reader, &mut inherited_map()).unwrap();
    let fields = reader.finish().unwrap();
    let oracle = include_str!("../fixtures/model-archives/native-resolved-start.txt");
    assert_eq!(super::node_units::assert_oracle(&fields, oracle), 11);
    assert_eq!(
        encode_fields(&fields, LIMITS.maximum_bytes).unwrap(),
        NATIVE
    );
    let field = fields
        .iter()
        .find(|field| field.path.as_ref() == "root.relation_manager")
        .unwrap();
    assert!(matches!(&field.value, ModelFieldValue::ObjectTag(tag)
        if matches!(&tag.record, ObjectRecord::New { index, .. } if index.value() == 56)));
}

#[test]
fn a_fresh_map_changes_native_relation_identity_even_when_scalar_bytes_parse() {
    let mut fresh = decoder(NATIVE, 19000);
    resolved_start::read(&mut fresh, &mut ObjectReader::new()).unwrap();
    let fresh_fields = fresh.finish().unwrap();
    let mut inherited = decoder(NATIVE, 19000);
    resolved_start::read(&mut inherited, &mut inherited_map()).unwrap();
    assert_ne!(fresh_fields, inherited.finish().unwrap());
}

#[test]
fn resolved_handoff_uses_one_builder_and_does_not_publish_the_unread_root() {
    let document = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let bytes = document.read_stream("Contents/Config-0").unwrap();
    let mut map = ObjectReader::new();
    let mut reader = loaded_native_builder(&bytes, &mut map);
    assert_eq!(reader.section_size("resolved_size").unwrap(), 21103);
    reader
        .bounded(20454, |reader| composed_defaults(reader, &mut map))
        .unwrap();
    assert_eq!(reader.remaining(), 649);
    assert!(reader.finish().is_err());
}

fn composed_defaults(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    resolved_start::read(reader, map)?;
    reader.scope("detail_definitions", |reader| {
        super::detail_definitions::read(reader, map)
    })?;
    reader.scope("after_details", |reader| {
        super::environment::read(reader, map)
    })?;
    reader.scope("views", |reader| super::views::read(reader, map))
}

#[test]
fn invalid_atom_and_ghost_counts_fail_before_any_large_allocation() {
    let null_atom: Vec<u8> = [0_i32, 1, 0]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    assert!(decode(1, 19000, &null_atom).is_err());
    let impossible: Vec<u8> = [0_u32, 0xffffffff]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    assert!(decode(2, 19000, &impossible).is_err());
    assert!(decode(1, 19000, &[0, 0, 0, 0, 255, 255, 255, 255]).is_err());
}

#[test]
fn embedded_atom_geometry_fails_explicitly_instead_of_treating_it_as_an_empty_body() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (kind, version, bytes, _) = next_case(&mut cursor);
        if kind != 5 {
            continue;
        }
        let fields = decode(kind, version, bytes).unwrap();
        let index = fields
            .iter()
            .position(|field| field.path.ends_with(".body_mode"))
            .unwrap();
        let offset = encode_fields(&fields[..index], 10000).unwrap().len();
        let mut altered = bytes.to_vec();
        altered[offset..offset + 4].copy_from_slice(&1_i32.to_le_bytes());
        let error = decode(kind, version, &altered).unwrap_err();
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}
