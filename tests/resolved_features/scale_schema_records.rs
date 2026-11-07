use super::annotation_view_tests::decode;
use super::{ModelFieldValue, encode_fields};

type ScaleSchemaFixture = (&'static [u8], &'static str, usize);
const COORDINATE: &[u8] = include_bytes!("../fixtures/resolved-features/scale-coordinate.bin");
const CASES: &[ScaleSchemaFixture] = &[
    (
        include_bytes!("../fixtures/resolved-features/scale-empty.bin"),
        include_str!("../fixtures/resolved-features/scale-empty-primitives.tsv"),
        89,
    ),
    (
        COORDINATE,
        include_str!("../fixtures/resolved-features/scale-coordinate-primitives.tsv"),
        117,
    ),
    (
        include_bytes!("../fixtures/resolved-features/scale-edited.bin"),
        include_str!("../fixtures/resolved-features/scale-edited-primitives.tsv"),
        117,
    ),
    (
        include_bytes!("../fixtures/resolved-features/schema-unicode.bin"),
        include_str!("../fixtures/resolved-features/schema-unicode-primitives.tsv"),
        9,
    ),
    (
        include_bytes!("../fixtures/resolved-features/schema-empty.bin"),
        include_str!("../fixtures/resolved-features/schema-empty-primitives.tsv"),
        6,
    ),
];

#[test]
fn scale_and_schema_bodies_match_independent_primitive_partitions() {
    for (bytes, rows, count) in CASES {
        let fields = decode(19000, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, rows, *count);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), *bytes);
    }
}

#[test]
fn scale_and_schema_prefixes_trailing_bytes_and_wrong_context_reject() {
    for (bytes, _, _) in CASES {
        for prefix in 0..bytes.len() {
            assert!(decode(19000, &bytes[..prefix]).is_err());
        }
        assert!(decode(19000, &[*bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

#[test]
fn scale_edit_matches_independent_bytes_and_preserves_input() {
    let mut input = COORDINATE.to_vec();
    let fields = decode(19000, &input).unwrap();
    input.fill(0xa5);
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.raw_808",
        &(-22_i32).to_le_bytes(),
    );
    assert_eq!(changed, CASES[2].0);
    assert_eq!(encode_fields(&fields, 30000).unwrap(), COORDINATE);
    let edited = decode(19000, &changed).unwrap();
    assert_eq!(
        edited
            .iter()
            .find(|field| field.path.as_ref() == "root.object.raw_808")
            .unwrap()
            .value,
        ModelFieldValue::I32(-22)
    );
}

#[test]
fn scale_coordinate_pointer_keeps_shared_identity_and_rejects_wrong_class() {
    let fields = decode(19000, COORDINATE).unwrap();
    let wrong = super::manager_context_tests::replace_field(
        &fields,
        "root.object.coordinate_system_828",
        &[2, 0],
    );
    assert!(decode(19000, &wrong).is_err());
    let pointer = fields
        .iter()
        .find(|field| field.path.as_ref() == "root.object.array_830[0]")
        .unwrap();
    let ModelFieldValue::ObjectTag(tag) = &pointer.value else {
        panic!("expected object pointer")
    };
    let super::ModelObjectRecord::Reference { index, .. } = &tag.record else {
        panic!("expected shared reference")
    };
    assert_eq!(index.value(), 4);
}
