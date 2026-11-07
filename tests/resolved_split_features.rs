//! Complete split features, positive kernel items and immutable native-map views (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::{CompressedKernelContext, ModelFieldValue};
use bessemer::parasolid::{BaseSchema, TransmitLimits};
use std::collections::BTreeMap;
const CASES: [&[u8]; 27] = [
    include_bytes!("fixtures/split-features/owned-0-0.bin"),
    include_bytes!("fixtures/split-features/owned-0-1.bin"),
    include_bytes!("fixtures/split-features/owned-0-2.bin"),
    include_bytes!("fixtures/split-features/owned-0-3.bin"),
    include_bytes!("fixtures/split-features/owned-0-4.bin"),
    include_bytes!("fixtures/split-features/owned-0-5.bin"),
    include_bytes!("fixtures/split-features/owned-1-0.bin"),
    include_bytes!("fixtures/split-features/owned-1-1.bin"),
    include_bytes!("fixtures/split-features/owned-1-2.bin"),
    include_bytes!("fixtures/split-features/owned-1-3.bin"),
    include_bytes!("fixtures/split-features/owned-1-4.bin"),
    include_bytes!("fixtures/split-features/owned-1-5.bin"),
    include_bytes!("fixtures/split-features/owned-2-0.bin"),
    include_bytes!("fixtures/split-features/owned-2-1.bin"),
    include_bytes!("fixtures/split-features/owned-2-2.bin"),
    include_bytes!("fixtures/split-features/owned-2-3.bin"),
    include_bytes!("fixtures/split-features/owned-2-4.bin"),
    include_bytes!("fixtures/split-features/owned-2-5.bin"),
    include_bytes!("fixtures/split-features/owned-3-0.bin"),
    include_bytes!("fixtures/split-features/owned-3-1.bin"),
    include_bytes!("fixtures/split-features/owned-3-2.bin"),
    include_bytes!("fixtures/split-features/owned-3-3.bin"),
    include_bytes!("fixtures/split-features/owned-3-4.bin"),
    include_bytes!("fixtures/split-features/owned-3-5.bin"),
    include_bytes!("fixtures/split-features/owned-0-2-edited.bin"),
    include_bytes!("fixtures/split-features/owned-1-2-edited.bin"),
    include_bytes!("fixtures/split-features/owned-0-4-edited.bin"),
];
const SLOTS: [usize; 27] = [
    101, 109, 121, 117, 113, 127, 99, 99, 107, 101, 101, 107, 99, 99, 103, 99, 99, 103, 99, 99,
    103, 99, 99, 103, 121, 107, 113,
];

fn kernel() -> CompressedKernelContext {
    CompressedKernelContext {
        base: BaseSchema::from_bytes(include_bytes!("fixtures/parasolid/base-schema.s_t")).unwrap(),
        maximum_expanded: 1024 * 1024,
        transmit_limits: TransmitLimits {
            maximum_records: 1000,
            maximum_field_elements: 10000,
        },
    }
}

#[test]
fn complete_split_documents_reconstruct_all_storage_with_positive_kernel_items() {
    for (index, (bytes, slots)) in CASES.into_iter().zip(SLOTS).enumerate() {
        if (12..24).contains(&index) {
            contract::check_document(bytes, slots);
        } else {
            contract::check_document_with_kernel(bytes, slots, Some(kernel()));
        }
    }
}

#[test]
fn isolated_parent_and_fixed_word_edits_preserve_other_features_and_inputs() {
    contract::check_word_edit_with_kernel(
        CASES[2],
        CASES[24],
        "root.features[0].items[0].payload.raw_910",
        Some(kernel()),
    );
    contract::check_value_edit(
        CASES[8],
        CASES[25],
        "root.features[0].items[0].transform_20[0]",
        ModelFieldValue::U32(12345),
        Some(kernel()),
    );
    contract::check_word_edit(CASES[4], CASES[26], "root.features[0].payload.raw_910");
}

#[test]
fn rejected_parent_word_types_preserve_complete_documents_with_and_without_kernel_bodies() {
    contract::check_rejected_edit_with_kernel(
        CASES[2],
        "root.features[0].items[0].payload.raw_910",
        Some(kernel()),
    );
    contract::check_rejected_edit(CASES[4], "root.features[0].payload.raw_910");
}

#[test]
fn derived_maps_remain_owned_and_do_not_change_stored_pairs_or_published_archives() {
    let mut document = contract::document_with(CASES[4]);
    let archive = contract::parsed_with_kernel(&document, None);
    let maps = archive.split_integer_maps().unwrap();
    assert_eq!(maps.len(), 1);
    assert_eq!(maps[0].raw_count(), 5);
    assert_eq!(
        maps[0].raw_pairs(),
        &[(7, 8), (7, 9), (0, i32::MIN), (-1, -1), (0, 17)]
    );
    assert_eq!(maps[0].loaded(), &BTreeMap::from([(0, 17), (7, 9)]));
    assert_eq!(maps[0].path(), "root.features[0].payload.map_9a8");
    document
        .replace_stream("Contents/Config-0-ResolvedFeatures", vec![255])
        .unwrap();
    assert_eq!(archive.to_bytes().unwrap(), CASES[4]);
    assert_eq!(archive.split_integer_maps().unwrap(), maps);
    let no_maps = contract::parsed_with_kernel(&contract::document_with(CASES[12]), None);
    assert!(no_maps.split_integer_maps().unwrap().is_empty());
}
