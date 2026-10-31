//! Complete counted MeshControl fields against independent owned/native operations (#161).
use bessemer::{model_archives::ModelArchiveLimits, simulation::*};
#[path = "simulation_mesh_control_records/budgets.rs"]
mod budgets;
#[path = "simulation_mesh_control_records/contracts.rs"]
mod contracts;
#[path = "simulation_mesh_control_records/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
#[allow(
    dead_code,
    reason = "The shared frame retains a context value; this codec has one fixed reader grammar."
)]
mod load_edit_case;
#[path = "support/binary_load_history.rs"]
mod load_history;
#[path = "support/binary_load_limits.rs"]
#[allow(
    dead_code,
    unused_imports,
    reason = "The shared limits module also serves malformed-grammar corpora."
)]
mod load_limits;
#[path = "support/binary_load_mutations.rs"]
#[allow(
    dead_code,
    reason = "Shared mutation helpers are exercised by the persistence grammar corpora."
)]
mod load_mutations;
#[path = "support/binary_load_projection.rs"]
#[allow(
    dead_code,
    reason = "Shared helpers also serve full ordered-history tests."
)]
mod load_projection;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-mesh-control/records-projection.bin");
const LIMITS: SimulationMeshControlLimits = SimulationMeshControlLimits {
    maximum_bytes: 8_000_000,
    maximum_fields: 1_000_000,
    maximum_entries: 2_000_000,
    maximum_text_bytes: 1_000_000,
    embedded: ModelArchiveLimits {
        maximum_bytes: 8_000_000,
        maximum_fields: 1_000_000,
        maximum_entries: 2_000_000,
        maximum_depth: 256,
        maximum_objects: 100_000,
    },
};

fn read_collection(cursor: &mut &[u8]) -> SimulationMeshControlRecords {
    assert_eq!(read_word(cursor), 0);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationMeshControlRecords::from_bytes(take_projection(cursor, size), version, LIMITS)
        .unwrap()
}

fn assert_constructed(collection: &SimulationMeshControlRecords) {
    let rebuilt = SimulationMeshControlRecords::from_fields(
        collection.fields().to_vec(),
        collection.version(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes(), collection.to_bytes());
    assert_eq!(rebuilt.fields(), collection.fields());
    assert_eq!(rebuilt.field_offsets(), collection.field_offsets());
    assert_eq!(rebuilt.record_spans(), collection.record_spans());
    assert_eq!(rebuilt.version(), collection.version());
    assert_ne!(
        rebuilt.field_id(0).unwrap(),
        collection.field_id(0).unwrap()
    );
}

fn project_collection(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let collection = read_collection(cursor);
    assert_constructed(&collection);
    project_word(output, 0);
    project_word(output, collection.version().value() as usize);
    let raw = collection.to_bytes();
    project_word(output, raw.len());
    output.extend_from_slice(&raw);
    project_word(output, collection.fields().len());
    for (field, span) in collection.fields().iter().zip(collection.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

#[test]
fn all_owned_records_match_complete_source_fields_offsets_and_typed_construction() {
    project_packet(OWNED, 322, project_collection);
}

#[test]
fn every_public_record_result_and_provenance_type_is_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationMeshControlRecords>();
    check::<SimulationMeshControlEdit>();
    check::<SimulationMeshControlCoverage>();
    check::<SimulationMeshControlRecordSpan>();
    check::<SimulationMeshControlLimits>();
    check::<SimulationMeshControlCompletion>();
    check::<SimulationMeshControlCheckMode>();
    check::<SimulationMeshControlDiagnostic>();
    check::<SimulationMeshControlField>();
    check::<SimulationMeshControlFieldId>();
    check::<SimulationMeshControlValue>();
    check::<SimulationMeshControlHistory>();
    check::<SimulationMeshControlHistoryParent>();
    check::<SimulationMeshControlRelation>();
    check::<SimulationMeshControlNestedHistory>();
    check::<SimulationMeshControlNestedParent>();
}

#[test]
#[ignore = "requires complete independently projected native MeshControl record fields"]
fn all_native_records_match_source_fields_offsets_and_construction_with_optional_export() {
    let path = std::env::var_os("BESSEMER_MESH_RECORDS_NATIVE_PROJECTION_INPUT")
        .expect("set native record projection input");
    let native = std::fs::read(path).unwrap();
    let mut output = project_packet(OWNED, 322, project_collection);
    output.extend(project_packet(&native, 12, project_collection));
    if let Some(path) = std::env::var_os("BESSEMER_MESH_RECORDS_PROJECTION_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
