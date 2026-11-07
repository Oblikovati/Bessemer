//! Complete C-765 graph fields and atomic character-count text against independent oracles (#165).
use bessemer::simulation::*;
#[path = "simulation_post_graph/budgets.rs"]
mod budgets;
#[path = "simulation_post_graph/concurrency.rs"]
mod concurrency;
#[path = "simulation_post_graph/contracts.rs"]
mod contracts;
#[path = "simulation_post_graph/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[allow(
    dead_code,
    reason = "Shared primitive encoders serve the other complete persistence corpora."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "simulation_post_graph/malformed.rs"]
mod malformed;
#[path = "simulation_post_graph/typed_edits.rs"]
mod typed_edits;
#[allow(
    dead_code,
    reason = "Graph has no embedded archives; other families use the nested assertions."
)]
#[path = "support/typed_load_history.rs"]
mod typed_load_history;
#[allow(
    dead_code,
    reason = "Shared value/span helpers also serve embedded corpus families."
)]
#[path = "support/typed_load_values.rs"]
mod typed_load_values;
use load_projection::*;

const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-post-graph/projection.bin");
const LIMITS: SimulationPostGraphLimits = SimulationPostGraphLimits {
    maximum_bytes: 2000000,
    maximum_fields: 100000,
    maximum_entries: 100000,
    maximum_text_bytes: 1000000,
    embedded: bessemer::model_archives::ModelArchiveLimits {
        maximum_bytes: 0,
        maximum_fields: 0,
        maximum_entries: 0,
        maximum_depth: 0,
        maximum_objects: 0,
    },
};

fn read_body(cursor: &mut &[u8]) -> SimulationPostGraphParameters {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    SimulationPostGraphParameters::from_bytes(take_projection(cursor, size), version, LIMITS)
        .unwrap()
}

fn assert_constructed(body: &SimulationPostGraphParameters) {
    let rebuilt =
        SimulationPostGraphParameters::from_fields(body.fields().to_vec(), body.version(), LIMITS)
            .unwrap();
    assert_eq!(rebuilt.to_bytes(), body.to_bytes());
    assert_eq!(rebuilt.fields(), body.fields());
    assert_eq!(rebuilt.field_offsets(), body.field_offsets());
    assert_ne!(rebuilt.field_id(0).unwrap(), body.field_id(0).unwrap());
    let coverage = body.coverage();
    assert_eq!(coverage.version, body.version());
    assert_eq!(
        coverage.completion,
        SimulationPostGraphCompletion::CompletedBody
    );
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.checked_bytes, body.to_bytes().len());
    assert_eq!(coverage.checked_fields, body.fields().len());
    assert_eq!(coverage.requested_pattern_error_bits, 0);
}

fn graph_wire(value: &SimulationPostGraphValue) -> (usize, Vec<u8>) {
    match value {
        SimulationPostGraphValue::Signed(word) => (0, word.to_le_bytes().to_vec()),
        SimulationPostGraphValue::Raw64(raw) => (4, raw.to_vec()),
        SimulationPostGraphValue::Text(text) => (6, text.to_bytes()),
    }
}

fn project_graph_field(
    output: &mut Vec<u8>,
    field: &SimulationPostGraphField,
    span: (usize, usize),
) {
    let (tag, raw) = graph_wire(&field.value);
    project_word(output, field.path.len());
    output.extend_from_slice(field.path.as_bytes());
    project_word(output, tag);
    project_word(output, span.0);
    project_word(output, span.1);
    assert_eq!(raw.len(), span.1 - span.0);
    output.extend(raw);
}

fn project_graph_fields(output: &mut Vec<u8>, body: &SimulationPostGraphParameters) {
    project_word(output, body.fields().len());
    for (field, span) in body.fields().iter().zip(body.field_offsets()) {
        project_graph_field(output, field, *span);
    }
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let body = read_body(cursor);
    assert_constructed(&body);
    project_word(output, body.version().value() as usize);
    project_word(output, body.to_bytes().len());
    output.extend(body.to_bytes());
    project_graph_fields(output, &body);
    consume_expected_fields(cursor);
}

#[test]
fn every_owned_context_matches_all_independent_typed_fields_and_construction() {
    project_packet(EXPECTED, 270, project_case);
}

#[test]
#[ignore = "requires fresh complete native graph field expectations"]
fn every_native_context_matches_complete_fields_with_optional_export() {
    let path = std::env::var_os("BESSEMER_POST_GRAPH_NATIVE_PROJECTION_INPUT")
        .expect("set full native graph projection packet");
    let mut packet = project_packet(EXPECTED, 270, project_case);
    packet.extend(project_packet(
        &std::fs::read(path).unwrap(),
        224,
        project_case,
    ));
    if let Some(path) = std::env::var_os("BESSEMER_POST_GRAPH_PROJECTION_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}
