//! Complete geometry transport, context, resource and typed-field rejection checks.
#[path = "common/composer_scenes.rs"]
mod reference;
use bessemer::composer::geometry::*;

fn cases() -> Vec<reference::SceneCase<'static>> {
    reference::scene_cases(include_bytes!("fixtures/composer-scenes/scenes.bin"))
}

#[test]
fn all_owned_compressed_prefixes_and_unexplained_decoded_suffixes_fail() {
    for case in cases() {
        for end in 0..case.encoded.len() {
            assert!(
                ComposerGeometryArchive::from_gzip(
                    &case.encoded[..end],
                    case.context,
                    reference::LIMITS
                )
                .is_err()
            );
        }
        let mut decoded = case.decoded.to_vec();
        decoded.push(b'X');
        assert!(
            ComposerGeometryArchive::from_decoded(&decoded, case.context, reference::LIMITS)
                .is_err()
        );
    }
}

#[test]
fn gzip_crc_size_optional_flags_and_incomplete_deflate_are_checked() {
    let case = &cases()[32];
    for position in [3, case.encoded.len() - 8, case.encoded.len() - 4] {
        let mut encoded = case.encoded.to_vec();
        encoded[position] ^= 1;
        assert!(
            ComposerGeometryArchive::from_gzip(&encoded, case.context, reference::LIMITS).is_err()
        );
    }
    let mut encoded = case.encoded.to_vec();
    encoded.remove(10);
    assert!(ComposerGeometryArchive::from_gzip(&encoded, case.context, reference::LIMITS).is_err());
    let mut decoded = case.decoded.to_vec();
    decoded[..8].copy_from_slice(&7.8_f64.to_le_bytes());
    assert!(
        ComposerGeometryArchive::from_decoded(&decoded, case.context, reference::LIMITS).is_err()
    );
}

#[test]
fn every_scene_budget_applies_before_publication_or_output() {
    let case = &cases()[32];
    let mut limits = reference::LIMITS;
    limits.maximum_encoded_bytes = case.encoded.len() - 1;
    reject_limit(case, limits);
    limits = reference::LIMITS;
    limits.maximum_decoded_bytes = case.decoded.len() - 1;
    reject_limit(case, limits);
    limits = reference::LIMITS;
    limits.maximum_fields = reference::check_scene(case).fields().len() - 1;
    reject_limit(case, limits);
    limits = reference::LIMITS;
    limits.maximum_records = 0;
    reject_limit(case, limits);
    limits = reference::LIMITS;
    limits.maximum_text_units = 0;
    reject_limit(case, limits);
}

fn reject_limit(case: &reference::SceneCase<'_>, limits: ComposerSceneLimits) {
    assert!(ComposerGeometryArchive::from_gzip(case.encoded, case.context, limits).is_err());
}

#[test]
fn bounded_gzip_output_failure_keeps_complete_decoded_fields_available() {
    let case = &cases()[32];
    let limits = ComposerSceneLimits {
        maximum_encoded_bytes: 18,
        ..reference::LIMITS
    };
    let decoded =
        ComposerGeometryArchive::from_decoded(case.decoded, case.context, limits).unwrap();
    assert!(decoded.to_gzip().is_err());
    assert_eq!(decoded.to_decoded().unwrap(), case.decoded);
}

#[test]
fn changed_typed_paths_counts_and_array_caller_contexts_cannot_be_hidden() {
    let case = &cases()[32];
    let source = reference::check_scene(case);
    let mut fields = source.fields().to_vec();
    fields[0].path = "wrong-version-path".into();
    assert!(
        ComposerGeometryArchive::from_fields(&fields, case.context, reference::LIMITS).is_err()
    );
    fields = source.fields().to_vec();
    fields.pop();
    assert!(
        ComposerGeometryArchive::from_fields(&fields, case.context, reference::LIMITS).is_err()
    );
}

#[test]
fn appearance_import_context_is_required_for_positive_strip_texture_layers() {
    let false_context = ComposerSceneContext {
        texture_import_enabled: false,
    };
    assert!(
        ComposerGeometryArchive::from_decoded(
            cases()[10].decoded,
            false_context,
            reference::LIMITS
        )
        .is_err()
    );
}

#[test]
fn byte_identical_index_values_cannot_carry_a_different_declared_domain() {
    let case = &cases()[5];
    let source = reference::check_scene(case);
    let mut fields = source.fields().to_vec();
    let field = fields
        .iter_mut()
        .find(|field| field.path.ends_with(".coordinate_indices"))
        .unwrap();
    let ComposerSceneValue::Indices(original) = &field.value else {
        panic!("expected index block")
    };
    field.value = changed_domain(original);
    assert!(
        ComposerGeometryArchive::from_fields(&fields, case.context, reference::LIMITS).is_err()
    );
}

fn changed_domain(original: &bessemer::composer::arrays::ComposerIndexBlock) -> ComposerSceneValue {
    use bessemer::composer::arrays::ComposerIndexBlock;
    let raw = original.to_bytes();
    let changed =
        ComposerIndexBlock::from_bytes(&raw, original.count(), original.domain() + 1, 100).unwrap();
    ComposerSceneValue::Indices(changed)
}

#[test]
fn byte_identical_constant_vectors_cannot_change_their_parent_row_context() {
    let case = &cases()[44];
    let source = reference::check_scene(case);
    let mut fields = source.fields().to_vec();
    let field = fields
        .iter_mut()
        .find(|field| matches!(field.value, ComposerSceneValue::Vector(_)))
        .unwrap();
    let ComposerSceneValue::Vector(original) = &field.value else {
        panic!("expected vector")
    };
    let changed = changed_vector(original);
    field.value = ComposerSceneValue::Vector(changed);
    assert!(
        ComposerGeometryArchive::from_fields(&fields, case.context, reference::LIMITS).is_err()
    );
}

fn changed_vector(
    original: &bessemer::composer::arrays::ComposerVectorBlock,
) -> bessemer::composer::arrays::ComposerVectorBlock {
    use bessemer::composer::arrays::ComposerVectorBlock;
    let layout = bessemer::composer::arrays::ComposerVectorLayout {
        count: 2,
        ..original.layout()
    };
    ComposerVectorBlock::from_parts(
        layout,
        original.packing(),
        original.axes(),
        original.packed_bytes(),
        100,
    )
    .unwrap()
}

#[test]
fn input_buffers_and_caller_fields_are_owned_and_concurrent_edits_match() {
    let case = &cases()[32];
    let mut decoded = case.decoded.to_vec();
    let source =
        ComposerGeometryArchive::from_decoded(&decoded, case.context, reference::LIMITS).unwrap();
    decoded.fill(0);
    assert_eq!(source.to_decoded().unwrap(), case.decoded);
    let mut fields = source.fields().to_vec();
    let constructed =
        ComposerGeometryArchive::from_fields(&fields, case.context, reference::LIMITS).unwrap();
    fields.clear();
    assert_eq!(constructed.to_decoded().unwrap(), case.decoded);
}

#[test]
fn concurrent_scene_edits_match_sequential_bytes_and_keep_the_original_unchanged() {
    let source = reference::check_scene(&cases()[32]);
    let expected = edited_name_bytes(&source);
    std::thread::scope(|scope| {
        let handles = (0..6)
            .map(|_| scope.spawn(|| edited_name_bytes(&source)))
            .collect::<Vec<_>>();
        for handle in handles {
            assert_eq!(handle.join().unwrap(), expected);
        }
    });
    assert_eq!(source.to_decoded().unwrap(), cases()[32].decoded);
}

fn edited_name_bytes(source: &ComposerGeometryArchive) -> Vec<u8> {
    let id = source.field_id("entries[0].name").unwrap();
    let text = ComposerSceneText::from_text("parallel Ω", ComposerTextTerminator::Newline).unwrap();
    source
        .edit_field(&id, ComposerSceneValue::Text(text))
        .unwrap()
        .archive
        .to_gzip()
        .unwrap()
}
