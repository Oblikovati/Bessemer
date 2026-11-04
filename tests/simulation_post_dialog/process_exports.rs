use super::*;

fn project_complete_parent(body: &SimulationPostDialog, output: &mut Vec<u8>) {
    let raw = body.to_bytes();
    project_word(output, 0);
    project_word(output, body.version().value() as usize);
    project_word(output, raw.len());
    output.extend(raw);
    output.extend(project_dialog(body));
}

#[test]
#[ignore = "Requires freshly verified complete native dialog projections."]
fn all_parent_contexts_export_complete_fields_for_process_comparison() {
    let native = std::fs::read("target/simulation-post-dialog-native-projection.bin").unwrap();
    let mut cases = owned_dialogs();
    cases.extend(dialogs_from_projection(&native, 224));
    let mut output = Vec::new();
    for body in &cases {
        project_complete_parent(body, &mut output);
    }
    assert_eq!(cases.len(), 449);
    if let Some(path) = std::env::var_os("BESSEMER_POST_DIALOG_PROJECTION_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
