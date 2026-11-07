//! Complete saved bitmap setting operations against independent Source-owned roots (#187).
#[path = "model_bitmap_transparency/concurrency.rs"]
mod concurrency;
#[path = "model_bitmap_transparency/documents.rs"]
mod documents;
#[path = "support/model_operation_snapshot.rs"]
mod model_operation_snapshot;
#[path = "model_bitmap_transparency/native.rs"]
mod native;
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;
#[path = "support/visual_workers.rs"]
mod visual_workers;
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const ROOTS: &[u8] = include_bytes!("fixtures/bitmap-transparency/roots.bin");
const SCALARS: &str = include_str!("fixtures/bitmap-transparency/scalars.tsv");
const DOMAIN: [i32; 9] = [i32::MIN, -1, 0, 1, 2, 3, 255, 256, i32::MAX];

fn bitmap_context() -> (PartConfigurationArchive, Vec<u8>) {
    let document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let configuration = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    (
        configuration,
        document.read_stream("Contents/CMgr").unwrap(),
    )
}

fn bitmap_images() -> Vec<&'static [u8]> {
    assert_eq!(&ROOTS[..4], b"BTR1");
    assert_eq!(u32::from_le_bytes(ROOTS[4..8].try_into().unwrap()), 731);
    let mut cursor = &ROOTS[8..];
    let mut images = Vec::new();
    while !cursor.is_empty() {
        let length = u32::from_le_bytes(cursor[..4].try_into().unwrap()) as usize;
        images.push(&cursor[4..4 + length]);
        cursor = &cursor[4 + length..];
    }
    assert_eq!(images.len(), 731);
    images
}

fn bitmap_archive(
    raw: &[u8],
    configuration: &PartConfigurationArchive,
    manager: &[u8],
) -> PartResolvedFeaturesArchive {
    let archive =
        PartResolvedFeaturesArchive::from_bytes(raw, configuration.clone(), manager, LIMITS)
            .unwrap();
    assert_eq!(archive.to_bytes().unwrap(), raw);
    archive
}

fn bitmap_scalar(case: usize, owner: usize, role: usize) -> (usize, i32) {
    let row = SCALARS.lines().nth(case * 6 + owner * 3 + role).unwrap();
    let columns: Vec<_> = row.split('\t').collect();
    assert_eq!(columns[0].parse::<usize>().unwrap(), case);
    assert_eq!(columns[1].parse::<usize>().unwrap(), owner);
    (columns[3].parse().unwrap(), columns[4].parse().unwrap())
}

fn setting_request(role: usize, value: i32) -> ModelBitmapTransparencyEdit {
    match role {
        0 => ModelBitmapTransparencyEdit::HasTransparencySetting(value),
        1 => ModelBitmapTransparencyEdit::LevelParameter(value),
        2 => ModelBitmapTransparencyEdit::TypeSelector(value),
        _ => panic!("role {role}; expected0..3"),
    }
}

fn verify_bitmap_edit(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &ModelBitmapTransparencyState,
    role: usize,
    value: i32,
    expected: (usize, i32),
) -> Vec<u8> {
    let result = source
        .edit_bitmap_transparency(state, setting_request(role, value))
        .unwrap();
    verify_bitmap_result(source, raw, state, role, value, expected, &result)
}

fn verify_bitmap_history(
    source: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
    role: usize,
    result: &PartResolvedFeaturesEdit,
    changed: bool,
) {
    if changed {
        verify_bitmap_changed_history(source, state, role, result);
        return;
    }
    let rebound = result
        .archive()
        .bitmap_transparency_state(&state.source_fields()[0])
        .unwrap();
    assert_eq!(rebound, *state);
}

fn verify_bitmap_state(
    case: usize,
    owner: usize,
    archive: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &ModelBitmapTransparencyState,
    output: &mut Option<Vec<u8>>,
) {
    verify_bitmap_projection(case, owner, archive, state);
    for role in 0..3 {
        let expected = bitmap_scalar(case, owner, role);
        for value in DOMAIN {
            let actual = verify_bitmap_edit(archive, raw, state, role, value, expected);
            append_bitmap_export(output, &actual);
        }
    }
}

#[test]
fn all_owned_settings_and_signed_edits_match_independent_literal_oracles() {
    let path = std::env::var_os("BESSEMER_BITMAP_EXPORT_PATH");
    let mut output = path
        .as_ref()
        .map(|_| [b"BTE1".as_slice(), &39474_u32.to_le_bytes()].concat());
    let (configuration, manager) = bitmap_context();
    for (case, raw) in bitmap_images().into_iter().enumerate() {
        let archive = bitmap_archive(raw, &configuration, &manager);
        let states = archive.bitmap_transparency_states().unwrap();
        assert_eq!(states.len(), 2);
        for (owner, state) in states.iter().enumerate() {
            verify_bitmap_state(case, owner, &archive, raw, state, &mut output);
        }
    }
    if let (Some(path), Some(bytes)) = (path, output) {
        std::fs::write(path, bytes).unwrap();
    }
}

#[test]
fn settings_reject_foreign_revisions_and_nonleading_metadata_handles() {
    let (configuration, manager) = bitmap_context();
    let raw = bitmap_images()[0];
    let source = bitmap_archive(raw, &configuration, &manager);
    let foreign = bitmap_archive(raw, &configuration, &manager);
    let state = source.bitmap_transparency_states().unwrap().remove(0);
    verify_foreign_bitmap_revision(&foreign, &state);
    for offset in 1..5 {
        let field = source
            .field_handle(state.source_fields()[0].index() + offset)
            .unwrap();
        let error = source.bitmap_transparency_state(&field).unwrap_err();
        assert!(error.to_string().contains("expected"));
    }
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(foreign.to_bytes().unwrap(), raw);
}

#[test]
fn every_public_bitmap_setting_type_is_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<ModelBitmapTransparencyWords>();
    require_send_sync::<ModelBitmapTransparencyState>();
    require_send_sync::<ModelBitmapTransparencyEdit>();
}

fn verify_bitmap_result_contract(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    changed: bool,
) {
    let completion = if changed {
        ModelEditCompletion::Changed
    } else {
        ModelEditCompletion::Unchanged
    };
    assert_eq!(result.completion(), completion);
    assert_eq!(result.changes().len(), usize::from(changed));
    assert_eq!(
        result.unchanged_fields() + result.changes().len(),
        source.fields().len()
    );
    verify_bitmap_coverage(source, result);
}

fn verify_bitmap_changed_history(
    source: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
    role: usize,
    result: &PartResolvedFeaturesEdit,
) {
    let change = &result.changes()[0];
    let parent = &state.source_fields()[role];
    assert_eq!(change.parents(), std::slice::from_ref(parent));
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(
        change.path(),
        source.resolve_field(parent).unwrap().path.as_ref()
    );
    assert!(
        result
            .archive()
            .resolve_field(change.result().unwrap())
            .is_ok()
    );
    verify_stale_bitmap_revision(result.archive(), state, role);
}

fn verify_stale_bitmap_revision(
    archive: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
    role: usize,
) {
    let expected = archive
        .resolve_field(&state.source_fields()[0])
        .unwrap_err();
    assert_eq!(
        archive
            .bitmap_transparency_state(&state.source_fields()[0])
            .unwrap_err(),
        expected
    );
    assert_eq!(
        archive
            .edit_bitmap_transparency(state, setting_request(role, 0))
            .unwrap_err(),
        expected
    );
}

fn verify_foreign_bitmap_revision(
    foreign: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
) {
    let expected = foreign
        .resolve_field(&state.source_fields()[0])
        .unwrap_err();
    assert_eq!(
        foreign
            .bitmap_transparency_state(&state.source_fields()[0])
            .unwrap_err(),
        expected
    );
    for role in 0..3 {
        assert_eq!(
            foreign
                .edit_bitmap_transparency(state, setting_request(role, 1))
                .unwrap_err(),
            expected
        );
    }
}

fn verify_bitmap_coverage(source: &PartResolvedFeaturesArchive, result: &PartResolvedFeaturesEdit) {
    assert_eq!(result.diagnostics(), source.diagnostics());
    assert_eq!(result.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(
        result.coverage().mode,
        ModelEditCheckMode::ExactStoredArchive
    );
    assert_eq!(
        result.coverage().wire_bytes,
        source.to_bytes().unwrap().len()
    );
    assert_eq!(result.coverage().typed_fields, source.fields().len());
}

#[test]
fn unqualified_bitmap_schemas_report_errors_instead_of_completed_empty_results() {
    let (configuration, manager) = bitmap_context();
    for schema in [0_u16, 2, 0x8001, u16::MAX] {
        let mut raw = bitmap_images()[0].to_vec();
        raw[8..10].copy_from_slice(&schema.to_le_bytes());
        let archive = bitmap_archive(&raw, &configuration, &manager);
        let field = archive
            .fields()
            .iter()
            .position(|field| field.path.ends_with(".raw_70"))
            .unwrap();
        let error = archive
            .bitmap_transparency_state(&archive.field_handle(field).unwrap())
            .unwrap_err();
        assert_eq!(archive.bitmap_transparency_states().unwrap_err(), error);
        assert!(error.cause.contains(&schema.to_string()));
        assert!(error.cause.contains("expected"));
        assert_eq!(archive.to_bytes().unwrap(), raw);
    }
}

#[test]
fn complete_archive_without_bitmaps_returns_an_empty_setting_list() {
    let document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let (configuration, manager) = bitmap_context();
    let raw = document
        .read_stream("Contents/Config-0-ResolvedFeatures")
        .unwrap();
    let archive = bitmap_archive(&raw, &configuration, &manager);
    assert!(archive.bitmap_transparency_states().unwrap().is_empty());
}

fn append_bitmap_export(output: &mut Option<Vec<u8>>, actual: &[u8]) {
    if let Some(output) = output {
        output.extend_from_slice(&u32::try_from(actual.len()).unwrap().to_le_bytes());
        output.extend_from_slice(actual);
    }
}

fn verify_bitmap_projection(
    case: usize,
    owner: usize,
    archive: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
) {
    let words = state.words();
    let actual = [
        words.has_transparency_setting,
        words.level_parameter,
        words.type_selector,
    ];
    assert_eq!(
        actual,
        std::array::from_fn(|role| bitmap_scalar(case, owner, role).1)
    );
    let rebound = archive
        .bitmap_transparency_state(&state.source_fields()[0])
        .unwrap();
    assert_eq!(rebound, *state);
}

fn verify_bitmap_result(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &ModelBitmapTransparencyState,
    role: usize,
    value: i32,
    expected: (usize, i32),
    result: &PartResolvedFeaturesEdit,
) -> Vec<u8> {
    let mut literal = raw.to_vec();
    literal[expected.0..expected.0 + 4].copy_from_slice(&value.to_le_bytes());
    let actual = result.archive().to_bytes().unwrap();
    assert_eq!(actual, literal);
    let changed = value != expected.1;
    verify_bitmap_result_contract(source, result, changed);
    verify_bitmap_history(source, state, role, result, changed);
    assert_eq!(source.to_bytes().unwrap(), raw);
    actual
}
