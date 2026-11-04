use super::*;
use crate::unselected_document_streams::assert_unselected_document_streams;
use bessemer::Document;

struct NativeEllipseCase {
    index: usize,
    version: u32,
    path: &'static str,
    configuration: &'static str,
    bytes: usize,
    owners: usize,
}

struct NativeEllipseSpan {
    map_index: u32,
    offset: usize,
    snapshot: i32,
}

fn native_ellipse_cases() -> Vec<NativeEllipseCase> {
    include_str!("../fixtures/ellipse-backup-rotation/native-cases.tsv")
        .lines()
        .map(native_ellipse_case)
        .collect()
}

fn native_ellipse_case(line: &'static str) -> NativeEllipseCase {
    let columns: Vec<_> = line.split('\t').collect();
    assert_eq!(columns.len(), 6);
    NativeEllipseCase {
        index: columns[0].parse().unwrap(),
        version: columns[1].parse().unwrap(),
        path: columns[2],
        configuration: columns[3],
        bytes: columns[4].parse().unwrap(),
        owners: columns[5].parse().unwrap(),
    }
}

fn native_ellipse_spans(index: usize) -> Vec<NativeEllipseSpan> {
    include_str!("../fixtures/ellipse-backup-rotation/native-spans.tsv")
        .lines()
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .filter(|columns| columns[0].parse::<usize>().unwrap() == index)
        .enumerate()
        .map(|(ordinal, columns)| {
            assert_eq!(columns[1].parse::<usize>().unwrap(), ordinal);
            assert_eq!(columns[4], "1");
            NativeEllipseSpan {
                map_index: columns[2].parse().unwrap(),
                offset: columns[3].parse().unwrap(),
                snapshot: columns[5].parse().unwrap(),
            }
        })
        .collect()
}

fn verify_native_ellipse_projection(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    spans: &[NativeEllipseSpan],
) {
    let states = source.ellipse_backup_rotation_states().unwrap();
    assert_eq!(states.len(), spans.len());
    for (state, span) in states.iter().zip(spans) {
        assert_eq!(state.rotation_direction_snapshot(), span.snapshot);
        assert_eq!(
            &raw[span.offset..span.offset + 4],
            &span.snapshot.to_le_bytes()
        );
        verify_native_ellipse_identity(source, state, span);
    }
    assert_eq!(source.to_bytes().unwrap(), raw);
}

fn verify_native_ellipse_identity(
    source: &PartResolvedFeaturesArchive,
    state: &ModelEllipseBackupRotationState,
    span: &NativeEllipseSpan,
) {
    let ModelFieldValue::ObjectTag(tag) = &source.resolve_field(state.owner_field()).unwrap().value
    else {
        panic!("expected ellipse tag")
    };
    let ModelObjectRecord::New { index, class, .. } = &tag.record else {
        panic!("expected physical ellipse")
    };
    assert_eq!(index.value(), span.map_index);
    assert_eq!(
        (class.name.as_str(), class.schema),
        ("moEllipseBackedUpData_c", 1)
    );
}

fn verify_native_ellipse_request(
    document: &Document,
    case: &NativeEllipseCase,
    source: &PartResolvedFeaturesArchive,
    owner: usize,
    span: &NativeEllipseSpan,
    snapshot: i32,
) {
    let states = source.ellipse_backup_rotation_states().unwrap();
    let state = &states[owner];
    let edit = source
        .edit_ellipse_backup_rotation(state, snapshot)
        .unwrap();
    let raw = source.to_bytes().unwrap();
    assert_eq!(
        edit.archive().to_bytes().unwrap(),
        patch_ellipse_word(&raw, span.offset, snapshot)
    );
    let mut fields = source.fields().to_vec();
    fields[state.source_field().index()].value = ModelFieldValue::I32(snapshot);
    assert_eq!(edit.archive().fields(), fields);
    contracts::verify_ellipse_result(source, state, &edit, snapshot);
    verify_native_ellipse_document(document, case, &edit, owner, span.snapshot);
    assert_eq!(source.to_bytes().unwrap(), raw);
}

fn verify_native_ellipse_document(
    original: &Document,
    case: &NativeEllipseCase,
    edit: &PartResolvedFeaturesEdit,
    owner: usize,
    snapshot: i32,
) {
    let stream = format!("{}-ResolvedFeatures", case.configuration);
    let mut output = original.clone();
    output
        .replace_stream(&stream, edit.archive().to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    assert_unselected_document_streams(original, &reopened, &stream);
    let fresh =
        historical::native_ellipse_from_document(&reopened, case.configuration, case.version);
    let states = fresh.ellipse_backup_rotation_states().unwrap();
    let reverse = fresh
        .edit_ellipse_backup_rotation(&states[owner], snapshot)
        .unwrap();
    output
        .replace_stream(&stream, reverse.archive().to_bytes().unwrap())
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), original.to_bytes().unwrap());
}

fn verify_native_ellipse_case(case: &NativeEllipseCase) -> usize {
    let bytes = std::fs::read(case.path).unwrap();
    let document = Document::from_bytes(&bytes).unwrap();
    let stream = format!("{}-ResolvedFeatures", case.configuration);
    let raw = document.read_stream(&stream).unwrap();
    let source =
        historical::native_ellipse_from_document(&document, case.configuration, case.version);
    let spans = native_ellipse_spans(case.index);
    assert_eq!(raw.len(), case.bytes);
    assert_eq!(spans.len(), case.owners);
    verify_native_ellipse_projection(&source, &raw, &spans);
    for (owner, span) in spans.iter().enumerate() {
        for snapshot in REQUESTS {
            verify_native_ellipse_request(&document, case, &source, owner, span, snapshot);
        }
    }
    assert_eq!(document.to_bytes().unwrap(), bytes);
    spans.len() * 5
}

#[test]
#[ignore = "Requires five retained original native files and installed kernel schema"]
fn all_native_ellipse_words_match_source_spans_edit_reopen_and_reverse_exact_documents() {
    let cases = native_ellipse_cases();
    assert_eq!(cases.len(), 5);
    let mut operations = 0;
    for case in cases {
        let completed = verify_native_ellipse_case(&case);
        println!("ELLIPSE_NATIVE_DOCUMENT\t{}\t{}", case.index, completed);
        operations += completed;
    }
    assert_eq!(operations, 60);
}
