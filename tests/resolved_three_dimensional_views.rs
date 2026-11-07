//! Complete 3D-view managers through the public model dependency contract (#54).
#[path = "common/configuration_manager_contract.rs"]
mod contract;
const MANAGERS: &[&[u8]] = &[
    include_bytes!("fixtures/three-dimensional-views/owned-0-0.bin"),
    include_bytes!("fixtures/three-dimensional-views/owned-0-1.bin"),
    include_bytes!("fixtures/three-dimensional-views/owned-0-2.bin"),
    include_bytes!("fixtures/three-dimensional-views/owned-0-3.bin"),
    include_bytes!("fixtures/three-dimensional-views/owned-0-4.bin"),
    include_bytes!("fixtures/three-dimensional-views/owned-0-5.bin"),
    include_bytes!("fixtures/three-dimensional-views/owned-0-2-edited.bin"),
];
#[test]
fn three_dimensional_view_managers_load_save_and_reopen() {
    contract::check_documents(MANAGERS);
}
#[test]
fn three_dimensional_manager_source_mutation_keeps_loaded_results_isolated() {
    contract::check_isolation(MANAGERS);
}
#[test]
fn incomplete_three_dimensional_view_managers_preserve_input_documents() {
    contract::check_incomplete(MANAGERS);
}
