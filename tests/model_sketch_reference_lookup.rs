//! Canonical saved sketch-reference keys and independent complete-root edits (#188).
#[path = "model_sketch_reference_lookup/documents.rs"]
mod documents;
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;
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
const ROOTS: &[u8] = include_bytes!("fixtures/sketch-reference-configuration-keys/roots.bin");
const SCALARS: &str = include_str!("fixtures/sketch-reference-configuration-keys/scalars.tsv");
const SIGNED_REQUESTS: [i32; 10] = [
    i32::MIN,
    i32::MIN + 1,
    -12346,
    -12345,
    -12344,
    -1,
    0,
    1,
    i32::MAX - 1,
    i32::MAX,
];
type LookupKeys = ModelSketchReferenceStoredLookupKeys;
type LookupState = ModelSketchReferenceStoredLookupState;
type LookupEdit = ModelSketchReferenceStoredLookupEdit;

fn lookup_context() -> (PartConfigurationArchive, Vec<u8>) {
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

fn lookup_images() -> Vec<&'static [u8]> {
    assert_eq!(&ROOTS[..4], b"SKK1");
    assert_eq!(u32::from_le_bytes(ROOTS[4..8].try_into().unwrap()), 400);
    let mut cursor = &ROOTS[8..];
    let mut images = Vec::new();
    while !cursor.is_empty() {
        let length = u32::from_le_bytes(cursor[..4].try_into().unwrap()) as usize;
        images.push(&cursor[4..4 + length]);
        cursor = &cursor[4 + length..];
    }
    assert_eq!(images.len(), 400);
    images
}

fn lookup_archive(
    raw: &[u8],
    configuration: &PartConfigurationArchive,
    manager: &[u8],
) -> PartResolvedFeaturesArchive {
    let source =
        PartResolvedFeaturesArchive::from_bytes(raw, configuration.clone(), manager, LIMITS)
            .unwrap();
    assert_eq!(source.to_bytes().unwrap(), raw);
    source
}

fn lookup_literal_pair(case: usize, owner: usize) -> ([usize; 2], [i32; 2]) {
    let rows: Vec<_> = SCALARS
        .lines()
        .skip(case * 4 + owner * 2)
        .take(2)
        .map(|row| row.split('\t').collect::<Vec<_>>())
        .collect();
    for row in &rows {
        assert_eq!(row[0].parse::<usize>().unwrap(), case);
        assert_eq!(row[1].parse::<usize>().unwrap(), owner);
    }
    (
        std::array::from_fn(|role| rows[role][3].parse().unwrap()),
        std::array::from_fn(|role| rows[role][4].parse().unwrap()),
    )
}

fn lookup_request(kind: usize, value: i32) -> LookupEdit {
    match kind {
        0 => LookupEdit::ConfigurationObjectKey(value),
        1 => LookupEdit::ConfigurationRecordKey(value),
        2 => LookupEdit::BothKeys {
            configuration_object_key: value,
            configuration_record_key: !value,
        },
        _ => panic!("request kind {kind}; expected0..3"),
    }
}

fn expected_lookup_keys(original: [i32; 2], kind: usize, value: i32) -> [i32; 2] {
    match kind {
        0 => [value, original[1]],
        1 => [original[0], value],
        2 => [value, !value],
        _ => panic!("request kind {kind}; expected0..3"),
    }
}

fn verify_lookup_coverage(source: &PartResolvedFeaturesArchive, result: &PartResolvedFeaturesEdit) {
    assert_eq!(
        result.coverage().mode,
        ModelEditCheckMode::ExactStoredArchive
    );
    assert_eq!(
        result.coverage().wire_bytes,
        source.to_bytes().unwrap().len()
    );
    assert_eq!(result.coverage().typed_fields, source.fields().len());
    assert_eq!(result.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(result.diagnostics(), source.diagnostics());
    assert_eq!(
        result.unchanged_fields() + result.changes().len(),
        source.fields().len()
    );
}

fn verify_lookup_change(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    change: &ModelFieldChange,
    parent: &ModelFieldHandle,
    expected: i32,
) {
    assert_eq!(change.parents(), std::slice::from_ref(parent));
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(
        change.path(),
        source.resolve_field(parent).unwrap().path.as_ref()
    );
    let selected = result
        .archive()
        .resolve_field(change.result().unwrap())
        .unwrap();
    assert_eq!(selected.value, ModelFieldValue::I32(expected));
}

fn verify_lookup_history(
    source: &PartResolvedFeaturesArchive,
    state: &LookupState,
    result: &PartResolvedFeaturesEdit,
    expected: [i32; 2],
) {
    let changed = lookup_changed_roles(state, expected);
    assert_eq!(result.changes().len(), changed.len());
    for (change, role) in result.changes().iter().zip(changed) {
        verify_lookup_change(
            source,
            result,
            change,
            &state.source_fields()[role],
            expected[role],
        );
    }
    let completion = if result.changes().is_empty() {
        ModelEditCompletion::Unchanged
    } else {
        ModelEditCompletion::Changed
    };
    assert_eq!(result.completion(), completion);
    verify_lookup_revisions(state, result);
}

fn verify_lookup_revisions(state: &LookupState, result: &PartResolvedFeaturesEdit) {
    if result.completion() == ModelEditCompletion::Unchanged {
        let rebound = result
            .archive()
            .sketch_reference_lookup_state(&state.source_fields()[0])
            .unwrap();
        assert_eq!(rebound, *state);
        return;
    }
    verify_stale_lookup_requests(result.archive(), state);
}

fn verify_lookup_operation(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &LookupState,
    offsets: [usize; 2],
    expected: [i32; 2],
    result: &PartResolvedFeaturesEdit,
) -> Vec<u8> {
    let mut literal = raw.to_vec();
    for (offset, value) in offsets.into_iter().zip(expected) {
        literal[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    let actual = result.archive().to_bytes().unwrap();
    assert_eq!(actual, literal);
    verify_lookup_coverage(source, result);
    verify_lookup_history(source, state, result, expected);
    verify_lookup_rebound(result.archive(), state, expected);
    assert_eq!(source.to_bytes().unwrap(), raw);
    actual
}

fn verify_lookup_owner(
    case: usize,
    owner: usize,
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &LookupState,
    output: &mut Option<Vec<u8>>,
) {
    let (offsets, original) = lookup_literal_pair(case, owner);
    verify_lookup_projection(source, state, original);
    for kind in 0..3 {
        for value in SIGNED_REQUESTS {
            let result = source
                .edit_sketch_reference_lookup_keys(state, lookup_request(kind, value))
                .unwrap();
            let actual = verify_lookup_operation(
                source,
                raw,
                state,
                offsets,
                expected_lookup_keys(original, kind, value),
                &result,
            );
            append_lookup_export(output, &actual);
        }
    }
}

#[test]
fn all_canonical_owned_lookup_key_edits_match_complete_literal_source_roots() {
    let path = std::env::var_os("BESSEMER_LOOKUP_EXPORT_PATH");
    let mut output = path
        .as_ref()
        .map(|_| [b"SKE1".as_slice(), &24000_u32.to_le_bytes()].concat());
    let (configuration, manager) = lookup_context();
    let parent = configuration.to_bytes().unwrap();
    for (case, raw) in lookup_images().into_iter().enumerate() {
        let source = lookup_archive(raw, &configuration, &manager);
        let states = source.sketch_reference_lookup_states().unwrap();
        assert_eq!(states.len(), 2);
        for (owner, state) in states.iter().enumerate() {
            verify_lookup_owner(case, owner, &source, raw, state, &mut output);
        }
    }
    assert_eq!(configuration.to_bytes().unwrap(), parent);
    if let (Some(path), Some(bytes)) = (path, output) {
        std::fs::write(path, bytes).unwrap();
    }
}

fn verify_foreign_lookup_requests(foreign: &PartResolvedFeaturesArchive, state: &LookupState) {
    let expected = foreign
        .resolve_field(&state.source_fields()[0])
        .unwrap_err();
    assert_eq!(
        foreign
            .sketch_reference_lookup_state(&state.source_fields()[0])
            .unwrap_err(),
        expected
    );
    for kind in 0..3 {
        assert_eq!(
            foreign
                .edit_sketch_reference_lookup_keys(state, lookup_request(kind, -12345))
                .unwrap_err(),
            expected
        );
    }
}

#[test]
fn lookup_keys_reject_foreign_revisions_and_nonleading_word_handles() {
    let (configuration, manager) = lookup_context();
    let raw = lookup_images()[0];
    let source = lookup_archive(raw, &configuration, &manager);
    let foreign = lookup_archive(raw, &configuration, &manager);
    let state = source.sketch_reference_lookup_states().unwrap().remove(0);
    verify_foreign_lookup_requests(&foreign, &state);
    for index in [
        state.source_fields()[0].index() - 1,
        state.source_fields()[1].index(),
    ] {
        let error = source
            .sketch_reference_lookup_state(&source.field_handle(index).unwrap())
            .unwrap_err();
        assert!(error.cause.contains("expected"));
    }
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(foreign.to_bytes().unwrap(), raw);
}

#[test]
fn unqualified_lookup_schemas_return_errors_instead_of_completed_empty_results() {
    let (configuration, manager) = lookup_context();
    for schema in [0_u16, 2, 0x8001, u16::MAX] {
        let mut raw = lookup_images()[0].to_vec();
        raw[8..10].copy_from_slice(&schema.to_le_bytes());
        let source = lookup_archive(&raw, &configuration, &manager);
        let index = source
            .fields()
            .iter()
            .position(|field| field.path.ends_with(".unknown_6c"))
            .unwrap();
        let expected = source
            .sketch_reference_lookup_state(&source.field_handle(index).unwrap())
            .unwrap_err();
        assert_eq!(
            source.sketch_reference_lookup_states().unwrap_err(),
            expected
        );
        assert!(expected.cause.contains(&schema.to_string()));
    }
}

#[test]
fn complete_archive_without_sketch_references_returns_an_empty_key_list() {
    let document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let (configuration, manager) = lookup_context();
    let raw = document
        .read_stream("Contents/Config-0-ResolvedFeatures")
        .unwrap();
    assert!(
        lookup_archive(&raw, &configuration, &manager)
            .sketch_reference_lookup_states()
            .unwrap()
            .is_empty()
    );
}

#[test]
fn every_public_saved_lookup_type_is_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<LookupKeys>();
    require_send_sync::<LookupState>();
    require_send_sync::<LookupEdit>();
}

fn lookup_changed_roles(state: &LookupState, expected: [i32; 2]) -> Vec<usize> {
    let keys = state.keys();
    [keys.configuration_object_key, keys.configuration_record_key]
        .into_iter()
        .zip(expected)
        .enumerate()
        .filter_map(|(role, (original, new))| (original != new).then_some(role))
        .collect()
}

fn verify_stale_lookup_requests(archive: &PartResolvedFeaturesArchive, state: &LookupState) {
    let leading = &state.source_fields()[0];
    let expected = archive.resolve_field(leading).unwrap_err();
    let projection = archive.sketch_reference_lookup_state(leading).unwrap_err();
    assert_eq!(projection, expected);
    for kind in 0..3 {
        let rejection = archive
            .edit_sketch_reference_lookup_keys(state, lookup_request(kind, 0))
            .unwrap_err();
        assert_eq!(rejection, expected);
    }
}

fn verify_lookup_rebound(
    archive: &PartResolvedFeaturesArchive,
    state: &LookupState,
    expected: [i32; 2],
) {
    let leading = archive
        .field_handle(state.source_fields()[0].index())
        .unwrap();
    let current = archive.sketch_reference_lookup_state(&leading).unwrap();
    assert_eq!(
        current.keys(),
        LookupKeys {
            configuration_object_key: expected[0],
            configuration_record_key: expected[1]
        }
    );
}

fn verify_lookup_projection(
    archive: &PartResolvedFeaturesArchive,
    state: &LookupState,
    original: [i32; 2],
) {
    assert_eq!(
        state.keys(),
        LookupKeys {
            configuration_object_key: original[0],
            configuration_record_key: original[1]
        }
    );
    let rebound = archive
        .sketch_reference_lookup_state(&state.source_fields()[0])
        .unwrap();
    assert_eq!(rebound, *state);
}

fn append_lookup_export(output: &mut Option<Vec<u8>>, actual: &[u8]) {
    if let Some(output) = output {
        output.extend_from_slice(&u32::try_from(actual.len()).unwrap().to_le_bytes());
        output.extend_from_slice(actual);
    }
}
