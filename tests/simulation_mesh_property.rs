//! Complete Mesh Property operations against independent owned/native packets (#163).
use bessemer::{model_archives::ModelArchiveLimits, simulation::*};
#[path = "simulation_mesh_property/budgets.rs"]
mod budgets;
#[path = "simulation_mesh_property/contracts.rs"]
mod contracts;
#[path = "simulation_mesh_property/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
#[allow(
    dead_code,
    reason = "The shared frame retains a kind; Mesh Property has one fixed grammar."
)]
mod load_edit_case;
#[path = "support/binary_load_history.rs"]
mod load_history;
#[path = "support/binary_load_limits.rs"]
#[allow(
    dead_code,
    unused_imports,
    reason = "Shared limits helpers also serve malformed grammar and mutation suites."
)]
mod load_limits;
#[path = "support/binary_load_mutations.rs"]
#[allow(
    dead_code,
    reason = "Shared mutation helpers serve the limits module before grammar corpus extensions."
)]
mod load_mutations;
#[path = "support/binary_load_projection.rs"]
#[allow(
    dead_code,
    reason = "Shared exporters also serve malformed and nested-history corpora."
)]
mod load_projection;
#[path = "simulation_mesh_property/records.rs"]
mod records;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-mesh-property/projection.bin");
const LIMITS: SimulationMeshPropertyLimits = SimulationMeshPropertyLimits {
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

fn read_property(cursor: &mut &[u8]) -> SimulationMeshProperty {
    assert_eq!(read_word(cursor), 0);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationMeshProperty::from_bytes(take_projection(cursor, size), version, LIMITS).unwrap()
}

fn assert_constructed(property: &SimulationMeshProperty) {
    let rebuilt =
        SimulationMeshProperty::from_fields(property.fields().to_vec(), property.version(), LIMITS)
            .unwrap();
    assert_eq!(rebuilt.to_bytes(), property.to_bytes());
    assert_eq!(rebuilt.fields(), property.fields());
    assert_eq!(rebuilt.field_offsets(), property.field_offsets());
    assert_eq!(rebuilt.record_spans(), property.record_spans());
    assert_eq!(rebuilt.version(), property.version());
    assert_ne!(rebuilt.field_id(0).unwrap(), property.field_id(0).unwrap());
    assert_eq!(
        rebuilt.coverage().completion,
        SimulationMeshPropertyCompletion::CompletedBody
    );
}

fn project_property(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let property = read_property(cursor);
    assert_constructed(&property);
    project_word(output, 0);
    project_word(output, property.version().value() as usize);
    let raw = property.to_bytes();
    project_word(output, raw.len());
    output.extend_from_slice(&raw);
    project_word(output, property.fields().len());
    for (field, span) in property.fields().iter().zip(property.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

#[test]
fn every_owned_property_matches_complete_fields_offsets_and_typed_construction() {
    project_packet(OWNED, 588, project_property);
}

#[test]
fn all_published_property_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationMeshProperty>();
    check::<SimulationMeshPropertyEdit>();
    check::<SimulationMeshPropertyCoverage>();
    check::<SimulationMeshPropertyRecordSpan>();
    check::<SimulationMeshPropertyLimits>();
    check::<SimulationMeshPropertyCompletion>();
    check::<SimulationMeshPropertyCheckMode>();
    check::<SimulationMeshPropertyDiagnostic>();
    check::<SimulationMeshPropertyField>();
    check::<SimulationMeshPropertyFieldId>();
    check::<SimulationMeshPropertyValue>();
    check::<SimulationMeshPropertyHistory>();
    check::<SimulationMeshPropertyHistoryParent>();
    check::<SimulationMeshPropertyRelation>();
    check::<SimulationMeshPropertyNestedHistory>();
    check::<SimulationMeshPropertyNestedParent>();
}

#[test]
#[ignore = "requires independently recovered complete native Mesh Property field expectations"]
fn every_native_property_matches_complete_fields_with_optional_export() {
    let native =
        std::fs::read(std::env::var_os("BESSEMER_MESH_PROPERTY_NATIVE_PROJECTION_INPUT").unwrap())
            .unwrap();
    let mut output = project_packet(OWNED, 588, project_property);
    output.extend(project_packet(&native, 357, project_property));
    if let Some(path) = std::env::var_os("BESSEMER_MESH_PROPERTY_PROJECTION_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}

#[test]
#[ignore = "requires all original native Mesh Property candidate arguments, including failures"]
fn every_original_native_candidate_retains_its_success_or_failure() {
    let native =
        std::fs::read(std::env::var_os("BESSEMER_MESH_PROPERTY_NATIVE_INPUT").unwrap()).unwrap();
    let mut cursor = native.as_slice();
    let (mut accepted, mut failed) = (0, 0);
    while !cursor.is_empty() {
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let expected = read_word(&mut cursor) != 0;
        let size = read_word(&mut cursor) as usize;
        let parsed =
            SimulationMeshProperty::from_bytes(take_projection(&mut cursor, size), version, LIMITS);
        assert_eq!(parsed.is_ok(), expected, "{version:?}");
        if expected {
            accepted += 1;
        } else {
            failed += 1;
        }
    }
    assert_eq!((accepted, failed), (357, 34));
}
