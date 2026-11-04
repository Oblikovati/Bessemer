//! Complete Source-owned intersection references and isolated stored-role operations (#184).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;
#[path = "model_intersection_reference/concurrency.rs"]
mod concurrency;
#[path = "model_intersection_reference/contracts.rs"]
mod contracts;
#[path = "model_intersection_reference/documents.rs"]
mod documents;
#[path = "support/model_operation_snapshot.rs"]
mod model_operation_snapshot;
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;
#[path = "support/visual_workers.rs"]
mod visual_workers;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const CONTROL: &[u8] = include_bytes!("fixtures/intersection-reference-match/controls.bin");
const MATRIX: &[u8] = include_bytes!("fixtures/intersection-reference-match/matrix.bin");
const WORDS: &str = include_str!("fixtures/intersection-reference-match/matrix-words.tsv");

fn intersection_context() -> (PartConfigurationArchive, Vec<u8>) {
    let document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let source = PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap();
    (source, document.read_stream("Contents/CMgr").unwrap())
}

fn intersection_archive(
    raw: &[u8],
    source: &PartConfigurationArchive,
    manager: &[u8],
) -> PartResolvedFeaturesArchive {
    let archive =
        PartResolvedFeaturesArchive::from_bytes(raw, source.clone(), manager, LIMITS).unwrap();
    assert_eq!(archive.to_bytes().unwrap(), raw);
    archive
}

fn next_intersection_image<'a>(cursor: &mut &'a [u8]) -> &'a [u8] {
    let length = u32::from_le_bytes(cursor[..4].try_into().unwrap()) as usize;
    let raw = &cursor[4..4 + length];
    *cursor = &cursor[4 + length..];
    raw
}

fn append_intersection_export(output: &mut Option<Vec<u8>>, bytes: &[u8]) {
    if let Some(output) = output {
        output.extend_from_slice(&u32::try_from(bytes.len()).unwrap().to_le_bytes());
        output.extend_from_slice(bytes);
    }
}

fn write_intersection_export(path: Option<std::ffi::OsString>, output: Option<Vec<u8>>) {
    if let (Some(path), Some(output)) = (path, output) {
        std::fs::write(path, output).unwrap();
    }
}

#[test]
fn complete_intersection_reference_control_preserves_roles_and_history() {
    let (context, manager) = intersection_context();
    let source = intersection_archive(CONTROL, &context, &manager);
    let rows: Vec<_> = WORDS.lines().take(8).collect();
    assert_eq!(source.intersection_reference_states().unwrap().len(), 8);
    contracts::verify_reference_case(&source, CONTROL, &rows, &mut None, false);
}

#[test]
fn complete_intersection_reference_matrix_matches_every_owned_word_and_edit() {
    let output_path = std::env::var_os("BESSEMER_INTERSECTION_EDIT_EXPORT");
    let mut output = output_path
        .as_ref()
        .map(|_| 1344_u32.to_le_bytes().to_vec());
    let (context, manager) = intersection_context();
    let original = context.to_bytes().unwrap();
    let mut cursor = MATRIX;
    let rows: Vec<_> = WORDS.lines().collect();
    assert_eq!(rows.len(), 10752);
    for (index, words) in rows.as_chunks::<8>().0.iter().enumerate() {
        let raw = next_intersection_image(&mut cursor);
        let source = intersection_archive(raw, &context, &manager);
        append_intersection_export(&mut output, raw);
        contracts::verify_reference_case(&source, raw, words, &mut output, true);
        println!("INTERSECTION_TYPED_MATRIX\t{index}\t8\t32");
    }
    assert!(cursor.is_empty());
    assert_eq!(context.to_bytes().unwrap(), original);
    write_intersection_export(output_path, output);
}

#[test]
fn complete_empty_intersection_enumeration_is_successful() {
    let (context, manager) = intersection_context();
    let mut empty = (-71_i32).to_le_bytes().to_vec();
    empty.extend_from_slice(&[0; 6]);
    let source = intersection_archive(&empty, &context, &manager);
    assert!(source.intersection_reference_states().unwrap().is_empty());
    assert_eq!(source.root_count(), 0);
}

#[test]
fn foreign_intersection_revisions_fail_without_mutating_either_input() {
    let (context, manager) = intersection_context();
    let first = intersection_archive(CONTROL, &context, &manager);
    let other = intersection_archive(CONTROL, &context, &manager);
    let state = first.intersection_reference_states().unwrap().remove(0);
    assert!(
        other
            .edit_intersection_reference_state(
                &state,
                ModelIntersectionReferenceStoredEdit::SpanCountState(0)
            )
            .is_err()
    );
    assert!(
        other
            .intersection_reference_state(&state.source_fields()[0])
            .is_err()
    );
    assert_eq!(other.to_bytes().unwrap(), CONTROL);
    assert_eq!(first.to_bytes().unwrap(), CONTROL);
}
