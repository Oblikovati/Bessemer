//! Complete Results root/category/plot field and edit-history acceptance (#164).
use bessemer::simulation::*;
#[path = "simulation_results_property/budgets.rs"]
mod budgets;
#[path = "simulation_results_property/concurrency.rs"]
mod concurrency;
#[path = "simulation_results_property/contracts.rs"]
mod contracts;
#[path = "simulation_results_property/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[allow(
    dead_code,
    reason = "Shared complete-history and primitive encoders serve other corpus suites."
)]
#[path = "support/binary_load_history.rs"]
mod load_history;
#[allow(
    dead_code,
    reason = "Shared primitive encoders serve other corpus suites."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "simulation_results_property/malformed.rs"]
mod malformed;
#[path = "simulation_results_property/typed_edits.rs"]
mod typed_edits;
#[allow(
    dead_code,
    reason = "Results has no nested archives; other families use the shared nested assertions."
)]
#[path = "support/typed_load_history.rs"]
mod typed_load_history;
#[allow(
    dead_code,
    reason = "Shared value encoders also support embedded and raw-byte corpus families."
)]
#[path = "support/typed_load_values.rs"]
mod typed_load_values;
use load_projection::*;

const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-results-headers/projection.bin");
const LIMITS: SimulationResultsLimits = SimulationResultsLimits {
    maximum_bytes: 2000000,
    maximum_fields: 200000,
    maximum_entries: 200000,
    maximum_text_bytes: 1048576,
    embedded: bessemer::model_archives::ModelArchiveLimits {
        maximum_bytes: 0,
        maximum_fields: 0,
        maximum_entries: 0,
        maximum_depth: 0,
        maximum_objects: 0,
    },
};

fn context(level: usize, version: SimulationPersistVersion) -> SimulationResultsContext {
    match level {
        0 => SimulationResultsContext::RootReader(version),
        1 => SimulationResultsContext::CategoryReader(version),
        2 => SimulationResultsContext::PlotReader(version),
        _ => panic!("level {level}; expected Results hierarchy 0..=2"),
    }
}

fn read_body(cursor: &mut &[u8]) -> (usize, SimulationPersistVersion, SimulationResultsProperty) {
    let level = read_word(cursor) as usize;
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let body = SimulationResultsProperty::from_bytes(
        take_projection(cursor, size),
        context(level, version),
        LIMITS,
    )
    .unwrap();
    (level, version, body)
}

fn assert_constructed(body: &SimulationResultsProperty) {
    let rebuilt =
        SimulationResultsProperty::from_fields(body.fields().to_vec(), body.context(), LIMITS)
            .unwrap();
    assert_eq!(rebuilt.to_bytes(), body.to_bytes());
    assert_eq!(rebuilt.fields(), body.fields());
    assert_eq!(rebuilt.field_offsets(), body.field_offsets());
    assert_ne!(rebuilt.field_id(0).unwrap(), body.field_id(0).unwrap());
    assert_coverage(body);
}

fn assert_coverage(body: &SimulationResultsProperty) {
    let coverage = body.coverage();
    assert_eq!(coverage.context, body.context());
    assert_eq!(
        coverage.completion,
        SimulationResultsCompletion::CompletedBody
    );
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.checked_bytes, body.to_bytes().len());
    assert_eq!(coverage.checked_fields, body.fields().len());
    assert_eq!(
        (
            coverage.embedded_archives,
            coverage.embedded_fields,
            coverage.requested_pattern_error_bits
        ),
        (0, 0, 0)
    );
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let (level, version, body) = read_body(cursor);
    assert_constructed(&body);
    project_word(output, level);
    project_word(output, version.value() as usize);
    project_word(output, body.to_bytes().len());
    output.extend(body.to_bytes());
    project_word(output, body.fields().len());
    for (field, span) in body.fields().iter().zip(body.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

#[test]
fn every_owned_context_matches_complete_primitive_fields_and_construction() {
    project_packet(EXPECTED, 660, project_case);
}

#[test]
#[ignore = "requires fresh complete native Results primitive expectations"]
fn every_native_context_matches_complete_fields_with_optional_export() {
    let path = std::env::var_os("BESSEMER_RESULTS_PROPERTY_NATIVE_PROJECTION_INPUT")
        .expect("set full native Results projection packet");
    let mut packet = project_packet(EXPECTED, 660, project_case);
    packet.extend(project_packet(
        &std::fs::read(path).unwrap(),
        450,
        project_case,
    ));
    if let Some(path) = std::env::var_os("BESSEMER_RESULTS_PROPERTY_PROJECTION_OUTPUT") {
        std::fs::write(path, packet).unwrap();
    }
}
