//! Complete manager scalar maps remain usable through public model loading and saving (#54).
#[path = "common/configuration_manager_contract.rs"]
mod contract;
const MANAGERS: &[&[u8]] = &[
    include_bytes!("fixtures/manager-scalars/owned-0-0.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-1.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-2.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-3.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-4.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-5.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-0.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-1.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-2.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-3.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-4.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-5.bin"),
    include_bytes!("fixtures/manager-scalars/owned-0-2-edited.bin"),
    include_bytes!("fixtures/manager-scalars/owned-1-2-edited.bin"),
];
#[test]
fn manager_scalar_documents_load_save_and_reopen_without_changing_maps() {
    contract::check_documents(MANAGERS);
}
#[test]
fn manager_mutation_after_public_load_does_not_change_the_owned_result() {
    contract::check_isolation(MANAGERS);
}
#[test]
fn incomplete_manager_scalar_maps_fail_without_changing_the_document() {
    contract::check_incomplete(MANAGERS);
}
