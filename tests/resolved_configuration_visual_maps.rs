//! Direct configuration visual maps survive public loading and saving (#54).
#[path = "common/configuration_manager_contract.rs"]
mod contract;
const MANAGERS: &[&[u8]] = &[
    include_bytes!("fixtures/configuration-visual-maps/owned-0-0.bin"),
    include_bytes!("fixtures/configuration-visual-maps/owned-0-1.bin"),
    include_bytes!("fixtures/configuration-visual-maps/owned-0-2.bin"),
    include_bytes!("fixtures/configuration-visual-maps/owned-0-3.bin"),
    include_bytes!("fixtures/configuration-visual-maps/owned-0-4.bin"),
    include_bytes!("fixtures/configuration-visual-maps/owned-0-5.bin"),
    include_bytes!("fixtures/configuration-visual-maps/owned-0-2-edited.bin"),
];
#[test]
fn configuration_visual_documents_load_save_and_reopen_without_changing_maps() {
    contract::check_documents(MANAGERS);
}
#[test]
fn configuration_visual_source_mutation_does_not_change_the_owned_result() {
    contract::check_isolation(MANAGERS);
}
#[test]
fn incomplete_configuration_visual_maps_fail_without_changing_the_document() {
    contract::check_incomplete(MANAGERS);
}
