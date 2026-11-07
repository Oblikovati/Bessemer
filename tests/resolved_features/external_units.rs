use super::annotation_view_tests::LIMITS;
use super::component_detail_provenance_tests::source_archive;
use super::{ModelDecoder, ModelFieldValue, PartConfigurationArchive, PartResolvedFeaturesArchive};
use crate::archive_codec::ObjectRecord;
use crate::header::ArchiveVersion;
const CASES: &[(&[u8], &str, usize, usize)] = &[
    (
        include_bytes!("../fixtures/external-unit-components/owned-0.bin"),
        include_str!("../fixtures/external-unit-components/owned-0.tsv"),
        8,
        99,
    ),
    (
        include_bytes!("../fixtures/external-unit-components/owned-1.bin"),
        include_str!("../fixtures/external-unit-components/owned-1.tsv"),
        38,
        101,
    ),
    (
        include_bytes!("../fixtures/external-unit-components/owned-2.bin"),
        include_str!("../fixtures/external-unit-components/owned-2.tsv"),
        39,
        101,
    ),
    (
        include_bytes!("../fixtures/external-unit-components/owned-3.bin"),
        include_str!("../fixtures/external-unit-components/owned-3.tsv"),
        39,
        101,
    ),
    (
        include_bytes!("../fixtures/external-unit-components/owned-4.bin"),
        include_str!("../fixtures/external-unit-components/owned-4.tsv"),
        69,
        103,
    ),
    (
        include_bytes!("../fixtures/external-unit-components/owned-5.bin"),
        include_str!("../fixtures/external-unit-components/owned-5.tsv"),
        47,
        103,
    ),
    (
        include_bytes!("../fixtures/external-unit-components/owned-2-edited.bin"),
        include_str!("../fixtures/external-unit-components/owned-2-edited.tsv"),
        39,
        101,
    ),
];
fn parse(
    bytes: &[u8],
    source: &PartConfigurationArchive,
) -> crate::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_bytes(
        bytes,
        source.clone(),
        include_bytes!("../fixtures/sheet-metal/manager-0.bin"),
        LIMITS,
    )
}

#[test]
fn complete_external_units_match_all_independent_primitives_bytes_and_slots() {
    let source = source_archive();
    for (bytes, oracle, reads, slots) in CASES {
        let archive = parse(bytes, &source).unwrap();
        super::manager_oracle_tests::assert_reference_fields(
            archive.fields(),
            bytes,
            oracle,
            *reads,
        );
        assert_eq!(archive.to_bytes().unwrap(), *bytes);
        assert_eq!(archive.object_slots(), (97, *slots));
    }
}

#[test]
fn external_units_reject_every_prefix_and_appended_byte() {
    let source = source_archive();
    for (bytes, _, _, _) in CASES {
        for length in 0..bytes.len() {
            assert!(parse(&bytes[..length], &source).is_err());
        }
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert!(parse(&extra, &source).is_err());
    }
}

#[test]
fn external_units_require_completed_part_context_but_internal_null_units_do_not() {
    assert!(super::solid_reference_tests::decode_graph(19000, &CASES[0].0[6..], false).is_ok());
    for (bytes, _, _, _) in &CASES[1..5] {
        let Err(error) = super::solid_reference_tests::decode_graph(19000, &bytes[6..], false)
        else {
            panic!("external unit accepted missing context")
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.cause.contains("current-part proof None"));
    }
}

#[test]
fn external_model_pointer_is_nonnull_and_class_checked() {
    let source = source_archive();
    let fields = parse(CASES[2].0, &source).unwrap().fields().to_vec();
    for tag in [0_u16, 98] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.features[0].external_model",
            &tag.to_le_bytes(),
        );
        assert!(parse(&changed, &source).is_err());
    }
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.features[0].owner",
        &98_u16.to_le_bytes(),
    );
    assert!(parse(&changed, &source).is_err());
}

#[test]
fn internal_component_detail_provenance_still_rejects_external_units() {
    let source = source_archive();
    for index in [1, 2] {
        let archive = parse(CASES[index].0, &source).unwrap();
        let ModelFieldValue::ObjectTag(tag) = &archive.fields()[2].value else {
            panic!("expected root unit tag")
        };
        assert!(matches!(tag.record, ObjectRecord::New { .. }));
        assert!(
            super::component_detail_provenance::require_internal_unit(
                archive.fields(),
                &tag.record,
                Some(source.parsed.verified_root())
            )
            .is_err()
        );
    }
}

#[test]
fn verified_part_context_guard_accepts_only_completed_part_proof() {
    let mut reader = ModelDecoder::new_in_context(
        &[],
        ArchiveVersion::new(19000),
        LIMITS,
        super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap();
    assert!(reader.require_verified_part_context().is_err());
    let source = source_archive();
    let mut reader = reader.with_verified_part_root(source.parsed.verified_root());
    assert!(reader.require_verified_part_context().is_ok());
}
