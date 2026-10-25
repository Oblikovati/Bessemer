//! Graphics references, fixed plane layout, object cycles and explicitly versioned optional fields.
use bessemer::display_lists::*;
use bessemer::header::StoredDouble;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "../examples/support/framing.rs"]
mod framing;
#[path = "../examples/support/graphics_trace.rs"]
mod graphics_trace;

#[derive(Clone, Copy)]
struct GraphicsCase {
    version: u32,
    kind: DisplayDocumentKind,
    scenario: u8,
    bytes: &'static [u8],
    expected: &'static [u8],
}

fn graphics_blob(remaining: &mut &'static [u8]) -> &'static [u8] {
    let length = framing::take_u32(remaining).unwrap() as usize;
    framing::take(remaining, length).unwrap()
}

fn read_case(remaining: &mut &'static [u8]) -> GraphicsCase {
    let version = framing::take_u32(remaining).unwrap();
    let kind = if framing::take(remaining, 1).unwrap()[0] == 0 {
        DisplayDocumentKind::Part
    } else {
        DisplayDocumentKind::Assembly
    };
    let scenario = framing::take(remaining, 1).unwrap()[0];
    let bytes = graphics_blob(remaining);
    let expected = graphics_blob(remaining);
    GraphicsCase {
        version,
        kind,
        scenario,
        bytes,
        expected,
    }
}

fn cases() -> Vec<GraphicsCase> {
    let mut remaining: &'static [u8] =
        include_bytes!("fixtures/display-lists/graphics-references.bin");
    let count = framing::take_u32(&mut remaining).unwrap();
    framing::require_count(count, remaining.len(), 14).unwrap();
    let cases = (0..count).map(|_| read_case(&mut remaining)).collect();
    framing::require_finished(remaining).unwrap();
    cases
}

fn selected(kind: DisplayDocumentKind, scenario: u8) -> GraphicsCase {
    cases()
        .into_iter()
        .find(|case| case.version == 19000 && case.kind == kind && case.scenario == scenario)
        .unwrap()
}

fn case_context(case: GraphicsCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..context()
    }
}

fn parsed(case: GraphicsCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

#[test]
fn all_graphics_reference_classes_match_independent_plane_and_scalar_fields_and_bytes() {
    assert_eq!(cases().len(), 5);
    for case in cases() {
        let archive = parsed(case);
        assert_eq!(graphics_trace::trace(&archive), case.expected);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
}

const MANAGER: &str = "root.tessellation.graphics_connect_ref_mgr_data";

fn plane_edit(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let path = format!("{MANAGER}.graphics_ground_face_data.unknown_18.root_point[0]");
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    assert!(matches!(field.value, DisplayFieldValue::F64(_)));
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn plane_edit_matches_independent_wire_and_preserves_original_document() {
    let case = selected(DisplayDocumentKind::Part, 0);
    let archive = parsed(case);
    let edited = plane_edit(&archive);
    assert_eq!(
        edited.to_bytes().unwrap(),
        selected(DisplayDocumentKind::Part, 1).bytes
    );
    assert_eq!(
        edited
            .with_contents(archive.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        case.bytes
    );
    let initial = include_bytes!("fixtures/blank-part.SLDPRT");
    let native = archive_support::decode_document(initial, DisplayDocumentKind::Part);
    let original = bessemer::Document::from_bytes(initial).unwrap();
    let mut document = original.clone();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    assert_eq!(
        archive_support::decode_document(&reopened.to_bytes().unwrap(), DisplayDocumentKind::Part)
            .contents(),
        edited.contents()
    );
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            reopened.read_stream(stream.name).unwrap()
        );
    }
    document
        .replace_stream("Contents/DisplayLists", native.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), initial);
}

#[test]
fn raw_presence_flags_empty_manager_and_versioned_magnetic_tail_are_retained() {
    let archive = parsed(selected(DisplayDocumentKind::Part, 0));
    let expected = format!(
        "{MANAGER}.has_graphics_ground_face_data=i:-17\n{MANAGER}.graphics_ground_face_data.has_unknown_18=i:-3\n{MANAGER}.all_graphics_connect_pnts.count=i:4\n"
    );
    assert_eq!(archive_support::check_scalar_table(&archive, &expected), 3);
    let empty = parsed(selected(DisplayDocumentKind::Part, 2));
    assert!(
        !empty
            .contents()
            .fields
            .iter()
            .any(|field| field.path.contains(".graphics_ground_face_data."))
    );
    let old = parsed(
        cases()
            .into_iter()
            .find(|case| case.version == 11000)
            .unwrap(),
    );
    assert!(
        !String::from_utf8(graphics_trace::trace(&old))
            .unwrap()
            .contains("unknown_28=")
    );
    assert!(
        String::from_utf8(graphics_trace::trace(&archive))
            .unwrap()
            .contains("unknown_28=f:8000000000000000")
    );
}

#[test]
fn malformed_counts_presence_changes_truncation_and_trailing_bytes_fail() {
    let case = selected(DisplayDocumentKind::Part, 0);
    let archive = parsed(case);
    for end in 0..case.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
            "prefix {end}"
        );
    }
    for count in [-1, i32::MAX] {
        let mut contents = archive.contents().clone();
        archive_support::set_integer(
            &mut contents,
            &format!("{MANAGER}.all_graphics_connect_pnts.count"),
            count,
        );
        assert!(archive.with_contents(contents, LIMITS).is_err());
    }
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        &format!("{MANAGER}.graphics_ground_face_data.has_unknown_18"),
        0,
    );
    assert!(archive.with_contents(contents, LIMITS).is_err());
    let mut trailing = case.bytes.to_vec();
    trailing.push(0);
    assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
}

#[test]
fn published_graphics_graph_owns_source_and_parallel_edits_are_identical() {
    let case = selected(DisplayDocumentKind::Part, 0);
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| plane_edit(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(
                worker.join().unwrap(),
                selected(DisplayDocumentKind::Part, 1).bytes
            );
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

fn object_target(archive: &DisplayArchive, suffix: &str) -> DisplayObjectPointer {
    let path = format!("{MANAGER}.{suffix}");
    let field = archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected object at {path}")
    };
    tag.target.clone()
}

#[test]
fn wrapper_cycle_magnetic_alias_and_null_reference_keep_shared_map_identity() {
    let archive = parsed(selected(DisplayDocumentKind::Part, 0));
    let DisplayObjectPointer::New(magnet) = object_target(&archive, "all_graphics_connect_pnts[0]")
    else {
        panic!("new magnetic")
    };
    assert_eq!(
        object_target(&archive, "all_graphics_connect_pnts[1]"),
        DisplayObjectPointer::Reference(magnet)
    );
    assert_eq!(
        object_target(&archive, "all_graphics_connect_pnts[2]"),
        DisplayObjectPointer::Null
    );
    let owner = object_target(&archive, "graphics_ground_face_data.unknown_50");
    assert_eq!(
        owner,
        object_target(&archive, "all_graphics_connect_pnts[0].unknown_18")
    );
    assert!(matches!(owner, DisplayObjectPointer::Reference(_)));
}

#[test]
fn exact_part_budgets_charge_manager_points_and_four_new_objects() {
    for scenario in [0, 2] {
        let case = selected(DisplayDocumentKind::Part, scenario);
        let archive = parsed(case);
        let entries = if scenario == 2 { 53 } else { 57 };
        let objects = if scenario == 2 { 47 } else { 50 };
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok());
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
        for limited in [
            DisplayArchiveLimits {
                maximum_entries: entries - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_objects: objects - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_fields: exact.maximum_fields - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_bytes: exact.maximum_bytes - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), limited).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limited)
                    .is_err()
            );
        }
    }
}

#[test]
fn unresolved_faceted_object_bodies_fail_explicitly_without_opaque_success() {
    let archive = parsed(selected(DisplayDocumentKind::Part, 0));
    let mut contents = archive.contents().clone();
    let path = format!("{MANAGER}.graphics_ground_face_data");
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &mut field.value else {
        panic!("object tag")
    };
    tag.runtime_class.as_mut().unwrap().name = "moFacetedData_c".into();
    tag.new_class.as_mut().unwrap().name = "moFacetedData_c".into();
    let error = archive.with_contents(contents, LIMITS).unwrap_err();
    assert!(error.to_string().contains("Unsupported"));
    assert!(error.to_string().contains("moFacetedData_c"));
}
