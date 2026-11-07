//! Compose complete geometry edits with immutable SMG carrier operations.
#[path = "common/composer_scenes.rs"]
mod reference;
use bessemer::composer::geometry::*;
use bessemer::composer::{
    ComposerArchive, ComposerCompression, ComposerLimits, ComposerMemberInput,
};

const CARRIER_LIMITS: ComposerLimits = ComposerLimits {
    maximum_bytes: 100000,
    maximum_members: 5,
    maximum_member_bytes: 10000,
    maximum_expanded_bytes: 20000,
};

#[test]
fn geometry_edits_write_into_a_carrier_and_leave_other_member_contents_unchanged() {
    let cases = reference::scene_cases(include_bytes!("fixtures/composer-scenes/scenes.bin"));
    let scene = reference::check_scene(&cases[32]);
    let carrier = make_carrier(cases[32].encoded);
    let id = carrier.member_named("scene.smgGeom").unwrap();
    let replacement = edited_geometry(&scene);
    let edit = carrier.edit_member(&id, &replacement).unwrap();
    let result = edit.archive();
    let payload = result
        .read_member(&result.member_named("scene.smgGeom").unwrap())
        .unwrap();
    let reopened =
        ComposerGeometryArchive::from_gzip(payload, cases[32].context, reference::LIMITS).unwrap();
    assert_eq!(reopened.to_decoded().unwrap(), cases[33].decoded);
    let keep_id = result.member_named("untouched.bin").unwrap();
    assert_eq!(result.read_member(&keep_id).unwrap(), b"keep");
    assert_eq!(carrier.read_member(&id).unwrap(), cases[32].encoded);
}

fn make_carrier(encoded: &[u8]) -> ComposerArchive {
    let members = vec![
        ComposerMemberInput {
            name: "scene.smgGeom".into(),
            contents: encoded.into(),
            compression: ComposerCompression::Deflated,
        },
        ComposerMemberInput {
            name: "untouched.bin".into(),
            contents: b"keep".to_vec(),
            compression: ComposerCompression::Stored,
        },
    ];
    ComposerArchive::from_members(members, b"scene integration", CARRIER_LIMITS).unwrap()
}

fn edited_geometry(scene: &ComposerGeometryArchive) -> Vec<u8> {
    let id = scene.field_id("entries[0].name").unwrap();
    scene
        .edit_field(&id, replacement_name())
        .unwrap()
        .archive
        .to_gzip()
        .unwrap()
}

fn replacement_name() -> ComposerSceneValue {
    ComposerSceneValue::Text(
        ComposerSceneText::from_text("Bessemer Ω", ComposerTextTerminator::Newline).unwrap(),
    )
}
