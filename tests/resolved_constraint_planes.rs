//! Complete constraint-plane families and immutable native flag interpretation (#54, #46).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::{ModelFieldValue, StoredDouble};
const CASES: [&[u8]; 126] = [
    include_bytes!("fixtures/constraint-plane-families/owned-0-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-0-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-0-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-0-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-0-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-0-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-1-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-1-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-1-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-1-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-1-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-1-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-2-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-2-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-2-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-2-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-2-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-2-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-3-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-3-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-3-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-3-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-3-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-3-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-4-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-4-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-4-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-4-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-4-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-4-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-5-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-5-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-5-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-5-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-5-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-5-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-6-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-6-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-6-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-6-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-6-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-6-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-7-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-7-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-7-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-7-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-7-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-7-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-8-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-8-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-8-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-8-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-8-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-8-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-9-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-9-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-9-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-9-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-9-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-9-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-10-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-10-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-10-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-10-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-10-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-10-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-11-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-11-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-11-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-11-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-11-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-11-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-12-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-12-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-12-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-12-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-12-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-12-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-13-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-13-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-13-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-13-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-13-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-13-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-14-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-14-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-14-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-14-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-14-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-14-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-15-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-15-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-15-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-15-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-15-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-15-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-16-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-16-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-16-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-16-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-16-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-16-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-17-0.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-17-1.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-17-2.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-17-3.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-17-4.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-17-5.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-0-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-1-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-2-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-3-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-4-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-5-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-6-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-7-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-8-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-9-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-10-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-11-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-12-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-13-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-14-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-15-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-16-2-edited.bin"),
    include_bytes!("fixtures/constraint-plane-families/owned-17-2-edited.bin"),
];
const SLOTS: [usize; 126] = [
    103, 103, 107, 103, 107, 115, 99, 105, 109, 107, 109, 119, 99, 105, 109, 107, 109, 119, 99,
    107, 111, 109, 111, 121, 99, 105, 109, 107, 109, 119, 99, 105, 109, 107, 109, 119, 99, 105,
    109, 107, 109, 119, 99, 105, 109, 107, 109, 119, 99, 105, 109, 107, 109, 119, 99, 103, 107,
    105, 103, 109, 99, 101, 105, 101, 101, 105, 99, 101, 105, 101, 101, 105, 99, 101, 105, 101,
    101, 105, 99, 101, 105, 101, 101, 105, 99, 101, 105, 101, 101, 105, 99, 101, 105, 101, 101,
    105, 99, 101, 105, 101, 101, 105, 101, 101, 107, 101, 101, 107, 107, 109, 109, 111, 109, 109,
    109, 109, 109, 107, 105, 105, 105, 105, 105, 105, 105, 107,
];
const PATHS: [&str; 18] = [
    "root.features[0].items[0].raw_9c",
    "root.features[0].items[0].word_50",
    "root.features[0].items[0].word_50",
    "root.features[0].items[0].raw_148",
    "root.features[0].items[0].word_50",
    "root.features[0].items[0].word_50",
    "root.features[0].items[0].word_50",
    "root.features[0].items[0].raw_e8",
    "root.features[0].items[0].raw_e8",
    "root.features[0].items[0].geometry_08.raw_plane_geometry_flag_38",
    "root.features[0].items[0].surface_u_10",
    "root.features[0].items[0].surface_u_10",
    "root.features[0].items[0].surface_u_10",
    "root.features[0].items[0].raw_plane_geometry_flag_38",
    "root.features[0].items[0].raw_plane_geometry_flag_38",
    "root.features[0].items[0].raw_plane_geometry_flag_38",
    "root.features[0].items[0].raw_plane_geometry_flag_40",
    "root.features[0].items[0].reference.unknown_64",
];

#[test]
fn all_owned_planes_reconstruct_complete_storage_without_mutation() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn independent_plane_edits_preserve_shared_targets_other_roots_and_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        let value = match kind {
            9 | 13..=16 => ModelFieldValue::U8(1),
            10..=12 => ModelFieldValue::F64(StoredDouble::from_bits(0x4045000000000000)),
            _ => {
                contract::check_word_edit(CASES[kind * 6 + 2], CASES[108 + kind], path);
                continue;
            }
        };
        contract::check_value_edit(CASES[kind * 6 + 2], CASES[108 + kind], path, value, None);
    }
}

#[test]
fn rejected_plane_scalar_types_preserve_complete_archives_and_documents() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(CASES[kind * 6 + 2], path);
    }
}

#[test]
fn published_plane_flags_keep_raw_bytes_native_semantics_and_owned_paths() {
    let original = contract::document_with(CASES[80]);
    let archive = contract::parsed_with_kernel(&original, None);
    let flags = archive.plane_geometry_flags().unwrap();
    assert_eq!(flags.len(), 2);
    assert!(flags.iter().all(|flag| flag.raw() == 2 && !flag.loaded()));
    assert!(flags[0].path().ends_with(".raw_plane_geometry_flag_38"));
    let changed = contract::document_with(CASES[121]);
    let edited = contract::parsed_with_kernel(&changed, None)
        .plane_geometry_flags()
        .unwrap();
    assert_eq!(
        (edited[0].raw(), edited[0].loaded(), edited[1].raw()),
        (1, true, 2)
    );
    assert!(flags.iter().all(|flag| flag.raw() == 2));
}

#[test]
fn completed_geometry_without_flags_returns_an_empty_flag_collection() {
    let empty = contract::document_with(CASES[60]);
    assert!(
        contract::parsed_with_kernel(&empty, None)
            .plane_geometry_flags()
            .unwrap()
            .is_empty()
    );
}
