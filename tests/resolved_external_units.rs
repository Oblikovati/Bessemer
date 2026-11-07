//! Saved external unit graphs through public document operations (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/external-unit-components/owned-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/external-unit-components/owned-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/external-unit-components/owned-2.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/external-unit-components/owned-3.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/external-unit-components/owned-4.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/external-unit-components/owned-5.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/external-unit-components/owned-2-edited.bin"),
        101,
    ),
];
#[test]
fn external_unit_documents_reconstruct_reopen_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, *slots);
    }
}
#[test]
fn external_unit_configuration_id_edit_matches_independent_golden() {
    contract::check_word_edit(
        CASES[2].0,
        CASES[6].0,
        "root.features[0].owner.referenced_configuration.id",
    );
}
#[test]
fn external_unit_wrong_typed_edit_preserves_inputs() {
    contract::check_rejected_edit(
        CASES[2].0,
        "root.features[0].owner.referenced_configuration.id",
    );
}
