use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use crate::archive_codec::ArchiveClass;

type AxisFixture = (&'static str, &'static [u8], &'static str, usize);
macro_rules! axis_fixture {
    ($kind:literal, $reads:literal) => {
        (
            $kind,
            include_bytes!(concat!("../fixtures/reference-axes/", $kind, ".bin")).as_slice(),
            include_str!(concat!("../fixtures/reference-axes/", $kind, ".tsv")),
            $reads,
        )
    };
}
const CASES: &[AxisFixture] = &[
    axis_fixture!("moRefAxisData_c", 103),
    axis_fixture!("moSurfaceAxisData_c", 104),
    axis_fixture!("moFacePtAxisData_c", 105),
    axis_fixture!("moPlanePtAxisData_c", 105),
    axis_fixture!("moPlaneInterAxisData_c", 105),
    axis_fixture!("moLineAxisData_c", 104),
    axis_fixture!("moConFaceAxisData_c", 104),
    axis_fixture!("moTwoPtsAxisData_c", 105),
    axis_fixture!("temporary-line", 141),
];

#[test]
fn all_complete_axis_data_classes_match_independent_fields_and_bytes() {
    for (kind, bytes, oracle, reads) in CASES {
        let fields = decode(19000, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, *reads);
        assert_eq!(
            super::encode_fields(&fields, 30000).unwrap(),
            *bytes,
            "{kind}"
        );
        assert!(fields.iter().any(|field| matches!(field.value,
            super::ModelFieldValue::F64(bits) if bits.bits() == 0x7ff8000000000042)));
    }
}

#[test]
fn axes_reject_every_truncated_prefix_trailing_bytes_old_context_and_null_data() {
    for (_, bytes, _, _) in CASES {
        for end in 0..bytes.len() {
            assert!(decode(19000, &bytes[..end]).is_err(), "prefix {end}");
        }
        assert!(decode(19000, &[*bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
    let body_end = CASES[0].1.len() - 112 - 6 - "moRefAxisData_c".len();
    let mut missing = CASES[0].1[..body_end].to_vec();
    missing.extend_from_slice(&[0, 0]);
    assert!(decode(19000, &missing).is_err());
}

#[test]
fn axis_data_edit_preserves_source_and_raw_float_bits() {
    let original = decode(19000, CASES[0].1).unwrap();
    let mut fields = original.clone();
    let field = fields
        .iter_mut()
        .find(|field| field.path.ends_with("raw_08"))
        .unwrap();
    field.value =
        super::ModelFieldValue::F64(crate::header::StoredDouble::from_bits(3.5f64.to_bits()));
    let edited = super::encode_fields(&fields, 30000).unwrap();
    let mut expected = CASES[0].1.to_vec();
    let start = expected.len() - 64;
    expected[start..start + 8].copy_from_slice(&3.5f64.to_le_bytes());
    assert_eq!(edited, expected);
    assert_eq!(decode(19000, &edited).unwrap(), fields);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), CASES[0].1);
}

#[test]
fn axis_suffixes_enforce_declared_references_and_preserve_aliases() {
    for (kind, classes) in SUFFIX_CASES {
        for valid in [true, false] {
            let bytes = suffix_wire(classes);
            let result = mapped_suffix(&bytes, kind, classes, valid);
            assert_eq!(result.is_ok(), valid, "{kind}");
            if let Ok(fields) = result {
                assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
            }
        }
    }
}

const SUFFIX_CASES: &[(&str, &[&str])] = &[
    ("moSurfaceAxisData_c", &["moCylinderRef_w"]),
    ("moFacePtAxisData_c", &["moSphereRef_w", "moPointRef_w"]),
    ("moPlanePtAxisData_c", &["moPlaneRef_w", "moPointRef_w"]),
    ("moPlaneInterAxisData_c", &["moPlaneRef_w", "moPlaneRef_w"]),
    ("moLineAxisData_c", &["moLineRef_w"]),
    ("moLineAxisData_c", &["moTempAxisRef_w"]),
    ("moConFaceAxisData_c", &["moCompFace_c"]),
    ("moTwoPtsAxisData_c", &["moPointRef_w", "moPointRef_w"]),
];

fn suffix_wire(classes: &[&str]) -> Vec<u8> {
    let mut bytes = vec![0; 112];
    for index in 1..=classes.len() {
        let reference = if classes[index - 1] == classes[0] {
            1
        } else {
            index as u16
        };
        bytes.extend_from_slice(&reference.to_le_bytes());
    }
    bytes
}

fn mapped_suffix(
    bytes: &[u8],
    kind: &str,
    classes: &[&str],
    valid: bool,
) -> crate::Result<Vec<super::ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        for class in classes {
            map.append_loaded_object(Some(ArchiveClass {
                name: if valid { *class } else { "moNodeName_c" }.into(),
                schema: 1,
            }))?;
        }
        super::reference_axes::data(reader, map, kind)
    })
}

#[test]
fn unknown_axis_data_class_rejects_before_consuming_payload() {
    assert!(
        decode_collection(19000, &[0; 112], |reader, map| {
            super::reference_axes::data(reader, map, "moRefAxisData_c_suffix")
        })
        .is_err()
    );
}
