use super::ModelObjectRole;
use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use crate::archive_codec::ArchiveClass;

type AxisCase = (u32, &'static [u8], &'static str);
const CASES: &[AxisCase] = &[
    (
        0x27cc,
        include_bytes!("../fixtures/reference-axis-owners/axis-27cc-0.bin"),
        include_str!("../fixtures/reference-axis-owners/axis-27cc-0.tsv"),
    ),
    (
        0x27cc,
        include_bytes!("../fixtures/reference-axis-owners/axis-27cc-1.bin"),
        include_str!("../fixtures/reference-axis-owners/axis-27cc-1.tsv"),
    ),
    (
        19000,
        include_bytes!("../fixtures/reference-axis-owners/axis-4a38-0.bin"),
        include_str!("../fixtures/reference-axis-owners/axis-4a38-0.tsv"),
    ),
    (
        19000,
        include_bytes!("../fixtures/reference-axis-owners/axis-4a38-1.bin"),
        include_str!("../fixtures/reference-axis-owners/axis-4a38-1.tsv"),
    ),
];

#[test]
fn reference_axes_are_valid_new_and_cached_atom_feature_owners() {
    for &(version, bytes, _) in CASES {
        let with_alias = [bytes, &2u16.to_le_bytes()].concat();
        let fields = decode_collection(version, &with_alias, |reader, map| {
            reader.object(
                "owner",
                map,
                ModelObjectRole::Declared(super::body_features::CLASSES),
            )?;
            reader.object(
                "alias",
                map,
                ModelObjectRole::Declared(super::body_features::CLASSES),
            )
        })
        .unwrap();
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), with_alias);
    }
}

#[test]
fn populated_reference_axes_match_independent_primitives_and_reject_incomplete_bytes() {
    for &(version, bytes, oracle) in CASES {
        let fields = decode(version, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            oracle,
            oracle.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        for end in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..end]).is_err(),
                "{version:#x} prefix {end}"
            );
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
    }
}

#[test]
fn unrelated_new_and_cached_classes_remain_invalid_atom_feature_owners() {
    for name in ["moNodeName_c", "moRefWrapper_c", "moRefAxis_c_suffix"] {
        let mut fresh = [
            u16::MAX.to_le_bytes(),
            1u16.to_le_bytes(),
            (name.len() as u16).to_le_bytes(),
        ]
        .concat();
        fresh.extend_from_slice(name.as_bytes());
        let new_error = decode_collection(19000, &fresh, |reader, map| {
            reader.object(
                "owner",
                map,
                ModelObjectRole::Declared(super::body_features::CLASSES),
            )
        })
        .unwrap_err();
        let cached_error = decode_collection(19000, &[1, 0], |reader, map| {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
            reader.object(
                "owner",
                map,
                ModelObjectRole::Declared(super::body_features::CLASSES),
            )
        })
        .unwrap_err();
        for error in [new_error, cached_error] {
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains(name), "{}", error.cause);
        }
    }
}
