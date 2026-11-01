use super::*;

fn consume_history(cursor: &mut &[u8]) {
    consume_expected_fields(cursor);
    for _ in 0..read_word(cursor) {
        read_word(cursor);
        read_word(cursor);
        for _ in 0..read_word(cursor) {
            read_word(cursor);
            read_word(cursor);
        }
    }
    let nested = read_word(cursor) as usize;
    take_projection(cursor, nested * 28);
}

fn project_info_history(output: &mut Vec<u8>, edit: &SimulationPostInfoEdit) {
    output.extend(project_info(edit.archive()));
    let source: Vec<_> = (0..edit.source().fields().len())
        .map(|index| edit.source().field_id(index).unwrap())
        .collect();
    let result: Vec<_> = (0..edit.archive().fields().len())
        .map(|index| edit.archive().field_id(index).unwrap())
        .collect();
    project_outer_history(output, edit.history(), &source, &result);
    assert!(edit.nested_history().is_empty());
    project_word(output, 0);
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let original = *cursor;
    let case = load_edit_case::read_edit_case(cursor, &[0_u32, 1, 2]);
    consume_history(cursor);
    if case.kind != 1 {
        output.extend_from_slice(&original[..original.len() - cursor.len()]);
        return;
    }
    let body = SimulationPostInfo::from_bytes(case.original, case.version, LIMITS).unwrap();
    let edit = body
        .edit_field(
            &body.field_id(0).unwrap(),
            SimulationPostInfoValue::RawWordBits(case.replacement as u32),
        )
        .unwrap();
    assert!(case.changed);
    assert_eq!(edit.source().to_bytes(), case.original);
    assert_eq!(edit.archive().to_bytes(), case.expected);
    output.extend_from_slice(&original[..20 + case.original.len() * 2]);
    project_info_history(output, &edit);
}

#[test]
fn every_owned_info_literal_edit_matches_all_ordered_parents() {
    project_packet(
        include_bytes!("../fixtures/simulation-post-dialog/history.bin"),
        495,
        project_case,
    );
}

#[test]
#[ignore = "Requires fresh complete native dialog history packets from freeze_post_dialog.py."]
fn every_native_info_literal_edit_matches_all_ordered_parents() {
    let packet = std::fs::read("target/simulation-post-dialog-native-history.bin").unwrap();
    project_packet(&packet, 396, project_case);
}
