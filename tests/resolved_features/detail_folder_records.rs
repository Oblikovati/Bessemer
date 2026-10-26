use super::annotation_view_tests::decode;

type DetailFolderFixture = (&'static [u8], &'static str, usize);
const CASES: &[DetailFolderFixture] = &[
    (
        include_bytes!("../fixtures/detail-folders/absent.bin"),
        include_str!("../fixtures/detail-folders/absent-primitives.tsv"),
        95,
    ),
    (
        include_bytes!("../fixtures/detail-folders/bucket-3d.bin"),
        include_str!("../fixtures/detail-folders/bucket-3d-primitives.tsv"),
        131,
    ),
    (
        include_bytes!("../fixtures/detail-folders/bucket-2d.bin"),
        include_str!("../fixtures/detail-folders/bucket-2d-primitives.tsv"),
        152,
    ),
    (
        include_bytes!("../fixtures/detail-folders/bucket-populated.bin"),
        include_str!("../fixtures/detail-folders/bucket-populated-primitives.tsv"),
        429,
    ),
    (
        include_bytes!("../fixtures/detail-folders/named.bin"),
        include_str!("../fixtures/detail-folders/named-primitives.tsv"),
        102,
    ),
    (
        include_bytes!("../fixtures/detail-folders/named-edited.bin"),
        include_str!("../fixtures/detail-folders/named-edited-primitives.tsv"),
        102,
    ),
];

#[test]
fn complete_detail_folders_match_independent_fields_and_exact_bytes() {
    for (bytes, rows, count) in CASES {
        let fields = decode(19000, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, rows, *count);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), *bytes);
    }
}

#[test]
fn detail_folder_prefixes_and_trailing_bytes_cannot_publish_partial_objects() {
    for (bytes, _, _) in CASES {
        for prefix in 0..bytes.len() {
            assert!(decode(19000, &bytes[..prefix]).is_err(), "prefix {prefix}");
        }
        assert!(decode(19000, &[*bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

#[test]
fn detail_folder_name_edit_matches_independent_expected_bytes() {
    use super::{ArchiveString, ModelFieldValue, TextEncoding};
    let original = decode(19000, CASES[4].0).unwrap();
    let mut fields = original.clone();
    let name = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.feature.node.name.name")
        .unwrap();
    name.value =
        ModelFieldValue::Text(ArchiveString::new("Detail Changed Ω", TextEncoding::Utf16).unwrap());
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), CASES[5].0);
    assert_eq!(decode(19000, CASES[5].0).unwrap(), fields);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), CASES[4].0);
}

#[test]
fn model_items_cover_text_widths_transforms_and_checked_collections() {
    let bytes = include_bytes!("../fixtures/detail-folders/items.bin");
    let fields = super::direct_collection_tests::decode_collection(19000, bytes, |reader, _| {
        crate::archive_codec::display_items::list(reader)
    })
    .unwrap();
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    assert!(fields.iter().any(
        |field| matches!(&field.value, super::ModelFieldValue::Text(text) if text.text() == "é😀")
    ));
    assert!(
        fields
            .iter()
            .any(|field| field.path.ends_with("unknown_48[1]"))
    );
    for prefix in [0, 1, bytes.len() / 2, bytes.len() - 1] {
        assert!(
            super::direct_collection_tests::decode_collection(
                19000,
                &bytes[..prefix],
                |reader, _| { crate::archive_codec::display_items::list(reader) }
            )
            .is_err()
        );
    }
}

#[test]
fn detail_folder_rejects_impossible_counts_wrong_bom_and_missing_bucket() {
    let fields = decode(19000, CASES[0].0).unwrap();
    for (path, wire) in [
        (
            "root.object.lists.350.count.count_dword",
            &[255, 255, 255, 255][..],
        ),
        ("root.object.bom", &[2, 0][..]),
        ("root.object.bucket_presence", &[255, 255, 255, 255][..]),
    ] {
        let bytes = super::manager_context_tests::replace_field(&fields, path, wire);
        assert!(decode(19000, &bytes).is_err(), "{path}");
    }
}

#[test]
fn detail_folder_accepts_only_the_declared_loaded_bom_class() {
    use crate::archive_codec::ArchiveClass;
    let transform = [&[0][..], &[0; 32], &[0]].concat();
    let bytes = [vec![0; 175], transform, vec![1, 0], vec![0; 40]].concat();
    for (name, accepted) in [
        ("moDisplayBom_c", true),
        ("moVisualProperties_c", false),
        ("moDisplayBom_c_suffix", false),
    ] {
        let result =
            super::direct_collection_tests::decode_collection(19000, &bytes, |reader, map| {
                map.append_loaded_object(Some(ArchiveClass {
                    name: name.into(),
                    schema: 1,
                }))?;
                super::detail_folder_records::read(reader, map)
            });
        assert_eq!(result.is_ok(), accepted, "{name}");
        if let Ok(fields) = result {
            assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        }
    }
}
