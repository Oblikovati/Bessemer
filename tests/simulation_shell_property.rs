//! Complete shell Property operations against independent explicit-context oracles (#159).
use bessemer::{model_archives::ModelArchiveLimits, simulation::*};
use std::sync::Arc;
#[path = "simulation_shell_property/budgets.rs"]
mod budgets;
#[path = "simulation_shell_property/concurrency.rs"]
mod concurrency;
#[path = "simulation_shell_property/contracts.rs"]
mod contracts;
#[path = "simulation_shell_property/failure_preservation.rs"]
mod failure_preservation;
#[path = "simulation_shell_property/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[path = "support/binary_load_history.rs"]
mod load_history;
#[path = "support/binary_load_limits.rs"]
mod load_limits;
#[path = "support/binary_load_mutations.rs"]
mod load_mutations;
#[allow(
    dead_code,
    reason = "Shared primitive/history helpers serve other load value families."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "simulation_shell_property/malformed.rs"]
mod malformed;
#[path = "simulation_shell_property/typed_edits.rs"]
mod typed_edits;
#[path = "simulation_shell_property/typed_history.rs"]
mod typed_history;
#[path = "support/typed_load_history.rs"]
mod typed_load_history;
#[path = "support/typed_load_values.rs"]
mod typed_load_values;
#[path = "simulation_shell_property/typed_values.rs"]
mod typed_values;
use load_projection::{
    consume_expected_fields, project_field, project_packet, project_word, read_word,
    take_projection,
};

const KINDS: [SimulationShellPropertyKind; 2] = [
    SimulationShellPropertyKind::Shells,
    SimulationShellPropertyKind::MidSurfShells,
];
const LIMITS: SimulationShellPropertyLimits = SimulationShellPropertyLimits {
    maximum_bytes: 8_000_000,
    maximum_fields: 1_000_000,
    maximum_entries: 1_000_000,
    maximum_text_bytes: 1_000_000,
    embedded: ModelArchiveLimits {
        maximum_bytes: 1_000_000,
        maximum_fields: 100_000,
        maximum_entries: 100_000,
        maximum_depth: 128,
        maximum_objects: 100_000,
    },
};
const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-shell-property/projection.bin");

fn read_presence(cursor: &mut &[u8]) -> SimulationShellLayerPresence {
    let count = read_word(cursor) as i32;
    if count == -1 {
        return SimulationShellLayerPresence::Unknown;
    }
    assert!(count >= 0);
    let values = take_projection(cursor, count as usize)
        .iter()
        .map(|value| {
            assert!(*value <= 1);
            *value != 0
        })
        .collect::<Vec<bool>>();
    SimulationShellLayerPresence::Known(Arc::from(values))
}

fn project_presence(output: &mut Vec<u8>, presence: &SimulationShellLayerPresence) {
    match presence {
        SimulationShellLayerPresence::Unknown => output.extend_from_slice(&(-1_i32).to_le_bytes()),
        SimulationShellLayerPresence::Known(values) => {
            project_word(output, values.len());
            output.extend(values.iter().map(|value| u8::from(*value)));
        }
    }
}

fn read_original(cursor: &mut &[u8]) -> SimulationShellProperty {
    let layer_presence = read_presence(cursor);
    let kind = KINDS[read_word(cursor) as usize];
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let context = shell_context(kind, layer_presence);
    SimulationShellProperty::from_bytes(take_projection(cursor, size), version, context, LIMITS)
        .unwrap()
}

fn shell_context(
    kind: SimulationShellPropertyKind,
    layer_presence: SimulationShellLayerPresence,
) -> SimulationShellPropertyContext {
    SimulationShellPropertyContext {
        kind,
        layer_presence,
    }
}

fn context_kind(body: &SimulationShellProperty) -> usize {
    KINDS
        .iter()
        .position(|kind| *kind == body.context().kind)
        .unwrap()
}

fn assert_constructed(body: &SimulationShellProperty) {
    let constructed = SimulationShellProperty::from_fields(
        body.fields().to_vec(),
        body.version(),
        body.context().clone(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(constructed.to_bytes(), body.to_bytes());
    assert_eq!(constructed.fields(), body.fields());
    assert_eq!(constructed.field_offsets(), body.field_offsets());
    assert_eq!(constructed.context(), body.context());
    assert_eq!(constructed.version(), body.version());
    assert_ne!(constructed.field_id(0).unwrap(), body.field_id(0).unwrap());
}

fn assert_coverage(body: &SimulationShellProperty) {
    let coverage = body.coverage();
    assert_eq!(coverage.version, body.version());
    assert_eq!(coverage.context, *body.context());
    assert_eq!(
        coverage.completion,
        SimulationShellPropertyCompletion::CompletedBody
    );
    assert_eq!(
        coverage.check_mode,
        SimulationShellPropertyCheckMode::ExactSelectedProfiles
    );
    assert_eq!(coverage.checked_bytes, body.to_bytes().len());
    assert_eq!(coverage.checked_fields, body.fields().len());
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.diagnostics.len(), 4);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let body = read_original(cursor);
    assert_constructed(&body);
    assert_coverage(&body);
    project_presence(output, &body.context().layer_presence);
    project_word(output, context_kind(&body));
    project_word(output, body.version().value() as usize);
    let raw = body.to_bytes();
    project_word(output, raw.len());
    output.extend_from_slice(&raw);
    project_word(output, body.fields().len());
    for (field, span) in body.fields().iter().zip(body.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

#[test]
fn every_owned_context_matches_complete_independent_fields_spans_and_constructed_bytes() {
    project_packet(EXPECTED, 2284, project_case);
}

#[test]
#[ignore = "requires every original native shell Property argument and failure"]
fn all_original_native_arguments_preserve_three_failed_attempts_without_repair() {
    let path = std::env::var_os("BESSEMER_SHELL_PROPERTY_NATIVE_INPUT")
        .expect("set native shell Property arguments");
    let raw = std::fs::read(path).unwrap();
    assert_native_arguments(&raw);
}

fn assert_native_arguments(mut cursor: &[u8]) {
    let mut counts = (0, 0);
    while !cursor.is_empty() {
        let kind = KINDS[read_word(&mut cursor) as usize];
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let expected = read_word(&mut cursor) != 0;
        let size = read_word(&mut cursor) as usize;
        let context = shell_context(kind, SimulationShellLayerPresence::Unknown);
        let result = SimulationShellProperty::from_bytes(
            take_projection(&mut cursor, size),
            version,
            context,
            LIMITS,
        );
        assert_eq!(result.is_ok(), expected, "{kind:?} {version:?}");
        counts.0 += usize::from(expected);
        counts.1 += usize::from(!expected);
    }
    assert_eq!(counts, (251, 3));
}

#[test]
#[ignore = "requires complete independent native shell Property primitive expectations"]
fn all_native_accepted_arguments_match_full_fields_with_optional_export() {
    let path = std::env::var_os("BESSEMER_SHELL_PROPERTY_NATIVE_PROJECTION_INPUT")
        .expect("set native shell Property fields");
    let native = std::fs::read(path).unwrap();
    let mut output = project_packet(EXPECTED, 2284, project_case);
    output.extend(project_packet(&native, 251, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_SHELL_PROPERTY_PROJECTION_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
