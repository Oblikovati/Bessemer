//! C-3341 saved appearance roles against independently built literal roots (#189).
#[path = "model_bitmap_appearance/concurrency.rs"]
mod concurrency;
#[path = "model_bitmap_appearance/documents.rs"]
mod documents;
#[path = "model_bitmap_appearance/exports.rs"]
mod exports;
#[path = "support/model_operation_snapshot.rs"]
mod model_operation_snapshot;
#[path = "model_bitmap_appearance/native.rs"]
mod native;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;
#[path = "support/visual_workers.rs"]
mod visual_workers;
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;

type Appearance = ModelBitmapAppearanceState;
type AppearanceEdit = ModelBitmapAppearanceEdit;
const ROOTS: &[u8] = include_bytes!("fixtures/bitmap-appearance/roots.bin");
const SCALARS: &str = include_str!("fixtures/bitmap-appearance/scalars.tsv");
const REQUESTS: [i32; 5] = [i32::MIN, -1, 0, 1, i32::MAX];
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 64 * 1024 * 1024,
    maximum_fields: 2_000_000,
    maximum_entries: 2_000_000,
    maximum_depth: 128,
    maximum_objects: 100_000,
};
#[path = "support/native_kernel_context.rs"]
mod native_kernel_context;

struct AppearanceCase<'a> {
    version: u32,
    raw: &'a [u8],
}

fn appearance_cases() -> Vec<AppearanceCase<'static>> {
    assert_eq!(&ROOTS[..4], b"BAP1");
    assert_eq!(u32::from_le_bytes(ROOTS[4..8].try_into().unwrap()), 825);
    let mut cursor = &ROOTS[8..];
    let mut cases = Vec::new();
    while !cursor.is_empty() {
        let version = u32::from_le_bytes(cursor[..4].try_into().unwrap());
        let length = u32::from_le_bytes(cursor[4..8].try_into().unwrap()) as usize;
        cases.push(AppearanceCase {
            version,
            raw: &cursor[8..8 + length],
        });
        cursor = &cursor[8 + length..];
    }
    assert_eq!(cases.len(), 825);
    cases
}

fn appearance_context(version: u32) -> (PartConfigurationArchive, Vec<u8>) {
    if version != 19000 {
        return native_appearance_context(version);
    }
    let document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let configuration = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(version),
        None,
        LIMITS,
    )
    .unwrap();
    (
        configuration,
        document.read_stream("Contents/CMgr").unwrap(),
    )
}

fn native_appearance_context(version: u32) -> (PartConfigurationArchive, Vec<u8>) {
    let columns = native_appearance_columns(version);
    let document = Document::from_bytes(&std::fs::read(columns[1]).unwrap()).unwrap();
    let identity = ConfigurationId::from_stream_name(columns[2]).unwrap();
    let kernel = native_kernel_context::native_test_kernel_context();
    let configuration = PartConfigurationArchive::from_document(
        &document,
        identity,
        ArchiveVersion::new(version),
        Some(kernel),
        LIMITS,
    )
    .unwrap();
    (
        configuration,
        document.read_stream("Contents/CMgr").unwrap(),
    )
}

fn appearance_archive(case: &AppearanceCase<'_>) -> PartResolvedFeaturesArchive {
    let (configuration, manager) = appearance_context(case.version);
    let source =
        PartResolvedFeaturesArchive::from_bytes(case.raw, configuration, &manager, LIMITS).unwrap();
    assert_eq!(source.to_bytes().unwrap(), case.raw);
    source
}

fn appearance_oracle(
    case: usize,
    owner: usize,
) -> ([Option<usize>; 3], ModelBitmapAppearanceWords) {
    let row: Vec<_> = SCALARS
        .lines()
        .nth(case * 2 + owner)
        .unwrap()
        .split('\t')
        .collect();
    assert_eq!(row[0].parse::<usize>().unwrap(), case);
    assert_eq!(row[2].parse::<usize>().unwrap(), owner);
    let offsets = appearance_offsets(&row);
    let words = appearance_expected_words(&row);
    (offsets, words)
}

fn appearance_request(role: usize, bits: i32) -> (AppearanceEdit, ModelFieldValue) {
    match role {
        0 => (
            AppearanceEdit::MatchingColorPercentage(bits),
            ModelFieldValue::I32(bits),
        ),
        1 => (
            AppearanceEdit::TransparentColor(bits as u32),
            ModelFieldValue::U32(bits as u32),
        ),
        2 => (
            AppearanceEdit::InkMarkupTexture(bits),
            ModelFieldValue::I32(bits),
        ),
        _ => panic!("appearance role {role}; expected 0..3"),
    }
}

fn verify_appearance_history(
    source: &PartResolvedFeaturesArchive,
    state: &Appearance,
    role: usize,
    result: &PartResolvedFeaturesEdit,
    expected: ModelFieldValue,
) {
    let parent = &state.source_fields()[role];
    let original = source.resolve_field(parent).unwrap();
    let count = usize::from(original.value != expected);
    assert_eq!(result.changes().len(), count);
    for change in result.changes() {
        verify_appearance_relation(source, result, parent, change, &expected);
    }
    assert_eq!(
        result.completion(),
        if count == 0 {
            ModelEditCompletion::Unchanged
        } else {
            ModelEditCompletion::Changed
        }
    );
    verify_appearance_coverage(source, result);
}

fn verify_appearance_revision(
    source: &PartResolvedFeaturesArchive,
    state: &Appearance,
    result: &PartResolvedFeaturesEdit,
    request: AppearanceEdit,
) {
    if result.completion() == ModelEditCompletion::Unchanged {
        for field in state.source_fields() {
            assert_eq!(
                result.archive().resolve_field(field).unwrap(),
                source.resolve_field(field).unwrap()
            );
        }
        return;
    }
    let expected = result
        .archive()
        .resolve_field(&state.source_fields()[0])
        .unwrap_err();
    assert_eq!(
        result
            .archive()
            .edit_bitmap_appearance(state, request)
            .unwrap_err(),
        expected
    );
}

fn verify_appearance_edit(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &Appearance,
    role: usize,
    offset: usize,
    bits: i32,
) -> Vec<u8> {
    let (request, _) = appearance_request(role, bits);
    let result = source.edit_bitmap_appearance(state, request).unwrap();
    verify_appearance_result(source, raw, state, role, offset, bits, &result);
    result.archive().to_bytes().unwrap()
}

fn verify_appearance_result(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &Appearance,
    role: usize,
    offset: usize,
    bits: i32,
    result: &PartResolvedFeaturesEdit,
) {
    let (request, expected) = appearance_request(role, bits);
    let mut literal = raw.to_vec();
    literal[offset..offset + 4].copy_from_slice(&bits.to_le_bytes());
    assert_eq!(result.archive().to_bytes().unwrap(), literal);
    let mut fields = source.fields().to_vec();
    fields[state.source_fields()[role].index()].value = expected.clone();
    assert_eq!(result.archive().fields(), fields);
    verify_appearance_history(source, state, role, result, expected);
    verify_appearance_revision(source, state, result, request);
    assert_eq!(source.to_bytes().unwrap(), raw);
}

fn verify_appearance_owner(
    index: usize,
    owner: usize,
    source: &PartResolvedFeaturesArchive,
    case: &AppearanceCase<'_>,
    output: &mut Option<Vec<u8>>,
) -> usize {
    let states = source.bitmap_appearance_states().unwrap();
    assert_eq!(states.len(), 2);
    let state = &states[owner];
    let (offsets, expected) = appearance_oracle(index, owner);
    verify_appearance_state(state, expected);
    for (role, offset) in offsets
        .into_iter()
        .enumerate()
        .filter_map(|(role, offset)| offset.map(|offset| (role, offset)))
    {
        for bits in REQUESTS {
            let actual = verify_appearance_edit(source, case.raw, state, role, offset, bits);
            exports::append_appearance_export(output, case.version, &actual);
        }
    }
    state.source_fields().len() * REQUESTS.len()
}

#[test]
#[ignore = "Requires explicit installed kernel schema and pinned11000/18000 native contexts"]
fn all_appearance_words_and_isolated_edits_match_independent_complete_roots() {
    let (path, mut output) = exports::appearance_export();
    let mut edits = 0;
    for (index, case) in appearance_cases().into_iter().enumerate() {
        let source = appearance_archive(&case);
        for owner in 0..2 {
            edits += verify_appearance_owner(index, owner, &source, &case, &mut output);
        }
    }
    assert_eq!(edits, 24000);
    exports::finish_appearance_export(path, output);
}

#[test]
fn owned_current_appearance_edits_match_all375_literal_roots() {
    let mut edits = 0;
    for (index, case) in appearance_cases()
        .into_iter()
        .enumerate()
        .filter(|(_, case)| case.version == 19000)
    {
        let source = appearance_archive(&case);
        for owner in 0..2 {
            edits += verify_appearance_owner(index, owner, &source, &case, &mut None);
        }
    }
    assert_eq!(edits, 11250);
}

#[test]
#[ignore = "Requires explicit installed kernel schema and pinned11000/18000 native contexts"]
fn absent_ink_fields_and_foreign_states_return_typed_errors_without_mutation() {
    for case in appearance_cases() {
        let source = appearance_archive(&case);
        let foreign = appearance_archive(&case);
        for state in source.bitmap_appearance_states().unwrap() {
            verify_appearance_rejections(&source, &foreign, &state, case.version);
        }
        assert_eq!(source.to_bytes().unwrap(), case.raw);
        assert_eq!(foreign.to_bytes().unwrap(), case.raw);
    }
}

#[test]
fn canonical_appearance_schema_errors_are_not_silently_empty_and_types_are_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ModelBitmapAppearanceWords>();
    send_sync::<AppearanceEdit>();
    send_sync::<Appearance>();
    let cases = appearance_cases();
    let source = appearance_archive(cases.iter().find(|case| case.version == 19000).unwrap());
    let original = source.to_bytes().unwrap();
    let mut raw = original.clone();
    raw[8..10].copy_from_slice(&2_u16.to_le_bytes());
    let case = AppearanceCase {
        version: 19000,
        raw: &raw,
    };
    let altered = appearance_archive(&case);
    assert!(altered.bitmap_appearance_states().is_err());
    assert_eq!(source.to_bytes().unwrap(), original);
}

fn appearance_offsets(row: &[&str]) -> [Option<usize>; 3] {
    [
        Some(row[3].parse().unwrap()),
        Some(row[4].parse().unwrap()),
        if row[5] == "-1" {
            None
        } else {
            Some(row[5].parse().unwrap())
        },
    ]
}

fn appearance_expected_words(row: &[&str]) -> ModelBitmapAppearanceWords {
    ModelBitmapAppearanceWords {
        matching_color_percentage: row[6].parse().unwrap(),
        transparent_color: row[7].parse().unwrap(),
        ink_markup_texture: if row[8] == "none" {
            None
        } else {
            Some(row[8].parse().unwrap())
        },
    }
}

fn verify_appearance_relation(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    parent: &ModelFieldHandle,
    change: &ModelFieldChange,
    expected: &ModelFieldValue,
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
    assert_eq!(&selected.value, expected);
}

fn verify_appearance_coverage(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
) {
    assert_eq!(
        result.coverage().mode,
        ModelEditCheckMode::ExactStoredArchive
    );
    assert_eq!(result.coverage().typed_fields, source.fields().len());
    assert_eq!(
        result.coverage().wire_bytes,
        source.to_bytes().unwrap().len()
    );
    assert_eq!(result.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(result.diagnostics(), source.diagnostics());
}

fn verify_appearance_state(state: &Appearance, expected: ModelBitmapAppearanceWords) {
    assert_eq!(state.words(), expected);
    assert_eq!(
        state.source_fields().len(),
        if expected.ink_markup_texture.is_some() {
            3
        } else {
            2
        }
    );
}

fn verify_appearance_rejections(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    state: &Appearance,
    version: u32,
) {
    let expected = foreign
        .resolve_field(&state.source_fields()[0])
        .unwrap_err();
    assert_eq!(
        foreign
            .edit_bitmap_appearance(state, AppearanceEdit::TransparentColor(7))
            .unwrap_err(),
        expected
    );
    if version < 0x2bd3 {
        let error = source
            .edit_bitmap_appearance(state, AppearanceEdit::InkMarkupTexture(-1))
            .unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains("InkMarkupTexture(-1)"));
        assert!(error.cause.contains("stored raw80"));
    }
}

fn native_appearance_columns(version: u32) -> Vec<&'static str> {
    let row = include_str!("fixtures/bitmap-appearance/native-contexts.tsv")
        .lines()
        .find(|line| line.split('\t').next().unwrap().parse::<u32>().unwrap() == version)
        .unwrap();
    let columns: Vec<_> = row.split('\t').collect();
    assert_eq!(columns.len(), 5);
    columns
}
