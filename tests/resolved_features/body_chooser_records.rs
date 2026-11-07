use super::annotation_view_tests::decode;
use super::{ModelFieldValue, ModelObjectRecord};

type FixtureCase = (&'static [u8], &'static str, usize);
const CASES: &[FixtureCase] = &[
    (
        include_bytes!("../fixtures/resolved-features/body-chooser.bin"),
        include_str!("../fixtures/resolved-features/body-chooser-primitives.tsv"),
        18,
    ),
    (
        include_bytes!("../fixtures/resolved-features/named-body-chooser.bin"),
        include_str!("../fixtures/resolved-features/named-body-chooser-primitives.tsv"),
        24,
    ),
    (
        include_bytes!("../fixtures/resolved-features/body-bounding-center.bin"),
        include_str!("../fixtures/resolved-features/body-bounding-center-primitives.tsv"),
        9,
    ),
];

#[test]
fn complete_chooser_variants_match_independent_primitives_and_shared_root_identities() {
    for (bytes, oracle, reads) in CASES {
        let fields = decode(19000, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, *reads);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), *bytes);
    }
    let fields = decode(19000, CASES[0].0).unwrap();
    let aliases = fields
        .iter()
        .filter(|field| {
            matches!(&field.value, ModelFieldValue::ObjectTag(tag)
        if matches!(&tag.record, ModelObjectRecord::Reference { index, .. } if index.value() == 2))
        })
        .count();
    assert_eq!(aliases, 2);
}

#[test]
fn chooser_prefixes_and_appended_bytes_never_publish_partial_objects() {
    for (bytes, _, _) in CASES {
        for length in 0..bytes.len() {
            assert!(decode(19000, &bytes[..length]).is_err(), "prefix {length}");
        }
        assert!(decode(19000, &[*bytes, &[0]].concat()).is_err());
    }
}

#[test]
fn chooser_typed_filename_reference_and_impossible_collection_counts_reject() {
    let mut wrong_kind = CASES[1].0.to_vec();
    let reference = wrong_kind.len() - 6;
    wrong_kind[reference..reference + 2].copy_from_slice(&2u16.to_le_bytes());
    assert!(decode(19000, &wrong_kind).is_err());
    let mut missing = CASES[0].0.to_vec();
    let count = 6 + "moPerBodyChooserData_c".len();
    missing[count + 2..count + 4].copy_from_slice(&300u16.to_le_bytes());
    assert!(decode(19000, &missing).is_err());
    let oversized = [
        &CASES[0].0[..count],
        &u16::MAX.to_le_bytes(),
        &u32::MAX.to_le_bytes(),
        &CASES[0].0[count + 2..],
    ]
    .concat();
    assert!(decode(19000, &oversized).is_err());
}

#[test]
fn chooser_edit_matches_independent_expected_bytes_and_preserves_original_fields() {
    let mut input = CASES[0].0.to_vec();
    let original = decode(19000, &input).unwrap();
    input.fill(0);
    let mut changed = original.clone();
    changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.raw_70")
        .unwrap()
        .value = ModelFieldValue::I32(i32::MAX);
    let expected = include_bytes!("../fixtures/resolved-features/body-chooser-edited.bin");
    assert_eq!(super::encode_fields(&changed, 30000).unwrap(), expected);
    assert_eq!(decode(19000, expected).unwrap(), changed);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), CASES[0].0);
}
