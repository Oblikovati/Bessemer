//! Complete installed geometry-stream corpus, including forced field reconstruction.
#[path = "common/composer_scenes.rs"]
mod reference;
use bessemer::composer::geometry::*;

#[test]
#[ignore = "requires freeze_scenes.py --export against the installed corpus"]
fn all_native_scene_streams_reconstruct_construct_and_reopen_completely() {
    let raw = std::fs::read("target/composer-scenes-native.bin").unwrap();
    let cases = reference::scene_cases(&raw);
    assert_eq!(cases.len(), 38);
    let total: usize = cases.iter().map(|case| case.decoded.len()).sum();
    assert_eq!(total, 45280926);
    let mut geodes = 0;
    let mut drawables = 0;
    let mut edits = b"SGED".to_vec();
    edits.extend_from_slice(&(cases.len() as u32).to_le_bytes());
    for case in cases {
        let archive = reference::check_scene(&case);
        geodes += archive.coverage().geodes;
        drawables += archive.coverage().drawables;
        assert_eq!(archive.coverage().sized_extension_bytes, 0);
        append_edit(&mut edits, &case, &archive);
    }
    assert_eq!((geodes, drawables), (1787, 2056));
    std::fs::write("target/composer-scenes-native-edits.bin", edits).unwrap();
}

fn append_edit(
    edits: &mut Vec<u8>,
    case: &reference::SceneCase<'_>,
    archive: &ComposerGeometryArchive,
) {
    let edited = edit_native_name(archive);
    edits.extend_from_slice(&(case.encoded.len() as u32).to_le_bytes());
    edits.extend_from_slice(&(edited.len() as u32).to_le_bytes());
    edits.extend_from_slice(case.encoded);
    edits.extend_from_slice(&edited);
}

fn edit_native_name(archive: &ComposerGeometryArchive) -> Vec<u8> {
    let id = archive.field_id("entries[0].name").unwrap();
    let ComposerSceneValue::Text(original) = &archive.field(&id).unwrap().value else {
        panic!("expected text")
    };
    let text =
        ComposerSceneText::from_text("Bessemer complete scene Ω", original.terminator()).unwrap();
    let edit = archive
        .edit_field(&id, ComposerSceneValue::Text(text))
        .unwrap();
    assert_eq!(archive.fields().len(), edit.archive.fields().len());
    for (before, after) in archive.fields().iter().zip(edit.archive.fields()) {
        if before.path.as_ref() != "entries[0].name" {
            assert_eq!(before, after);
        }
    }
    edit.archive.to_gzip().unwrap()
}
