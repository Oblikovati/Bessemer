use super::manager_dialect::ManagerDialect;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldValue, encode_fields};
use crate::archive_codec::ObjectReader;
use crate::header::ArchiveVersion;
fn decode(bytes: &[u8]) -> crate::Result<(Vec<ModelField>, usize)> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(11000),
        super::annotation_view_tests::LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?
    .with_manager_dialect(ManagerDialect::Owned11000);
    let mut map = ObjectReader::new();
    super::manager_tail::owned_suffix(&mut reader, &mut map)?;
    Ok((reader.finish()?, map.into_snapshot().slot_count()))
}
const TAILS: &[(&[u8], &str, usize)] = &[
    (
        include_bytes!("../fixtures/intermediate-managers/tail-0.bin"),
        include_str!("../fixtures/intermediate-managers/tail-0.tsv"),
        8,
    ),
    (
        include_bytes!("../fixtures/intermediate-managers/tail-1.bin"),
        include_str!("../fixtures/intermediate-managers/tail-1.tsv"),
        8,
    ),
    (
        include_bytes!("../fixtures/intermediate-managers/tail-2.bin"),
        include_str!("../fixtures/intermediate-managers/tail-2.tsv"),
        27,
    ),
    (
        include_bytes!("../fixtures/intermediate-managers/tail-3.bin"),
        include_str!("../fixtures/intermediate-managers/tail-3.tsv"),
        31,
    ),
    (
        include_bytes!("../fixtures/intermediate-managers/tail-4.bin"),
        include_str!("../fixtures/intermediate-managers/tail-4.tsv"),
        9,
    ),
    (
        include_bytes!("../fixtures/intermediate-managers/tail-5.bin"),
        include_str!("../fixtures/intermediate-managers/tail-5.tsv"),
        28,
    ),
];
#[test]
fn intermediate_owned_suffixes_match_independent_primitives_and_original_unsigned_values() {
    for (index, (bytes, oracle, reads)) in TAILS.iter().enumerate() {
        let (fields, slots) = decode(bytes).unwrap();
        assert_eq!(slots, 1);
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, *reads);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), *bytes);
        assert_eq!(
            fields.last().unwrap().value,
            ModelFieldValue::U32([0, 2, u32::MAX, 0x80000000, 255, u32::MAX][index])
        );
    }
    assert_eq!(TAILS[0].0.len(), 30);
}
#[test]
fn intermediate_owned_suffixes_reject_every_prefix_and_added_modern_bytes() {
    for (bytes, _, _) in TAILS {
        for end in 0..bytes.len() {
            assert!(decode(&bytes[..end]).is_err());
        }
        let mut extra = bytes.to_vec();
        extra.extend_from_slice(&[0; 4]);
        assert!(decode(&extra).is_err());
    }
}
#[test]
fn intermediate_owned_factories_reject_fresh_decal_and_conversion_bodies_before_loading() {
    for (bytes, name) in [
        (
            include_bytes!("../fixtures/intermediate-managers/fresh-decal.bin").as_slice(),
            "moDecalProperties_c",
        ),
        (
            include_bytes!("../fixtures/intermediate-managers/fresh-conversion.bin").as_slice(),
            "moConvertFeatIds_c",
        ),
    ] {
        let Err(error) = decode(bytes) else {
            panic!("expected unsupported fresh dependency");
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(
            error.cause.contains(name)
                && error
                    .cause
                    .contains("expected null or an already loaded object")
        );
    }
}
