use super::*;

#[test]
fn every_owned_parent_color_preserves_its_absolute_body_span() {
    for dialog in owned_dialogs() {
        let raw = dialog.to_bytes();
        let colors = dialog.colors().unwrap();
        assert!(!colors.is_empty());
        for color in colors {
            let (start, end) = color.wire_range;
            assert_eq!(end - start, 16);
            assert_eq!(color.color.to_bytes(), raw[start..end]);
            assert!(
                dialog
                    .fields()
                    .iter()
                    .any(|field| field.path.as_ref() == format!("{}.component_0", color.path))
            );
        }
        assert_eq!(dialog.to_bytes(), raw);
    }
}

#[test]
fn all_owned_parent_colors_match_independent_source_paths_spans_bits_and_finite_values() {
    let actual = owned_dialogs()
        .into_iter()
        .map(|dialog| (dialog.version().value(), dialog.colors().unwrap()));
    let count = color_expectations::assert_packet(
        include_bytes!("../fixtures/simulation-post-dialog/rgba-views.bin"),
        0,
        actual,
    );
    assert_eq!(count, 225);
}

#[test]
#[ignore = "Requires fresh independent complete native RGBA and field projections."]
fn all_native_parent_colors_match_independent_source_expectations() {
    let fields = std::fs::read("target/simulation-post-dialog-native-projection.bin").unwrap();
    let colors = std::fs::read("target/simulation-post-rgba-native.bin").unwrap();
    let actual = dialogs_from_projection(&fields, 224)
        .into_iter()
        .map(|dialog| (dialog.version().value(), dialog.colors().unwrap()));
    assert_eq!(color_expectations::assert_packet(&colors, 0, actual), 224);
}

#[test]
fn eight_workers_match_serial_color_queries_for_every_owned_parent() {
    color_expectations::assert_threaded(&owned_dialogs(), SimulationPostDialog::colors);
}

#[test]
#[ignore = "Requires fresh native inputs; optional output supports process replay."]
fn complete_color_projection_for_two_process_replay() {
    let native = std::fs::read("target/simulation-post-dialog-native-projection.bin").unwrap();
    let mut dialogs = owned_dialogs();
    dialogs.extend(dialogs_from_projection(&native, 224));
    let contexts = dialogs
        .into_iter()
        .map(|dialog| (dialog.version().value(), dialog.colors().unwrap()));
    let projected = color_expectations::project_collections(0, contexts);
    if let Some(path) = std::env::var_os("BESSEMER_POST_RGBA_OUTPUT") {
        std::fs::write(path, projected).unwrap();
    }
}
