use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::{ModelFieldValue, encode_fields};

type PolylineFixture = (&'static [u8], &'static str, usize);
const PROJECT: &[u8] = include_bytes!("../fixtures/resolved-features/polyline-project.bin");
const CASES: &[PolylineFixture] = &[
    (
        include_bytes!("../fixtures/resolved-features/polyline-base.bin"),
        include_str!("../fixtures/resolved-features/polyline-base-primitives.tsv"),
        96,
    ),
    (
        PROJECT,
        include_str!("../fixtures/resolved-features/polyline-project-primitives.tsv"),
        119,
    ),
    (
        include_bytes!("../fixtures/resolved-features/polyline-edited.bin"),
        include_str!("../fixtures/resolved-features/polyline-edited-primitives.tsv"),
        119,
    ),
    (
        include_bytes!("../fixtures/resolved-features/polyline-words.bin"),
        include_str!("../fixtures/resolved-features/polyline-words-primitives.tsv"),
        9,
    ),
];

#[test]
fn polylines_match_independent_primitive_partitions_and_reconstruct_exactly() {
    for (bytes, rows, count) in CASES {
        let fields = decode(19000, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, rows, *count);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), *bytes);
    }
}

#[test]
fn polyline_truncations_and_extra_bytes_cannot_publish_partial_results() {
    for (bytes, _, _) in CASES {
        for length in 0..bytes.len() {
            assert!(decode(19000, &bytes[..length]).is_err(), "prefix {length}");
        }
        assert!(decode(19000, &[*bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

#[test]
fn polyline_edit_matches_independent_bytes_without_changing_input() {
    let mut input = PROJECT.to_vec();
    let fields = decode(19000, &input).unwrap();
    input.fill(0xa5);
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.raw_814",
        &(-990_i32).to_le_bytes(),
    );
    assert_eq!(changed, CASES[2].0);
    assert_eq!(encode_fields(&fields, 30000).unwrap(), PROJECT);
    let edited = decode(19000, &changed).unwrap();
    let raw = edited
        .iter()
        .find(|field| field.path.as_ref() == "root.object.raw_814")
        .unwrap();
    assert_eq!(raw.value, ModelFieldValue::I32(-990));
}

#[test]
fn polyline_required_data_and_owner_reject_null_or_incompatible_classes() {
    let fields = decode(19000, PROJECT).unwrap();
    for path in ["root.object.data_808", "root.object.data_808.owner"] {
        let null = super::manager_context_tests::replace_field(&fields, path, &[0, 0]);
        assert!(decode(19000, &null).is_err());
        let wrong = super::manager_context_tests::replace_field(&fields, path, &[4, 0]);
        assert!(decode(19000, &wrong).is_err());
    }
}

#[test]
fn polyline_keeper_rejects_negative_and_impossible_surface_counts() {
    for count in [-1_i32, i32::MAX, 1] {
        assert!(
            decode_collection(19000, &count.to_le_bytes(), super::polyline_records::keeper)
                .is_err()
        );
    }
    assert!(decode_collection(19000, &[0, 0], super::polyline_records::data).is_err());
}
