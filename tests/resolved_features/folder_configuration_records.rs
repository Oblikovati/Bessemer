use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::{ModelField, encode_fields};
use crate::Result;
use crate::archive_codec::ArchiveClass;

type FolderConfigurationFixture = (&'static [u8], &'static str, usize);
const CASES: &[FolderConfigurationFixture] = &[
    (
        include_bytes!("../fixtures/resolved-features/cut-list-sort.bin"),
        include_str!("../fixtures/resolved-features/cut-list-sort-primitives.tsv"),
        9,
    ),
    (
        include_bytes!("../fixtures/resolved-features/cut-list-sort-edited.bin"),
        include_str!("../fixtures/resolved-features/cut-list-sort-edited-primitives.tsv"),
        9,
    ),
    (
        include_bytes!("../fixtures/resolved-features/face-color-configuration.bin"),
        include_str!("../fixtures/resolved-features/face-color-configuration-primitives.tsv"),
        7,
    ),
];

#[test]
fn folder_configuration_records_match_independent_primitives_and_edit_bytes() {
    for (bytes, rows, count) in CASES {
        let fields = decode(19000, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, rows, *count);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), *bytes);
    }
    let fields = decode(19000, CASES[0].0).unwrap();
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.raw_08",
        &(-77_i32).to_le_bytes(),
    );
    assert_eq!(changed, CASES[1].0);
    assert_eq!(encode_fields(&fields, 30000).unwrap(), CASES[0].0);
}

#[test]
fn folder_configuration_records_reject_all_prefixes_trailing_bytes_and_old_context() {
    for (bytes, _, _) in CASES {
        for prefix in 0..bytes.len() {
            assert!(decode(19000, &bytes[..prefix]).is_err());
        }
        assert!(decode(19000, &[*bytes, &[0]].concat()).is_err());
        assert!(decode(0x27cb, bytes).is_err());
    }
}

fn face_pair(bytes: &[u8], names: [&str; 2]) -> Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        for name in names {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
        }
        super::configuration_atoms::face_color(reader, map)
    })
}

#[test]
fn face_color_configuration_enforces_both_pointer_roles() {
    let bytes = [1, 0, 2, 0, 0, 0, 0, 128];
    let expected = ["moCompFace_c", "moVisualProperties_c"];
    assert_eq!(
        encode_fields(&face_pair(&bytes, expected).unwrap(), 30000).unwrap(),
        bytes
    );
    for names in [
        ["moCompEdge_c", expected[1]],
        [expected[0], "moCompFace_c"],
        ["moCompFace_c_suffix", expected[1]],
    ] {
        assert!(face_pair(&bytes, names).is_err());
    }
    for prefix in 0..bytes.len() {
        assert!(face_pair(&bytes[..prefix], expected).is_err());
    }
}

#[test]
fn cut_list_sort_options_bound_counts_and_keep_completed_empty_lists() {
    let empty = [(-7_i32).to_le_bytes().as_slice(), &[0, 0]].concat();
    let fields = decode_collection(19000, &empty, super::browser_folders::cut_list_sort).unwrap();
    assert_eq!(encode_fields(&fields, 30000).unwrap(), empty);
    let impossible = [
        0_i32.to_le_bytes().as_slice(),
        &[255, 255],
        u32::MAX.to_le_bytes().as_slice(),
    ]
    .concat();
    assert!(decode_collection(19000, &impossible, super::browser_folders::cut_list_sort).is_err());
    let fields = decode(19000, CASES[0].0).unwrap();
    let missing =
        super::manager_context_tests::replace_field(&fields, "root.object.items[0]", &[3, 0]);
    assert!(decode(19000, &missing).is_err());
}
