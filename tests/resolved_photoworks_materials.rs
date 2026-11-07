//! Complete direct PhotoWorks managers through public load/save APIs (#54).
#[path = "common/configuration_manager_contract.rs"]
mod contract;
const MANAGERS: &[&[u8]] = &[
    include_bytes!("fixtures/photoworks-materials/owned-0-0.bin"),
    include_bytes!("fixtures/photoworks-materials/owned-0-1.bin"),
    include_bytes!("fixtures/photoworks-materials/owned-0-2.bin"),
    include_bytes!("fixtures/photoworks-materials/owned-0-3.bin"),
    include_bytes!("fixtures/photoworks-materials/owned-0-4.bin"),
    include_bytes!("fixtures/photoworks-materials/owned-0-5.bin"),
    include_bytes!("fixtures/photoworks-materials/owned-0-2-edited.bin"),
];
#[test]
fn photoworks_documents_load_save_and_reopen_without_changing_materials() {
    contract::check_documents(MANAGERS);
}
#[test]
fn photoworks_source_mutation_does_not_change_the_owned_result() {
    contract::check_isolation(MANAGERS);
}
#[test]
fn incomplete_photoworks_materials_fail_without_changing_the_document() {
    contract::check_incomplete(MANAGERS);
}
