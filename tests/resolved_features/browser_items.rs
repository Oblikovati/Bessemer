use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::{ModelFieldValue, ModelObjectRecord};
use crate::archive_codec::ArchiveClass;

type BrowserFixture = (&'static [u8], &'static str, usize);
const CASES: &[BrowserFixture] = &[
    (
        include_bytes!("../fixtures/resolved-features/moFavoriteItemData_c.bin"),
        include_str!("../fixtures/resolved-features/moFavoriteItemData_c-primitives.tsv"),
        10,
    ),
    (
        include_bytes!("../fixtures/resolved-features/moHistoryItemData_c.bin"),
        include_str!("../fixtures/resolved-features/moHistoryItemData_c-primitives.tsv"),
        10,
    ),
    (
        include_bytes!("../fixtures/resolved-features/moFavoriteFeatItemData_c.bin"),
        include_str!("../fixtures/resolved-features/moFavoriteFeatItemData_c-primitives.tsv"),
        11,
    ),
    (
        include_bytes!("../fixtures/resolved-features/moHistoryFeatItemData_c.bin"),
        include_str!("../fixtures/resolved-features/moHistoryFeatItemData_c-primitives.tsv"),
        11,
    ),
    (
        include_bytes!("../fixtures/resolved-features/moLiveSectionFolder_c.bin"),
        include_str!("../fixtures/resolved-features/moLiveSectionFolder_c-primitives.tsv"),
        51,
    ),
    (
        include_bytes!("../fixtures/resolved-features/moPlaneSectionFolder_c.bin"),
        include_str!("../fixtures/resolved-features/moPlaneSectionFolder_c-primitives.tsv"),
        51,
    ),
    (
        include_bytes!("../fixtures/resolved-features/moConfigCommentsFolder_c.bin"),
        include_str!("../fixtures/resolved-features/moConfigCommentsFolder_c-primitives.tsv"),
        50,
    ),
];

#[test]
fn final_browser_item_classes_match_complete_independent_reference_primitives() {
    for (bytes, oracle, reads) in CASES {
        let fields = decode(19000, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, *reads);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), *bytes);
    }
}

#[test]
fn browser_item_and_section_prefixes_and_appended_bytes_cannot_publish_partial_fields() {
    for (bytes, _, _) in CASES {
        for length in 0..bytes.len() {
            assert!(decode(19000, &bytes[..length]).is_err());
        }
        assert!(decode(19000, &[*bytes, &[0]].concat()).is_err());
    }
}

#[test]
fn browser_feature_items_keep_the_existing_finite_component_feature_family() {
    let bytes = [vec![0; 19], vec![1, 0]].concat();
    for (name, accepted) in [
        ("moCompFeature_c", true),
        ("moCompBodyFeature_c", true),
        ("moCompSolidBody_c", false),
        ("moHistoryFeatItemData_c", false),
    ] {
        let result = decode_collection(19000, &bytes, |reader, map| {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
            super::browser_folders::feature_item(reader, map)
        });
        assert_eq!(result.is_ok(), accepted, "{name}");
        if let Ok(fields) = result {
            assert!(
                matches!(&fields.last().unwrap().value, ModelFieldValue::ObjectTag(tag)
                if matches!(&tag.record, ModelObjectRecord::Reference { class, .. } if class.name == name))
            );
        }
    }
}

#[test]
fn section_raw_word_edit_matches_independent_bytes_without_changing_owned_inputs() {
    let bytes = CASES[4].0;
    let original = decode(19000, bytes).unwrap();
    let mut fields = original.clone();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.raw_370")
        .unwrap()
        .value = ModelFieldValue::I32(i32::MIN);
    let mut expected = bytes.to_vec();
    let tail = expected.len() - 4;
    expected[tail..].copy_from_slice(&i32::MIN.to_le_bytes());
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), expected);
    assert_eq!(decode(19000, &expected).unwrap(), fields);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), bytes);
}
