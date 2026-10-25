//! Document context comes from exact markers and parsed declarations with explicit native versions.
use bessemer::display_lists::*;
use bessemer::tessellation::{
    ArchiveVersion, FacetArchiveMode, TessellationContext, TessellationLimits,
};
use bessemer::{Document, FailureClass};
#[allow(
    dead_code,
    reason = "Shared archive helpers also serve graph and scalar tests."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::LIMITS;

const DIRECTORY: &str = "FaceTessellations/Directory";
const COMPONENTS: &[&str] = &["Assem1", "assem1", "cache 鋼"];
const TESS_LIMITS: TessellationLimits = TessellationLimits {
    maximum_components: 100,
    maximum_faces: 100,
    maximum_array_elements: 1000,
};

fn document(name: &str) -> Document {
    let bytes: &[u8] = match name {
        "part" => include_bytes!("fixtures/display-lists/context-inline-part.bin"),
        "assembly" => include_bytes!("fixtures/display-lists/context-inline-assembly.bin"),
        "modern" => include_bytes!("fixtures/display-lists/context-external-modern.bin"),
        "pair" => include_bytes!("fixtures/display-lists/context-external-pair.bin"),
        "empty" => include_bytes!("fixtures/display-lists/context-empty-directory.bin"),
        _ => panic!("unknown fixture {name}"),
    };
    Document::from_bytes(bytes).unwrap()
}

fn tessellation(version: u32) -> TessellationContext {
    TessellationContext {
        archive_version: ArchiveVersion::new(version),
        mode: FacetArchiveMode::Full,
    }
}

fn resolved(
    document: &Document,
    kind: DisplayDocumentKind,
    file_version: u32,
    directory_version: u32,
) -> DisplayArchiveContext {
    DisplayArchiveContext::from_document(
        document,
        kind,
        file_version,
        tessellation(directory_version),
        TESS_LIMITS,
        LIMITS.maximum_bytes,
    )
    .unwrap()
}

#[test]
fn complete_owned_documents_resolve_independent_contexts_and_force_exact_stream_bytes() {
    for (name, kind, file_version, directory_version, external) in [
        ("part", DisplayDocumentKind::Part, 0x223, 19000, false),
        (
            "assembly",
            DisplayDocumentKind::Assembly,
            0x224,
            19000,
            false,
        ),
        ("modern", DisplayDocumentKind::Assembly, 19000, 19000, true),
        (
            "pair",
            DisplayDocumentKind::Assembly,
            u32::MAX,
            0x3b1c,
            true,
        ),
        ("empty", DisplayDocumentKind::Assembly, 0, 19000, false),
    ] {
        let document = document(name);
        let context = resolved(&document, kind, file_version, directory_version);
        assert_eq!(context.version, DisplayVersion::new(19000));
        assert_eq!(context.kind, kind);
        assert_eq!(context.file_version, file_version);
        let expected: Vec<_> = if external {
            COMPONENTS.to_vec()
        } else {
            Vec::new()
        };
        assert_eq!(
            context
                .face_tessellation_components
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            expected
        );
        let bytes = document
            .read_stream_limited("Contents/DisplayLists", LIMITS.maximum_bytes)
            .unwrap();
        let archive = DisplayArchive::from_bytes(&bytes, context.clone(), LIMITS).unwrap();
        let references = archive.contents().fields.iter().filter(|field| matches!(&field.value, DisplayFieldValue::ExternalFacet {component_id,..} if component_id.text()=="Assem1")).count();
        assert_eq!(references, if external { 2 } else { 0 });
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), context, LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
}

#[test]
fn missing_invalid_conflicting_and_lookalike_markers_do_not_fall_back_to_a_default_version() {
    for (bytes, class) in [
        (
            include_bytes!("fixtures/display-lists/context-no-marker.bin").as_slice(),
            FailureClass::Missing,
        ),
        (
            include_bytes!("fixtures/display-lists/context-conflict.bin").as_slice(),
            FailureClass::Invalid,
        ),
        (
            include_bytes!("fixtures/display-lists/context-invalid-marker.bin").as_slice(),
            FailureClass::Invalid,
        ),
        (
            include_bytes!("fixtures/display-lists/context-lookalike.bin").as_slice(),
            FailureClass::Missing,
        ),
    ] {
        let document = Document::from_bytes(bytes).unwrap();
        let error = DisplayArchiveContext::from_document(
            &document,
            DisplayDocumentKind::Part,
            19000,
            tessellation(19000),
            TESS_LIMITS,
            0,
        )
        .unwrap_err();
        assert_eq!(error.class, class);
        assert!(error.entity.contains("DLUpdateStamp"));
    }
    let document = Document::from_bytes(include_bytes!(
        "fixtures/display-lists/context-signed-aliases.bin"
    ))
    .unwrap();
    assert_eq!(
        resolved(&document, DisplayDocumentKind::Part, 4, 19000).version,
        DisplayVersion::new(u32::MAX)
    );
}

#[test]
fn explicit_directory_version_and_limits_are_load_bearing() {
    let pair = document("pair");
    assert!(
        DisplayArchiveContext::from_document(
            &pair,
            DisplayDocumentKind::Assembly,
            19000,
            tessellation(19000),
            TESS_LIMITS,
            LIMITS.maximum_bytes
        )
        .is_err()
    );
    let length = pair.read_stream(DIRECTORY).unwrap().len();
    assert!(
        DisplayArchiveContext::from_document(
            &pair,
            DisplayDocumentKind::Assembly,
            19000,
            tessellation(0x3b1c),
            TESS_LIMITS,
            length
        )
        .is_ok()
    );
    assert!(
        DisplayArchiveContext::from_document(
            &pair,
            DisplayDocumentKind::Assembly,
            19000,
            tessellation(0x3b1c),
            TESS_LIMITS,
            length - 1
        )
        .is_err()
    );
    assert!(
        DisplayArchiveContext::from_document(
            &pair,
            DisplayDocumentKind::Assembly,
            19000,
            tessellation(0x3b1c),
            TessellationLimits {
                maximum_components: 3,
                ..TESS_LIMITS
            },
            length
        )
        .is_err()
    );
    let absent = document("part");
    assert!(
        DisplayArchiveContext::from_document(
            &absent,
            DisplayDocumentKind::Part,
            19000,
            tessellation(19000),
            TESS_LIMITS,
            0
        )
        .is_ok()
    );
}

#[test]
fn malformed_or_replaced_directories_are_never_silently_treated_as_absent() {
    let mut document = document("modern");
    let original = resolved(&document, DisplayDocumentKind::Assembly, 19000, 19000);
    document.replace_stream(DIRECTORY, vec![0; 12]).unwrap();
    assert!(
        DisplayArchiveContext::from_document(
            &document,
            DisplayDocumentKind::Assembly,
            19000,
            tessellation(19000),
            TESS_LIMITS,
            LIMITS.maximum_bytes
        )
        .is_err()
    );
    assert_eq!(original.face_tessellation_components.len(), 3);
    let empty = self::document("empty").read_stream(DIRECTORY).unwrap();
    document.replace_stream(DIRECTORY, empty).unwrap();
    assert!(
        resolved(&document, DisplayDocumentKind::Assembly, 19000, 19000)
            .face_tessellation_components
            .is_empty()
    );
    assert_eq!(original.face_tessellation_components.len(), 3);
}

#[test]
fn derived_component_sets_own_sources_and_concurrent_contexts_remain_isolated() {
    let mut source = include_bytes!("fixtures/display-lists/context-external-modern.bin").to_vec();
    let document = Document::from_bytes(&source).unwrap();
    source.fill(0);
    let expected = resolved(&document, DisplayDocumentKind::Assembly, 19000, 19000);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| resolved(&document, DisplayDocumentKind::Assembly, 19000, 19000))
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(expected.face_tessellation_components.len(), 3);
}

#[test]
fn directory_crc_failures_remain_integrity_errors_before_declarations_are_published() {
    let document = Document::from_bytes(include_bytes!(
        "fixtures/display-lists/context-directory-bad-crc.bin"
    ))
    .unwrap();
    let error = DisplayArchiveContext::from_document(
        &document,
        DisplayDocumentKind::Assembly,
        19000,
        tessellation(19000),
        TESS_LIMITS,
        LIMITS.maximum_bytes,
    )
    .unwrap_err();
    assert_eq!(error.class, FailureClass::Integrity);
    assert_eq!(error.entity, DIRECTORY);
}

#[test]
fn resolved_context_edits_match_independent_bytes_and_reopen_without_changing_other_streams() {
    for (name, file_version, directory_version, expected) in [
        (
            "part",
            0x223,
            19000,
            include_bytes!("fixtures/display-lists/context-inline-part-edited-display.bin")
                .as_slice(),
        ),
        (
            "pair",
            u32::MAX,
            0x3b1c,
            include_bytes!("fixtures/display-lists/context-external-pair-edited-display.bin")
                .as_slice(),
        ),
    ] {
        let kind = if name == "part" {
            DisplayDocumentKind::Part
        } else {
            DisplayDocumentKind::Assembly
        };
        let original = document(name);
        let context = resolved(&original, kind, file_version, directory_version);
        let source = original.read_stream("Contents/DisplayLists").unwrap();
        let archive = DisplayArchive::from_bytes(&source, context.clone(), LIMITS).unwrap();
        let mut contents = archive.contents().clone();
        contents
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == "root.visual_properties.material.ambient")
            .unwrap()
            .value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
        let edited = archive.with_contents(contents, LIMITS).unwrap();
        assert_eq!(edited.to_bytes().unwrap(), expected);
        let mut output = original.clone();
        output
            .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
            .unwrap();
        let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
        let reopened_context = resolved(&reopened, kind, file_version, directory_version);
        assert_eq!(reopened_context, context);
        assert_eq!(
            DisplayArchive::from_bytes(
                &reopened.read_stream("Contents/DisplayLists").unwrap(),
                reopened_context,
                LIMITS
            )
            .unwrap()
            .contents(),
            edited.contents()
        );
        for stream in original
            .streams()
            .filter(|stream| stream.name != "Contents/DisplayLists")
        {
            assert_eq!(
                reopened.read_stream(stream.name).unwrap(),
                original.read_stream(stream.name).unwrap()
            );
        }
        output
            .replace_stream("Contents/DisplayLists", archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(output.to_bytes().unwrap(), original.to_bytes().unwrap());
    }
}
