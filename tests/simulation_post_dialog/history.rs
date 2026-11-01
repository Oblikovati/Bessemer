use super::*;
use load_history::{LoadHistorySnapshot, LoadHistoryView};

fn wire_fields(body: &SimulationPostDialog) -> Vec<SimulationThermalField> {
    body.fields()
        .iter()
        .map(|field| SimulationThermalField {
            path: field.path.clone(),
            value: wire_value(&field.value),
        })
        .collect()
}

fn wire_value(value: &SimulationPostDialogValue) -> SimulationThermalValue {
    match value {
        SimulationPostDialogValue::Signed(word) => SimulationThermalValue::Signed(*word),
        SimulationPostDialogValue::RawWordBits(bits)
        | SimulationPostDialogValue::Float32Bits(bits) => SimulationThermalValue::Unsigned(*bits),
        SimulationPostDialogValue::PersistText(text) => SimulationThermalValue::Text(text.clone()),
        SimulationPostDialogValue::Embedded(archive) => {
            SimulationThermalValue::Embedded(archive.clone())
        }
        value => SimulationThermalValue::Bytes(encoded_value(value).1.into()),
    }
}

fn snapshot<'a>(
    body: &'a SimulationPostDialog,
    fields: &'a [SimulationThermalField],
) -> LoadHistorySnapshot<'a> {
    LoadHistorySnapshot::new(body.to_bytes(), fields, body.field_offsets(), |index| {
        body.field_id(index).unwrap()
    })
}

fn consume_outer(cursor: &mut &[u8]) {
    consume_expected_fields(cursor);
    for _ in 0..read_word(cursor) {
        read_word(cursor);
        read_word(cursor);
        for _ in 0..read_word(cursor) {
            read_word(cursor);
            read_word(cursor);
        }
    }
}

fn make_edit(case: &load_edit_case::LoadEditCase<'_, u32>) -> SimulationPostDialogEdit {
    let body = SimulationPostDialog::from_bytes(case.original, case.version, LIMITS).unwrap();
    let replacement = SimulationPostDialogValue::RawWordBits(case.replacement as u32);
    let edit = body
        .edit_field(&body.field_id(0).unwrap(), replacement.clone())
        .unwrap();
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.archive().to_bytes(), case.expected);
    assert_eq!(edit.replacement(), &replacement);
    assert_eq!(edit.parent(), edit.source().field_id(0).unwrap());
    assert_eq!(edit.result(), edit.archive().field_id(0).unwrap());
    assert!(case.changed);
    assert_eq!(
        edit.coverage().completion,
        SimulationPostDialogCompletion::Changed
    );
    edit
}

fn project_parents(output: &mut Vec<u8>, edit: &SimulationPostDialogEdit) {
    let source: Vec<_> = (0..edit.source().fields().len())
        .map(|index| edit.source().field_id(index).unwrap())
        .collect();
    let result: Vec<_> = (0..edit.archive().fields().len())
        .map(|index| edit.archive().field_id(index).unwrap())
        .collect();
    project_outer_history(output, edit.history(), &source, &result);
}

fn project_nested(cursor: &mut &[u8], output: &mut Vec<u8>, edit: &SimulationPostDialogEdit) {
    let source = wire_fields(edit.source());
    let result = wire_fields(edit.archive());
    let view = LoadHistoryView {
        source: snapshot(edit.source(), &source),
        result: snapshot(edit.archive(), &result),
        outer: edit.history(),
        nested: edit.nested_history(),
    };
    load_history::project_nested(cursor, output, &view);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let original = *cursor;
    let case = load_edit_case::read_edit_case(cursor, &[0_u32, 1, 2]);
    consume_outer(cursor);
    if case.kind != 0 {
        let nested = read_word(cursor) as usize;
        take_projection(cursor, nested * 28);
        output.extend_from_slice(&original[..original.len() - cursor.len()]);
        return;
    }
    let edit = make_edit(&case);
    output.extend_from_slice(&original[..20 + case.original.len() * 2]);
    output.extend(project_dialog(edit.archive()));
    project_parents(output, &edit);
    project_nested(cursor, output, &edit);
}

#[test]
fn owned_parent_edits_match_complete_field_bytes_and_all_outer_nested_parents() {
    project_packet(
        include_bytes!("../fixtures/simulation-post-dialog/history.bin"),
        495,
        project_case,
    );
}

#[test]
#[ignore = "Requires fresh complete parent-dialog native history expectations."]
fn native_parent_edits_match_complete_field_bytes_and_all_outer_nested_parents() {
    let packet = std::fs::read("target/simulation-post-dialog-native-history.bin").unwrap();
    let mut output = project_packet(
        include_bytes!("../fixtures/simulation-post-dialog/history.bin"),
        495,
        project_case,
    );
    output.extend(project_packet(&packet, 396, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_POST_DIALOG_HISTORY_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
