//! Complete drawing bucket streams use model context and direct per-view collections.
use bessemer::display_lists::*;
use bessemer::{Document, header::ArchiveVersion};
#[path = "../examples/support/archive_scalar_trace.rs"]
mod archive_scalar_trace;
#[path = "../examples/support/drawing_bucket_trace.rs"]
mod drawing_bucket_trace;
#[path = "../examples/support/framing.rs"]
mod framing;

const LIMITS: DisplayArchiveLimits = DisplayArchiveLimits {
    maximum_bytes: 1024 * 1024,
    maximum_fields: 10000,
    maximum_objects: 1000,
    maximum_depth: 128,
    maximum_entries: 10000,
};
const VERSION: ArchiveVersion = ArchiveVersion::new(19000);

fn frames(mut raw: &'static [u8]) -> Vec<&'static [u8]> {
    let count = framing::take_u32(&mut raw).unwrap();
    framing::require_count(count, raw.len(), 4).unwrap();
    let mut records = Vec::new();
    for _ in 0..count {
        let size = framing::take_u32(&mut raw).unwrap() as usize;
        records.push(framing::take(&mut raw, size).unwrap());
    }
    framing::require_finished(raw).unwrap();
    records
}

fn cases() -> Vec<&'static [u8]> {
    frames(include_bytes!("fixtures/drawing-buckets/vectors.bin"))
}

fn parsed(raw: &[u8]) -> DrawingBucketArchive {
    DrawingBucketArchive::from_bytes(raw, VERSION, LIMITS).unwrap()
}

#[test]
fn owned_streams_match_independent_fields_and_forced_bytes() {
    let expected = frames(include_bytes!("fixtures/drawing-buckets/fields.bin"));
    assert_eq!(cases().len(), 5);
    let mut count = 0;
    for (raw, trace) in cases().into_iter().zip(expected) {
        let archive = parsed(raw);
        assert_eq!(drawing_bucket_trace::trace(&archive), trace);
        count += trace.iter().filter(|byte| **byte == b'\n').count();
        let forced =
            DrawingBucketArchive::new(archive.contents().clone(), VERSION, LIMITS).unwrap();
        assert_eq!(forced.to_bytes().unwrap(), raw);
        assert_eq!(archive.model_version(), VERSION);
    }
    assert_eq!(count, 139);
}

fn field<'a>(archive: &'a DrawingBucketArchive, path: &str) -> &'a DisplayFieldValue {
    &archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value
}

#[test]
fn one_object_map_preserves_top_level_aliases_nulls_and_nested_self_references() {
    let archive = parsed(cases()[2]);
    for (path, target) in [
        (
            "root.view_buckets[0]",
            DisplayObjectPointer::New(DisplayObjectId::new(2).unwrap()),
        ),
        (
            "root.view_buckets[1]",
            DisplayObjectPointer::Reference(DisplayObjectId::new(2).unwrap()),
        ),
        ("root.view_buckets[2]", DisplayObjectPointer::Null),
        (
            "root.view_buckets[0].sub_buckets[0]",
            DisplayObjectPointer::Reference(DisplayObjectId::new(2).unwrap()),
        ),
    ] {
        let DisplayFieldValue::ObjectTag(tag) = field(&archive, path) else {
            panic!("expected pointer")
        };
        assert_eq!(tag.target, target);
    }
}

#[test]
fn drawing_backend_omits_edge_points_without_discarding_the_rest_of_each_facet() {
    let archive = parsed(cases()[2]);
    let facets: Vec<_> = archive
        .contents()
        .fields
        .iter()
        .filter_map(|field| {
            if let DisplayFieldValue::Facet(facet) = &field.value {
                return Some(facet);
            }
            None
        })
        .collect();
    assert_eq!(facets.len(), 2);
    for facet in facets {
        assert!(facet.edge_points.is_none());
        assert!(facet.edges.is_some());
        assert!(facet.late_stamps.is_some());
    }
}

fn replace_edited_fields(contents: &mut DisplayArchiveContents, expected: &DrawingBucketArchive) {
    for field in &mut contents.fields {
        let path = field.path.as_ref();
        if path == "root.models[0].model_reference" || path.ends_with("[0].words[1]") {
            field.value = self::field(expected, path).clone();
        }
    }
}

fn edited_archive(source: &DrawingBucketArchive) -> DrawingBucketArchive {
    let expected = parsed(cases()[3]);
    let mut contents = source.contents().clone();
    replace_edited_fields(&mut contents, &expected);
    source.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn edits_match_independent_bytes_isolate_source_and_authentically_revert() {
    let source = parsed(cases()[2]);
    let edited = edited_archive(&source);
    assert_eq!(edited.to_bytes().unwrap(), cases()[3]);
    assert_eq!(source.to_bytes().unwrap(), cases()[2]);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), cases()[2]);
}

fn mutate_integer(
    archive: &DrawingBucketArchive,
    path: &str,
    value: i32,
) -> DisplayArchiveContents {
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    field.value = DisplayFieldValue::I32(value);
    contents
}

#[test]
fn invalid_kinds_counts_and_reordered_collection_fields_cannot_be_published() {
    let archive = parsed(cases()[2]);
    for (path, value) in [
        ("root.models[0].is_part_word", 2),
        ("root.model_count", -1),
        ("root.string_lists_28[0].count", i32::MAX),
        ("root.lists_f8[0].count", -1),
        ("root.view_bucket_count", i32::MAX),
    ] {
        assert!(
            archive
                .with_contents(mutate_integer(&archive, path, value), LIMITS)
                .is_err()
        );
    }
    let mut reordered = archive.contents().clone();
    reordered.fields.swap(1, 2);
    assert!(archive.with_contents(reordered, LIMITS).is_err());
    assert_eq!(archive.to_bytes().unwrap(), cases()[2]);
}

#[test]
fn every_truncation_and_trailing_bytes_fail_without_a_partial_result() {
    for raw in cases() {
        for length in 0..raw.len() {
            assert!(DrawingBucketArchive::from_bytes(&raw[..length], VERSION, LIMITS).is_err());
        }
        let mut trailing = raw.to_vec();
        trailing.push(0);
        assert!(DrawingBucketArchive::from_bytes(&trailing, VERSION, LIMITS).is_err());
    }
}

#[test]
fn limits_apply_to_reads_construction_and_equivalent_preserved_edits() {
    for raw in cases() {
        check_short_budget(raw);
        let no_fields = DisplayArchiveLimits {
            maximum_fields: 1,
            ..LIMITS
        };
        assert!(DrawingBucketArchive::from_bytes(raw, VERSION, no_fields).is_err());
    }
    let no_entries = DisplayArchiveLimits {
        maximum_entries: 0,
        ..LIMITS
    };
    assert!(DrawingBucketArchive::from_bytes(cases()[2], VERSION, no_entries).is_err());
}

fn check_short_budget(raw: &[u8]) {
    let archive = parsed(raw);
    let short = DisplayArchiveLimits {
        maximum_bytes: raw.len() - 1,
        ..LIMITS
    };
    assert!(DrawingBucketArchive::from_bytes(raw, VERSION, short).is_err());
    assert!(DrawingBucketArchive::new(archive.contents().clone(), VERSION, short).is_err());
    assert!(
        archive
            .with_contents(archive.contents().clone(), short)
            .is_err()
    );
}

#[test]
fn drawing_model_version_is_required_and_display_list_layout_is_not_inferred() {
    for version in [0, 0x27cb, 19001, u32::MAX] {
        assert!(
            DrawingBucketArchive::from_bytes(cases()[0], ArchiveVersion::new(version), LIMITS)
                .is_err()
        );
    }
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(19000),
        kind: DisplayDocumentKind::Part,
        file_version: 19000,
        face_tessellation_components: Default::default(),
    };
    assert!(DisplayArchive::from_bytes(cases()[2], context, LIMITS).is_err());
}

#[test]
fn complete_modern_range_matches_independent_models_and_backend_boundary() {
    let roots = frames(include_bytes!("fixtures/drawing-buckets/range.bin"));
    let fields = frames(include_bytes!("fixtures/drawing-buckets/range-fields.bin"));
    let versions = include_bytes!("fixtures/drawing-buckets/range-versions.bin");
    assert_eq!(roots.len(), 34);
    for ((raw, expected), version) in roots
        .into_iter()
        .zip(fields)
        .zip(versions.as_chunks::<4>().0)
    {
        let version = ArchiveVersion::new(u32::from_le_bytes(*version));
        let archive = DrawingBucketArchive::from_bytes(raw, version, LIMITS).unwrap();
        assert_eq!(drawing_bucket_trace::trace(&archive), expected);
        let forced =
            DrawingBucketArchive::new(archive.contents().clone(), version, LIMITS).unwrap();
        assert_eq!(forced.to_bytes().unwrap(), raw);
        check_range_facet(&archive, version.value());
    }
}

fn check_range_facet(archive: &DrawingBucketArchive, version: u32) {
    let facets: Vec<_> = archive
        .contents()
        .fields
        .iter()
        .filter_map(|field| {
            if let DisplayFieldValue::Facet(facet) = &field.value {
                return Some(facet);
            }
            None
        })
        .collect();
    assert_eq!(facets.len(), 1);
    assert_eq!(
        facets[0].edge_points.is_some(),
        (0x2bd1..0x2be5).contains(&version)
    );
}

#[test]
fn backend_boundary_rejects_the_other_facets_shape_without_mutating_source() {
    let roots = frames(include_bytes!("fixtures/drawing-buckets/range.bin"));
    let versions = include_bytes!("fixtures/drawing-buckets/range-versions.bin");
    for (raw, version) in roots.into_iter().zip(versions.as_chunks::<4>().0) {
        let value = u32::from_le_bytes(*version);
        if ![0x2be4, 0x2be5].contains(&value) {
            continue;
        }
        let source =
            DrawingBucketArchive::from_bytes(raw, ArchiveVersion::new(value), LIMITS).unwrap();
        let opposite = ArchiveVersion::new(if value == 0x2be4 { 0x2be5 } else { 0x2be4 });
        assert!(DrawingBucketArchive::new(source.contents().clone(), opposite, LIMITS).is_err());
        assert_eq!(source.to_bytes().unwrap(), raw);
    }
}

#[test]
fn failures_report_the_drawing_stream_and_preserve_typed_failure_classes() {
    let unsupported =
        DrawingBucketArchive::from_bytes(cases()[0], ArchiveVersion::new(0), LIMITS).unwrap_err();
    assert_eq!(unsupported.class, bessemer::FailureClass::Unsupported);
    assert_eq!(unsupported.operation, "drawing VBLists archive");
    assert_eq!(unsupported.entity, "Contents/VBLists");
    let invalid = parsed(cases()[2])
        .with_contents(DisplayArchiveContents { fields: Vec::new() }, LIMITS)
        .unwrap_err();
    assert_eq!(invalid.operation, "drawing VBLists archive");
    assert_eq!(invalid.entity, "Contents/VBLists");
    let document =
        Document::from_bytes(include_bytes!("fixtures/drawing-buckets/missing.bin")).unwrap();
    let missing = DrawingBucketArchive::from_document(&document, LIMITS).unwrap_err();
    assert_eq!(missing.class, bessemer::FailureClass::Missing);
}

#[test]
fn published_drawing_archives_are_send_sync_and_concurrent_edits_are_deterministic() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<DrawingBucketArchive>();
    let source = parsed(cases()[2]);
    let threads: Vec<_> = (0..8)
        .map(|_| {
            let source = source.clone();
            std::thread::spawn(move || edited_archive(&source).to_bytes().unwrap())
        })
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), cases()[3]);
    }
    assert_eq!(source.to_bytes().unwrap(), cases()[2]);
}

#[test]
fn whole_document_edit_reopen_and_revert_use_model_markers_and_preserve_other_streams() {
    let mut document =
        Document::from_bytes(include_bytes!("fixtures/drawing-buckets/document.bin")).unwrap();
    let source = DrawingBucketArchive::from_document(&document, LIMITS).unwrap();
    let mut reopened = replace_reopen(&mut document, &edited_archive(&source));
    let edited = DrawingBucketArchive::from_document(&reopened, LIMITS).unwrap();
    assert_eq!(edited.to_bytes().unwrap(), cases()[3]);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    let reverted_document = replace_reopen(&mut reopened, &reverted);
    let reverted = DrawingBucketArchive::from_document(&reverted_document, LIMITS).unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), cases()[2]);
}

fn replace_reopen(document: &mut Document, archive: &DrawingBucketArchive) -> Document {
    document
        .replace_stream("Contents/VBLists", archive.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    assert_eq!(
        reopened.read_stream("Unrelated/Payload").unwrap(),
        b"retained\0payload"
    );
    reopened
}

#[test]
fn document_model_markers_reject_missing_and_conflicting_versions_and_accept_numeric_aliases() {
    for raw in [
        include_bytes!("fixtures/drawing-buckets/missing.bin").as_slice(),
        include_bytes!("fixtures/drawing-buckets/conflicting.bin").as_slice(),
    ] {
        let document = Document::from_bytes(raw).unwrap();
        assert!(DrawingBucketArchive::from_document(&document, LIMITS).is_err());
    }
    let document =
        Document::from_bytes(include_bytes!("fixtures/drawing-buckets/alias.bin")).unwrap();
    assert_eq!(
        DrawingBucketArchive::from_document(&document, LIMITS)
            .unwrap()
            .model_version(),
        VERSION
    );
}
