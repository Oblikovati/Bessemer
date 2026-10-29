use super::*;
use crate::model_archives::reader::ModelArchiveContext;
use crate::model_archives::{ModelField, ModelFieldLimits, encode_fields};

#[path = "../support/header_cases.rs"]
mod cases;

const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 100_000,
    maximum_fields: 20_000,
    maximum_entries: 20_000,
    maximum_depth: 64,
    maximum_objects: 2000,
};

fn assert_source_case(cursor: &mut crate::archive_codec::ArchiveCursor<'static>) {
    let version = cursor.u32().unwrap();
    let length = cursor.u32().unwrap() as usize;
    let oracle_length = cursor.u32().unwrap() as usize;
    let slots = cursor.u32().unwrap() as usize;
    let bytes = cursor.take(length).unwrap();
    let oracle = std::str::from_utf8(cursor.take(oracle_length).unwrap()).unwrap();
    let mut reader = decoder(bytes, version);
    let mut map = ObjectReader::new();
    reader
        .object(
            "header",
            &mut map,
            ModelObjectRole::Declared(&["moHeader_c"]),
        )
        .unwrap();
    assert_eq!(map.into_snapshot().slot_count(), slots);
    let fields = reader.finish().unwrap();
    crate::model_archives::manager_oracle_tests::assert_reference_fields(
        &fields,
        bytes,
        oracle,
        oracle.lines().count(),
    );
}

#[test]
fn every_inline_header_field_matches_actual_source_primitives_and_map_census() {
    let packet = include_bytes!("../fixtures/forming-tools/inline-headers.bin");
    let mut cursor = crate::archive_codec::ArchiveCursor::new(packet);
    assert_eq!(cursor.take(4).unwrap(), b"IH01");
    let count = cursor.u32().unwrap();
    assert!(count > 15);
    for _ in 0..count {
        assert_source_case(&mut cursor);
    }
    cursor.finish().unwrap();
}

fn decoder(bytes: &[u8], version: u32) -> ModelDecoder<'_> {
    ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap()
}

fn decode(bytes: &[u8], version: u32) -> Result<Vec<ModelField>> {
    let mut reader = decoder(bytes, version);
    let mut map = ObjectReader::new();
    reader.object(
        "header",
        &mut map,
        ModelObjectRole::Declared(&["moHeader_c"]),
    )?;
    assert_eq!(reader.version(), version);
    reader.finish()
}

#[test]
fn modern_inline_headers_reuse_every_existing_boundary_fixture_exactly() {
    let mut tested = 0;
    for (name, version, bytes) in cases::CASES.iter().filter(|case| case.1 > 0x27cb) {
        let fields = decode(bytes, *version).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(
            encode_fields(&fields, LIMITS.maximum_bytes).unwrap(),
            *bytes,
            "{name}"
        );
        let independent =
            crate::header::DocumentHeader::from_bytes(bytes, ArchiveVersion::new(*version), 2000)
                .unwrap();
        assert_eq!(independent.to_bytes().unwrap(), *bytes);
        tested += 1;
    }
    assert!(tested > 15);
}

#[test]
fn native_header_and_embedded_version_overrides_reconstruct_and_restore_caller_version() {
    for bytes in [
        include_bytes!("../fixtures/headers/blank-part.bin").as_slice(),
        include_bytes!("../fixtures/headers/override-old.bin"),
        include_bytes!("../fixtures/headers/override-signed.bin"),
    ] {
        let fields = decode(bytes, 19000).unwrap();
        assert_eq!(encode_fields(&fields, LIMITS.maximum_bytes).unwrap(), bytes);
    }
}

#[test]
fn inline_header_keeps_its_real_root_identity_for_the_following_shared_pointer() {
    let bytes = include_bytes!("../fixtures/headers/blank-part.bin");
    let mut reader = decoder(bytes, 19000);
    let mut map = ObjectReader::new();
    let root = reader
        .object_record(
            "header",
            &mut map,
            ModelObjectRole::Declared(&["moHeader_c"]),
        )
        .unwrap();
    let ObjectRecord::New { index, .. } = root else {
        panic!("expected new header");
    };
    reader.finish().unwrap();
    let mut combined = bytes.to_vec();
    combined.extend_from_slice(&(index.value() as u16).to_le_bytes());
    let mut reader = decoder(&combined, 19000);
    let mut map = ObjectReader::new();
    reader
        .object(
            "header",
            &mut map,
            ModelObjectRole::Declared(&["moHeader_c"]),
        )
        .unwrap();
    let alias = reader
        .object_record(
            "following",
            &mut map,
            ModelObjectRole::Declared(&["moHeader_c"]),
        )
        .unwrap();
    assert!(matches!(alias, ObjectRecord::Reference { index: actual, .. } if actual == index));
    assert_eq!(
        encode_fields(&reader.finish().unwrap(), LIMITS.maximum_bytes).unwrap(),
        combined
    );
}

#[test]
fn every_incomplete_override_header_restores_version_and_poisoned_operation() {
    let bytes = include_bytes!("../fixtures/headers/override-old.bin");
    for length in 0..bytes.len() {
        let mut reader = decoder(&bytes[..length], 19000);
        let mut map = ObjectReader::new();
        let Err(error) = reader.object(
            "header",
            &mut map,
            ModelObjectRole::Declared(&["moHeader_c"]),
        ) else {
            panic!("prefix {length} unexpectedly completed");
        };
        assert_eq!(reader.version(), 19000, "prefix {length}");
        let Err(final_error) = reader.finish() else {
            panic!("prefix {length} was not poisoned");
        };
        assert_eq!(final_error, error);
    }
}

#[test]
fn independent_inline_header_edit_preserves_outer_fields_and_old_shared_names() {
    let original = include_bytes!("../fixtures/forming-tools/older-external.bin");
    let expected = include_bytes!("../fixtures/forming-tools/older-external-edited.bin");
    let baseline = decode(original, 19000).unwrap();
    let mut edited = baseline.clone();
    let next = edited
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.header.next_id")
        .unwrap();
    assert_eq!(next.value, ModelFieldValue::I32(-24));
    next.value = ModelFieldValue::I32(123456);
    assert_eq!(
        encode_fields(&edited, LIMITS.maximum_bytes).unwrap(),
        expected
    );
    assert_eq!(decode(expected, 19000).unwrap(), edited);
    assert_eq!(
        encode_fields(&baseline, LIMITS.maximum_bytes).unwrap(),
        original
    );
    assert!(baseline.iter().any(
        |field| field.path.as_ref() == "root.header.self_reference.path"
            && matches!(field.value, ModelFieldValue::Text(_))
    ));
}

#[test]
fn inline_header_context_budget_and_trailing_bytes_fail_explicitly() {
    let bytes = include_bytes!("../fixtures/headers/blank-part.bin");
    for context in [
        ModelArchiveContext::Unspecified,
        ModelArchiveContext::OrdinaryPartExternalXml,
    ] {
        let mut limits = LIMITS;
        limits.maximum_objects = 1;
        let mut reader =
            ModelDecoder::new_in_context(bytes, ArchiveVersion::new(19000), limits, context)
                .unwrap();
        let mut map = ObjectReader::new();
        assert!(
            reader
                .object(
                    "header",
                    &mut map,
                    ModelObjectRole::Declared(&["moHeader_c"])
                )
                .is_err()
        );
        assert!(reader.finish().is_err());
    }
    let mut trailing = bytes.to_vec();
    trailing.push(0);
    assert!(decode(&trailing, 19000).is_err());
}
