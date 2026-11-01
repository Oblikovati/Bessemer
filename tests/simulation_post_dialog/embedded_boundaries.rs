use super::*;

fn embedded_parent_case() -> (SimulationPostDialog, usize) {
    for body in owned_dialogs() {
        let ordinal = body
            .fields()
            .iter()
            .position(|field| matches!(field.value, SimulationPostDialogValue::Embedded(_)));
        if let Some(ordinal) = ordinal {
            return (body, ordinal);
        }
    }
    panic!("expected an independently frozen parent containing a scoped reference archive");
}

fn replace_embedded_root(body: &SimulationPostDialog, ordinal: usize, root: &[u8]) -> Vec<u8> {
    let original = body.to_bytes();
    let (start, end) = body.field_offsets()[ordinal];
    [&original[..start], root, &original[end..]].concat()
}

#[test]
fn reference_archives_apply_their_own_version_profile_with_complete_null_roots() {
    let (body, ordinal) = embedded_parent_case();
    for version in [4281_u32, 4307, 19000, 4280, 4282, 4306, 19001] {
        let root = [version.to_le_bytes().as_slice(), &[0, 0]].concat();
        let raw = replace_embedded_root(&body, ordinal, &root);
        let result = SimulationPostDialog::from_bytes(&raw, body.version(), LIMITS);
        let accepted = matches!(version, 4281 | 4307 | 19000);
        assert_eq!(result.is_ok(), accepted, "embedded version {version}");
        if let Ok(parsed) = result {
            assert_eq!(parsed.to_bytes(), raw);
        }
    }
}

#[test]
fn ordinary_model_class_names_cannot_escape_the_reference_archive_dialect() {
    let (body, ordinal) = embedded_parent_case();
    let original = body.to_bytes();
    let name = b"moCameraFeature_c";
    let mut root = 19000_u32.to_le_bytes().to_vec();
    root.extend_from_slice(&[0xff, 0xff, 1, 0]);
    root.extend_from_slice(&(name.len() as u16).to_le_bytes());
    root.extend_from_slice(name);
    let raw = replace_embedded_root(&body, ordinal, &root);
    let error = SimulationPostDialog::from_bytes(&raw, body.version(), LIMITS).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Unsupported);
    assert!(error.cause.contains("moCameraFeature_c"));
    assert!(error.cause.contains("Simulation embedded reader"));
    assert_eq!(body.to_bytes(), original);
}
