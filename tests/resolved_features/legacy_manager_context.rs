use super::configuration_manager::{parse, require_manager_scope};
use super::manager_dialect::ManagerDialect;
use super::reader::ModelArchiveContext;
use super::{ModelArchiveLimits, ModelDecoder, encode_fields};
use crate::header::ArchiveVersion;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 100_000,
    maximum_fields: 30_000,
    maximum_entries: 30_000,
    maximum_depth: 128,
    maximum_objects: 3_000,
};

#[test]
fn dialect_selection_accepts_only_the_three_independently_reviewed_versions() {
    for (version, dialect) in [
        (11000, ManagerDialect::Owned11000),
        (18000, ManagerDialect::Owned18000),
        (19000, ManagerDialect::Current19000),
    ] {
        assert_eq!(
            ManagerDialect::from_version(ArchiveVersion::new(version)).unwrap(),
            dialect
        );
        assert_eq!(dialect.version(), version);
    }
    for version in [
        0,
        10999,
        11001,
        17999,
        18001,
        0x46c1,
        18999,
        19001,
        u32::MAX,
    ] {
        assert!(ManagerDialect::from_version(ArchiveVersion::new(version)).is_err());
    }
}

#[test]
fn legacy_scope_requires_explicit_matching_dialect_and_ordinary_part_context() {
    for version in [17999, 18000, 18001, 19000] {
        let mut reader = ModelDecoder::new_in_context(
            &[],
            ArchiveVersion::new(version),
            LIMITS,
            ModelArchiveContext::OrdinaryPartExternalXml,
        )
        .unwrap()
        .with_manager_dialect(ManagerDialect::Owned18000);
        assert_eq!(require_manager_scope(&mut reader).is_ok(), version == 18000);
    }
    let mut unselected = ModelDecoder::new_in_context(
        &[],
        ArchiveVersion::new(18000),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap();
    assert!(require_manager_scope(&mut unselected).is_err());
    let mut wrong_context = ModelDecoder::new_in_context(
        &[],
        ArchiveVersion::new(18000),
        LIMITS,
        ModelArchiveContext::Unspecified,
    )
    .unwrap()
    .with_manager_dialect(ManagerDialect::Owned18000);
    assert!(require_manager_scope(&mut wrong_context).is_err());
}

const CASES: &[(&[u8], &str)] = &[
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-0.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-0-primitives.tsv"),
    ),
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-1.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-1-primitives.tsv"),
    ),
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-2.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-2-primitives.tsv"),
    ),
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-3.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-3-primitives.tsv"),
    ),
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-4.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-4-primitives.tsv"),
    ),
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-5.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-5-primitives.tsv"),
    ),
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-6.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-6-primitives.tsv"),
    ),
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-7.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-7-primitives.tsv"),
    ),
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-8.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-8-primitives.tsv"),
    ),
    (
        include_bytes!("../fixtures/legacy-owned-managers/manager-9.bin"),
        include_str!("../fixtures/legacy-owned-managers/manager-9-primitives.tsv"),
    ),
];

#[test]
fn legacy_native_managers_reconstruct_every_independently_observed_field() {
    for (bytes, oracle) in CASES {
        let parsed = parse(bytes, ArchiveVersion::new(18000), LIMITS).unwrap();
        assert_eq!(parsed.dialect, ManagerDialect::Owned18000);
        super::manager_oracle_tests::assert_reference_fields(
            &parsed.fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(
            encode_fields(&parsed.fields, LIMITS.maximum_bytes).unwrap(),
            *bytes
        );
        assert!(
            !parsed
                .fields
                .iter()
                .any(|field| field.path.ends_with("display_maps_presence"))
        );
    }
}

#[test]
fn legacy_managers_reject_truncation_modern_tail_and_wrong_version() {
    for (bytes, _) in CASES {
        assert!(
            parse(
                &bytes[..bytes.len() - 1],
                ArchiveVersion::new(18000),
                LIMITS
            )
            .is_err()
        );
        let mut modern_tail = bytes.to_vec();
        modern_tail.extend_from_slice(&0_u32.to_le_bytes());
        assert!(parse(&modern_tail, ArchiveVersion::new(18000), LIMITS).is_err());
        assert!(parse(bytes, ArchiveVersion::new(19000), LIMITS).is_err());
        let bounded = ModelArchiveLimits {
            maximum_bytes: bytes.len() - 1,
            ..LIMITS
        };
        assert!(parse(bytes, ArchiveVersion::new(18000), bounded).is_err());
    }
}
