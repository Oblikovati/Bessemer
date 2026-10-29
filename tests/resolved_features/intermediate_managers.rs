use super::manager_dialect::ManagerDialect;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldValue, encode_fields};
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;
// #54 / C-364: native census 350; the independent fresh-visual composition adds one.
const LIMITS: super::ModelArchiveLimits = super::ModelArchiveLimits {
    maximum_objects: 351,
    ..super::annotation_view_tests::LIMITS
};
type IntermediateRecord = (u32, u8, &'static [u8], &'static str, usize);
fn records() -> Vec<IntermediateRecord> {
    let mut cursor = ArchiveCursor::new(include_bytes!(
        "../fixtures/intermediate-managers/cases.bin"
    ));
    assert_eq!(cursor.take(4).unwrap(), b"ICM1");
    let count = cursor.u32().unwrap();
    assert_eq!(count, 18);
    let records = (0..count).map(|_| next_record(&mut cursor)).collect();
    assert_eq!(cursor.remaining(), 0);
    records
}
fn next_record(cursor: &mut ArchiveCursor<'static>) -> IntermediateRecord {
    let version = cursor.u32().unwrap();
    let style = cursor.u8().unwrap();
    let width = cursor.u32().unwrap();
    let size = cursor.u32().unwrap();
    let slots = cursor.u32().unwrap() as usize;
    let bytes = cursor.take(width as usize).unwrap();
    let oracle = std::str::from_utf8(cursor.take(size as usize).unwrap()).unwrap();
    (version, style, bytes, oracle, slots)
}
fn decode_object(version: u32, bytes: &[u8]) -> crate::Result<(Vec<ModelField>, usize)> {
    let dialect = ManagerDialect::from_version(ArchiveVersion::new(version))?;
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?
    .with_manager_dialect(dialect);
    let mut map = ObjectReader::new();
    reader.scope("config_object_4c8", |reader| {
        super::configuration_suffix::config_object(reader, &mut map)
    })?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}
#[test]
fn intermediate_config_objects_match_independent_primitives_slots_and_bytes() {
    for (version, _, bytes, oracle, slots) in records() {
        let (fields, actual_slots) = decode_object(version, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        assert_eq!(actual_slots, slots);
    }
}
#[test]
fn intermediate_config_objects_preserve_the_exact_50_52_54_byte_gates() {
    for (version, _, bytes, _, _) in records().into_iter().filter(|r| r.1 == 0) {
        assert_eq!(
            bytes.len(),
            match version {
                11000 => 50,
                18000 => 52,
                19000 => 54,
                _ => panic!("unreviewed fixture"),
            }
        );
        let fields = decode_object(version, bytes).unwrap().0;
        assert_eq!(
            fields.iter().any(|f| f.path.ends_with(".list_88")),
            version > 0x2b8a
        );
        assert_eq!(
            fields.iter().any(|f| f.path.ends_with(".object_a8")),
            version > 0x4972
        );
    }
}
#[test]
fn intermediate_config_objects_reject_every_prefix_extra_byte_and_neighboring_version() {
    for (version, _, bytes, _, _) in records() {
        for end in 0..bytes.len() {
            assert!(decode_object(version, &bytes[..end]).is_err());
        }
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert!(decode_object(version, &extra).is_err());
    }
    for version in [10999, 11001] {
        assert!(decode_object(version, records()[0].2).is_err());
    }
}
const NATIVE: &[(&[u8], &str, usize)] = &[
    (
        include_bytes!("../fixtures/intermediate-managers/native-manager.bin"),
        include_str!("../fixtures/intermediate-managers/native-manager.tsv"),
        4782,
    ),
    (
        include_bytes!("../fixtures/intermediate-managers/native-manager-edited.bin"),
        include_str!("../fixtures/intermediate-managers/native-manager-edited.tsv"),
        4782,
    ),
    (
        include_bytes!("../fixtures/intermediate-managers/native-manager-fresh-visual.bin"),
        include_str!("../fixtures/intermediate-managers/native-manager-fresh-visual.tsv"),
        4818,
    ),
];

#[test]
fn intermediate_native_managers_match_independent_primitives_and_exact_reconstruction() {
    for (bytes, oracle, reads) in NATIVE {
        let parsed =
            super::configuration_manager::parse(bytes, ArchiveVersion::new(11000), LIMITS).unwrap();
        assert_eq!(parsed.version, ArchiveVersion::new(11000));
        assert_eq!(parsed.dialect, ManagerDialect::Owned11000);
        super::manager_oracle_tests::assert_reference_fields(&parsed.fields, bytes, oracle, *reads);
        assert_eq!(encode_fields(&parsed.fields, 30000).unwrap(), *bytes);
    }
}
#[test]
fn intermediate_native_edit_matches_the_independent_config_object_golden() {
    let mut parsed =
        super::configuration_manager::parse(NATIVE[0].0, ArchiveVersion::new(11000), LIMITS)
            .unwrap();
    parsed
        .fields
        .iter_mut()
        .find(|f| f.path.ends_with(".config_object_4c8.raw_0c"))
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&parsed.fields, 30000).unwrap(), NATIVE[1].0);
}
#[test]
fn intermediate_native_manager_requires_its_real_endpoint_and_excludes_later_fields() {
    let parsed =
        super::configuration_manager::parse(NATIVE[0].0, ArchiveVersion::new(11000), LIMITS)
            .unwrap();
    assert_absent_fields(
        &parsed.fields,
        "root.manager.configuration_358.",
        &[
            "raw_848",
            "raw_4c0",
            "raw_4c4",
            "raw_a78",
            "envelope_bd0",
            "final_vector_bf0",
            "raw_cc8",
            "set_dc0",
        ],
    );
    assert_absent_fields(
        &parsed.fields,
        "root.manager.",
        &["plane_map_7f0", "string_808", "raw_858", "display_maps"],
    );
    assert_absent_fields(
        &parsed.fields,
        "root.manager.owned_6e0.objects[0].scene.",
        &["hdr_exposure", "dspbr_tone_map", "field_of_view"],
    );
    for bytes in [
        NATIVE[0].0[..NATIVE[0].0.len() - 1].to_vec(),
        [NATIVE[0].0, &[0; 4]].concat(),
    ] {
        assert!(
            super::configuration_manager::parse(
                &bytes,
                ArchiveVersion::new(11000),
                super::annotation_view_tests::LIMITS
            )
            .is_err()
        );
    }
}

#[test]
fn intermediate_native_manager_enforces_the_independent_object_census_budget() {
    let limits = super::ModelArchiveLimits {
        maximum_objects: 349,
        ..LIMITS
    };
    let Err(error) =
        super::configuration_manager::parse(NATIVE[0].0, ArchiveVersion::new(11000), limits)
    else {
        panic!("expected object budget rejection");
    };
    assert!(error.cause.contains("350") && error.cause.contains("349"));
}

fn assert_absent_fields(fields: &[ModelField], prefix: &str, labels: &[&str]) {
    for label in labels {
        let path = format!("{prefix}{label}");
        assert!(!fields.iter().any(|f| f.path.starts_with(&path)), "{path}");
    }
}

#[test]
fn intermediate_spherical_scene_preserves_the_complete_fresh_visual_and_real_version() {
    let parsed =
        super::configuration_manager::parse(NATIVE[2].0, ArchiveVersion::new(11000), LIMITS)
            .unwrap();
    assert_eq!(parsed.version, ArchiveVersion::new(11000));
    let mut classes = std::collections::BTreeSet::new();
    for field in &parsed.fields {
        if let ModelFieldValue::ObjectTag(pointer) = &field.value
            && let crate::archive_codec::ObjectRecord::New { class, .. } = &pointer.record
        {
            classes.insert(class.name.as_str());
        }
    }
    assert!(classes.contains("swSphericalScene_c"));
    assert!(classes.contains("moVisualProperties_c"));
}
