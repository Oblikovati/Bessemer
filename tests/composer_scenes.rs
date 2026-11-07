//! Complete typed scene operations against independent gzip and reference fixtures.
#[path = "common/composer_scenes.rs"]
mod reference;
use bessemer::composer::geometry::*;

#[test]
fn all_owned_scenes_reconstruct_construct_and_reopen_completely() {
    let cases = reference::scene_cases(include_bytes!("fixtures/composer-scenes/scenes.bin"));
    assert_eq!(cases.len(), 45);
    for case in &cases {
        reference::check_scene(case);
    }
}

fn mixed() -> ComposerGeometryArchive {
    let cases = reference::scene_cases(include_bytes!("fixtures/composer-scenes/scenes.bin"));
    reference::check_scene(&cases[32])
}

fn text(value: &str) -> ComposerSceneValue {
    ComposerSceneValue::Text(
        ComposerSceneText::from_text(value, ComposerTextTerminator::Newline).unwrap(),
    )
}

#[test]
fn unbounded_geode_name_edit_matches_independent_complete_scene_golden() {
    let cases = reference::scene_cases(include_bytes!("fixtures/composer-scenes/scenes.bin"));
    let source = mixed();
    let id = source.field_id("entries[0].name").unwrap();
    let edit = source.edit_field(&id, text("Bessemer Ω")).unwrap();
    assert_eq!(edit.archive.to_decoded().unwrap(), cases[33].decoded);
    assert_eq!(source.to_decoded().unwrap(), cases[32].decoded);
    let restored = edit
        .archive
        .edit_field(
            &edit.archive.field_id("entries[0].name").unwrap(),
            text("geode"),
        )
        .unwrap();
    assert_eq!(restored.archive.to_decoded().unwrap(), cases[32].decoded);
}

#[test]
fn bounded_material_text_edit_derives_size_and_matches_independent_scene_golden() {
    let cases = reference::scene_cases(include_bytes!("fixtures/composer-scenes/scenes.bin"));
    let source = mixed();
    let path = "entries[0].lods[0].faces[0].attributes.textures.names[0]";
    let edit = source
        .edit_field(&source.field_id(path).unwrap(), text("Bessemer texture Ω"))
        .unwrap();
    assert_eq!(edit.archive.to_decoded().unwrap(), cases[34].decoded);
    assert_eq!(edit.coverage.mode, ComposerSceneCheckMode::ExactSceneWire);
    assert_eq!(edit.fidelity, ComposerSceneEditFidelity::ExactTypedFields);
    assert_eq!(&edit.history.parent, source.revision());
    assert_eq!(&edit.history.result, edit.archive.revision());
    assert_ne!(edit.history.parent, edit.history.result);
}

#[test]
fn information_and_userdata_text_edits_follow_their_distinct_size_rules() {
    let cases = reference::scene_cases(include_bytes!("fixtures/composer-scenes/scenes.bin"));
    let source = mixed();
    let info =
        ComposerSceneText::from_text("Bessemer info Ω", ComposerTextTerminator::Nul).unwrap();
    let id = source.field_id("information.string_68").unwrap();
    let edited = source
        .edit_field(&id, ComposerSceneValue::Text(info))
        .unwrap();
    assert_eq!(edited.archive.to_decoded().unwrap(), cases[35].decoded);
    let id = source.field_id("entries[0].userdata.guid").unwrap();
    let edited = source.edit_field(&id, text("Bessemer GUID Ω")).unwrap();
    assert_eq!(edited.archive.to_decoded().unwrap(), cases[36].decoded);
}

#[test]
fn named_metadata_fields_preserve_native_order_bits_and_noncanonical_control_values() {
    let source = mixed();
    check_info_words(&source);
    let id = source.field_id("entries[0].matrix_presence").unwrap();
    assert_eq!(
        source.field(&id).unwrap().value,
        ComposerSceneValue::Byte(255)
    );
    let id = source
        .field_id("entries[0].lods[0].faces[0].attributes.material_ec")
        .unwrap();
    assert_eq!(
        source.field(&id).unwrap().value,
        ComposerSceneValue::Word(9)
    );
}

fn check_info_words(source: &ComposerGeometryArchive) {
    for (name, value) in [
        ("3c", 0),
        ("34", 1),
        ("54", 2),
        ("44", 3),
        ("4c", 4),
        ("5c", 5),
    ] {
        let id = source.field_id(&format!("information.raw_{name}")).unwrap();
        assert_eq!(
            source.field(&id).unwrap().value,
            ComposerSceneValue::Wide(value)
        );
    }
}

#[test]
fn complete_scene_vector_fields_feed_the_explicit_numeric_profile() {
    use bessemer::composer::arrays::ComposerVectorProfile;
    let source = mixed();
    let id = source
        .field_id("entries[0].lods[0].faces[0].geometry.coordinates")
        .unwrap();
    let ComposerSceneValue::Vector(block) = &source.field(&id).unwrap().value else {
        panic!("expected vector")
    };
    let profile = ComposerVectorProfile::NearestEvenGradual79V1;
    let values = block.reconstruct(profile, 9).unwrap();
    let actual = values
        .patterns()
        .iter()
        .map(|value| value.bits())
        .collect::<Vec<_>>();
    assert_eq!(actual, block.codes().collect::<Vec<_>>());
    assert!(values.report().coverage.raw_copy_only);
}

#[test]
fn text_retains_surrogates_and_both_terminators_without_replacement_or_injection() {
    let raw = ComposerSceneText::from_units(&[65, 0xd800], ComposerTextTerminator::Nul).unwrap();
    assert_eq!(raw.code_units(), [65, 0xd800]);
    assert_eq!(raw.terminator(), ComposerTextTerminator::Nul);
    assert!(raw.to_text().is_err());
    let unicode = ComposerSceneText::from_text("Ω", ComposerTextTerminator::Newline).unwrap();
    assert_eq!(unicode.to_text().unwrap(), "Ω");
    assert!(ComposerSceneText::from_units(&[10], ComposerTextTerminator::Nul).is_err());
    assert!(ComposerSceneText::from_text("a\0b", ComposerTextTerminator::Newline).is_err());
}

#[test]
fn noop_and_failed_edits_leave_owned_fields_and_original_gzip_unchanged() {
    let source = mixed();
    let original = source.to_gzip().unwrap();
    let id = source.field_id("entries[0].name").unwrap();
    let noop = source.edit_field(&id, text("geode")).unwrap();
    assert_eq!(noop.archive.revision(), source.revision());
    assert_eq!(
        noop.completion,
        bessemer::display_lists::DisplayEditCompletion::Unchanged
    );
    assert!(source.edit_field(&id, ComposerSceneValue::Word(0)).is_err());
    let size = source.field_id("information.logical_size").unwrap();
    assert!(
        source
            .edit_field(&size, ComposerSceneValue::Word(0))
            .is_err()
    );
    assert!(source.field_id("absent field").is_err());
    assert_eq!(source.to_gzip().unwrap(), original);
}

#[test]
fn every_public_scene_type_is_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<ComposerGeometryArchive>();
    assert_traits::<ComposerSceneContext>();
    assert_traits::<ComposerSceneLimits>();
    assert_traits::<ComposerSceneRevision>();
    assert_traits::<ComposerSceneFieldId>();
    assert_traits::<ComposerSceneField>();
    assert_traits::<ComposerSceneValue>();
    assert_traits::<ComposerSceneText>();
    assert_traits::<ComposerTextTerminator>();
    assert_traits::<ComposerSceneCheckMode>();
    assert_traits::<ComposerSceneCoverage>();
    assert_traits::<ComposerSceneDiagnostic>();
    assert_traits::<ComposerSceneEditFidelity>();
    assert_traits::<ComposerSceneHistory>();
    assert_traits::<ComposerSceneEdit>();
}
