//! Complete concentric mates and owned section geometry through public archive operations (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::{CompressedKernelContext, ModelFieldValue};
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/mate-planes/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/mate-planes/owned-0-1.bin"), 109),
    (include_bytes!("fixtures/mate-planes/owned-0-2.bin"), 109),
    (include_bytes!("fixtures/mate-planes/owned-0-3.bin"), 109),
    (include_bytes!("fixtures/mate-planes/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/mate-planes/owned-0-5.bin"), 109),
    (include_bytes!("fixtures/mate-planes/owned-1-0.bin"), 101),
    (include_bytes!("fixtures/mate-planes/owned-1-1.bin"), 111),
    (include_bytes!("fixtures/mate-planes/owned-1-2.bin"), 111),
    (include_bytes!("fixtures/mate-planes/owned-1-3.bin"), 115),
    (include_bytes!("fixtures/mate-planes/owned-1-4.bin"), 101),
    (include_bytes!("fixtures/mate-planes/owned-1-5.bin"), 115),
    (include_bytes!("fixtures/mate-planes/owned-2-0.bin"), 99),
    (include_bytes!("fixtures/mate-planes/owned-2-1.bin"), 101),
    (include_bytes!("fixtures/mate-planes/owned-2-2.bin"), 101),
    (include_bytes!("fixtures/mate-planes/owned-2-3.bin"), 101),
    (include_bytes!("fixtures/mate-planes/owned-2-4.bin"), 99),
    (include_bytes!("fixtures/mate-planes/owned-2-5.bin"), 101),
    (include_bytes!("fixtures/mate-planes/owned-3-0.bin"), 99),
    (include_bytes!("fixtures/mate-planes/owned-3-1.bin"), 99),
    (include_bytes!("fixtures/mate-planes/owned-3-2.bin"), 99),
    (include_bytes!("fixtures/mate-planes/owned-3-3.bin"), 99),
    (include_bytes!("fixtures/mate-planes/owned-3-4.bin"), 99),
    (include_bytes!("fixtures/mate-planes/owned-3-5.bin"), 99),
    (
        include_bytes!("fixtures/mate-planes/owned-0-2-edited.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/mate-planes/owned-1-2-edited.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/mate-planes/owned-2-2-edited.bin"),
        101,
    ),
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
fn complete_mate_plane_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn independent_mate_and_section_word_edits_match_owned_goldens() {
    contract::check_word_edit_with_kernel(
        CASES[2].0,
        CASES[24].0,
        "root.features[0].mate.raw_388",
        Some(kernel()),
    );
    contract::check_word_edit_with_kernel(
        CASES[8].0,
        CASES[25].0,
        "root.features[0].raw_710",
        Some(kernel()),
    );
}
#[test]
fn removing_one_complete_section_edge_body_matches_independent_bytes() {
    let source_document = contract::document_with(CASES[14].0);
    let source = contract::parsed_with_kernel(&source_document, Some(kernel()));
    let mut fields = source.fields().to_vec();
    fields.retain(|f| !f.path.starts_with("root.features[0].bodies[1]."));
    fields
        .iter_mut()
        .find(|f| f.path.as_ref() == "root.features[0].bodies.count")
        .unwrap()
        .value = ModelFieldValue::I32(1);
    let changed = source.edit_fields(fields).unwrap();
    assert_eq!(changed.archive().to_bytes().unwrap(), CASES[26].0);
    assert_eq!(source.to_bytes().unwrap(), CASES[14].0);
    assert_eq!(
        source_document
            .read_stream("Contents/Config-0-ResolvedFeatures")
            .unwrap(),
        CASES[14].0
    );
}
#[test]
fn rejected_mate_plane_edits_preserve_the_original_fields_and_document() {
    contract::check_rejected_edit(CASES[0].0, "root.features[0].mate.raw_388");
    contract::check_rejected_edit_with_kernel(
        CASES[8].0,
        "root.features[0].raw_710",
        Some(kernel()),
    );
}
