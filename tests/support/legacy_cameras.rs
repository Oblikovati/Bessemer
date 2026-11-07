//! Camera, viewer and text fields follow separate historical presence gates.
use super::*;

fn camera_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-camera-roots.bin"
    ))
}

#[test]
fn camera_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(camera_cases().len(), 66);
    assert_eq!(
        camera_cases().into_iter().map(check_case).sum::<usize>(),
        8192
    );
}

fn check_camera_presence(case: OldCase) {
    let archive = parsed(case);
    assert_eq!(
        field(&archive, "root.views[0].camera_name").is_some(),
        case.version > 0xbb6
    );
    for member in ["is_3d_text", "plane_xform"] {
        let path = format!("root.views[0].display_items.items[4].data.{member}");
        assert_eq!(field(&archive, &path).is_some(), case.version > 0xb74);
    }
    let viewer = archive
        .contents()
        .fields
        .iter()
        .any(|field| field.path.ends_with("viewer_comp_data.unknown_190"));
    assert_eq!(
        viewer,
        case.kind == DisplayDocumentKind::Assembly && case.scenario != 2 && case.version >= 0xb9b
    );
}

#[test]
fn camera_text_and_viewer_fields_obey_independent_presence_boundaries() {
    for case in camera_cases() {
        check_camera_presence(case);
    }
}
