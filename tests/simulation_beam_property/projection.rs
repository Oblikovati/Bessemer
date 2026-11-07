use super::load_projection::{
    consume_expected_fields, project_field, project_outer_history, project_packet, project_word,
};
use super::*;

const FIELDS: &[u8] = include_bytes!("../fixtures/simulation-beam-property/projection.bin");
const HISTORY: &[u8] = include_bytes!("../fixtures/simulation-beam-property/history.bin");

fn project_beam_field(output: &mut Vec<u8>, field: &SimulationBeamField, span: (usize, usize)) {
    match &field.value {
        SimulationBeamValue::Shared(value) => project_field(
            output,
            &SimulationThermalField {
                path: field.path.clone(),
                value: value.clone(),
            },
            span,
        ),
        SimulationBeamValue::SignedShort(value) => {
            project_word(output, field.path.len());
            output.extend_from_slice(field.path.as_bytes());
            project_word(output, 6);
            project_word(output, span.0);
            project_word(output, span.1);
            assert_eq!(span.1 - span.0, 2);
            output.extend_from_slice(&value.to_le_bytes());
        }
    }
}

fn project_all_fields(output: &mut Vec<u8>, property: &SimulationBeamProperty) {
    project_word(output, property.fields().len());
    for (field, span) in property.fields().iter().zip(property.field_offsets()) {
        project_beam_field(output, field, *span);
    }
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let version = word(cursor);
    let size = word(cursor) as usize;
    let raw = take(cursor, size);
    let property = SimulationBeamProperty::from_bytes(
        raw,
        SimulationPersistVersion::new(version as i32).unwrap(),
        LIMITS,
    )
    .unwrap();
    project_word(output, version as usize);
    project_word(output, size);
    output.extend_from_slice(&property.to_bytes());
    project_all_fields(output, &property);
    consume_expected_fields(cursor);
}

fn consume_history(cursor: &mut &[u8]) {
    for _ in 0..word(cursor) {
        word(cursor);
        word(cursor);
        let parents = word(cursor) as usize;
        take(cursor, parents * 8);
    }
}

fn short_edit(property: &SimulationBeamProperty, replacement: i32) -> SimulationBeamEdit {
    let index = ordinal(property, "record0.beam.shorte2");
    property
        .edit_field(
            &property.field_id(index).unwrap(),
            SimulationBeamValue::SignedShort(i16::try_from(replacement).unwrap()),
        )
        .unwrap()
}

fn project_edit(output: &mut Vec<u8>, edit: &SimulationBeamEdit) {
    let SimulationBeamValue::SignedShort(replacement) = edit.replacement() else {
        panic!("expected SignedShort");
    };
    project_word(output, edit.archive().version().value() as usize);
    output.extend_from_slice(&i32::from(*replacement).to_le_bytes());
    project_word(output, edit.source().to_bytes().len());
    output.extend_from_slice(&edit.source().to_bytes());
    output.extend_from_slice(&edit.archive().to_bytes());
    project_all_fields(output, edit.archive());
}

fn project_edit_parents(output: &mut Vec<u8>, edit: &SimulationBeamEdit) {
    let source_ids: Vec<SimulationBeamFieldId> = (0..edit.source().fields().len())
        .map(|i| edit.source().field_id(i).unwrap())
        .collect();
    let result_ids: Vec<SimulationBeamFieldId> = (0..edit.archive().fields().len())
        .map(|i| edit.archive().field_id(i).unwrap())
        .collect();
    project_outer_history(output, edit.history(), &source_ids, &result_ids);
}

pub(super) fn read_project_history(cursor: &mut &[u8], output: &mut Vec<u8>) -> SimulationBeamEdit {
    let version = word(cursor) as i32;
    let replacement = word(cursor) as i32;
    let size = word(cursor) as usize;
    let original = take(cursor, size);
    let expected = take(cursor, size);
    let property = read(&BeamCase {
        version,
        raw: original,
        spans: Vec::new(),
    });
    let edit = short_edit(&property, replacement);
    assert_eq!(edit.archive().to_bytes(), expected);
    project_edit(output, &edit);
    consume_expected_fields(cursor);
    project_edit_parents(output, &edit);
    consume_history(cursor);
    edit
}

fn project_history_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    read_project_history(cursor, output);
}

pub(super) fn export_packet(
    owned: &[u8],
    native_env: &str,
    output_env: &str,
    project: fn(&mut &[u8], &mut Vec<u8>),
) {
    let path =
        std::env::var_os(native_env).expect("set the explicit complete native expectation packet");
    let native = std::fs::read(path).unwrap();
    let mut packet = project_packet(owned, 104, project);
    packet.extend(project_packet(&native, 163, project));
    if let Some(path) = std::env::var_os(output_env) {
        std::fs::write(path, packet).unwrap();
    }
}

#[test]
fn every_owned_field_matches_independent_complete_typed_projection() {
    project_packet(FIELDS, 104, project_case);
}

#[test]
fn every_owned_short_edit_and_outer_parent_matches_independent_source() {
    project_packet(HISTORY, 104, project_history_case);
}

#[test]
#[ignore = "requires explicit complete native typed-field expectations"]
fn all_native_fields_match_complete_independent_projection_with_optional_export() {
    export_packet(
        FIELDS,
        "BESSEMER_BEAM_NATIVE_PROJECTION_INPUT",
        "BESSEMER_BEAM_PROJECTION_OUTPUT",
        project_case,
    );
}

#[test]
#[ignore = "requires explicit complete native edited-field/history expectations"]
fn all_native_short_edits_match_complete_history_with_optional_export() {
    export_packet(
        HISTORY,
        "BESSEMER_BEAM_NATIVE_HISTORY_INPUT",
        "BESSEMER_BEAM_HISTORY_OUTPUT",
        project_history_case,
    );
}
