use super::direct_collection_tests::decode_collection;
use super::{ModelDecoder, ModelField, reference_planes::PlaneDataKind};
use crate::Result;
use crate::archive_codec::{ArchiveClass, ObjectReader};

type RoleFixture = (u8, &'static [u8], &'static str, usize);
macro_rules! role_fixture {
    ($kind:literal, $name:literal, $reads:literal) => {
        (
            $kind,
            include_bytes!(concat!("../fixtures/line-reference-roles/", $name, ".bin")).as_slice(),
            include_str!(concat!("../fixtures/line-reference-roles/", $name, ".tsv")),
            $reads,
        )
    };
}
const CASES: &[RoleFixture] = &[
    role_fixture!(0, "revolution", 10),
    role_fixture!(1, "angle-plane", 70),
    role_fixture!(2, "curve-plane", 31),
    role_fixture!(3, "sketch-pattern", 23),
    role_fixture!(4, "intersection", 23),
];

fn read_role(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, kind: u8) -> Result<()> {
    match kind {
        0 => super::body_end_specs::revolution(reader, map),
        1 => super::reference_planes::data(reader, map, PlaneDataKind::Angle),
        2 => super::reference_planes::data(reader, map, PlaneDataKind::LinePoint),
        3 => super::sketch_patterns::linear(reader, map),
        4 => super::sketch_intersections::intersection(reader, map),
        _ => panic!("role fixture {kind}; expected 0..=4"),
    }
}

fn decode_role(kind: u8, bytes: &[u8], class: &str) -> Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        map.append_loaded_object(Some(ArchiveClass {
            name: class.into(),
            schema: 1,
        }))?;
        read_role(reader, map, kind)
    })
}

#[test]
fn line_roles_accept_both_documented_classes_with_exact_independent_fields() {
    for (kind, bytes, oracle, count) in CASES {
        for class in ["moLineRef_w", "moTempAxisRef_w"] {
            let fields = decode_role(*kind, bytes, class).unwrap();
            super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, *count);
            assert_eq!(super::encode_fields(&fields, 30000).unwrap(), *bytes);
        }
    }
}

#[test]
fn line_roles_reject_unrelated_classes_and_name_suffix_lookalikes() {
    for (kind, bytes, _, _) in CASES {
        for class in ["moSurfaceRef_w", "moTempAxisRef_w_suffix", "moNodeName_c"] {
            let error = decode_role(*kind, bytes, class).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains(class));
        }
    }
}

#[test]
fn line_roles_reject_truncation_and_trailing_bytes() {
    for (kind, bytes, _, _) in CASES {
        for end in 0..bytes.len() {
            assert!(decode_role(*kind, &bytes[..end], "moTempAxisRef_w").is_err());
        }
        assert!(decode_role(*kind, &[*bytes, &[0]].concat(), "moTempAxisRef_w").is_err());
    }
}
