//! Native empty-result acceptance stays separate from the positive owned visual matrices (#181).
use bessemer::Document;
use bessemer::display_lists::*;
use bessemer::header::ArchiveVersion;
use bessemer::tessellation::{FacetArchiveMode, TessellationContext, TessellationLimits};
use std::path::Path;
#[allow(
    dead_code,
    reason = "Shared framing also serves the archive reconstruction examples."
)]
#[path = "../examples/support/framing.rs"]
mod framing;

const NATIVE_LIMITS: DisplayArchiveLimits = DisplayArchiveLimits {
    maximum_bytes: 256 * 1024 * 1024,
    maximum_fields: 2_000_000,
    maximum_objects: 1_000_000,
    maximum_depth: 256,
    maximum_entries: 2_000_000,
};
// #181: Source defaults these display caller inputs independently of _MO_VERSION_.
const SOURCE_CURRENT_FILE_VERSION: u32 = 19000;
const SOURCE_DIRECTORY_VERSION: u32 = 19000;

const SOURCE_TESSELLATION: TessellationContext = TessellationContext {
    archive_version: ArchiveVersion::new(SOURCE_DIRECTORY_VERSION),
    mode: FacetArchiveMode::Full,
};
const NATIVE_TESSELLATION_LIMITS: TessellationLimits = TessellationLimits {
    maximum_components: 1_000_000,
    maximum_faces: 1_000_000,
    maximum_array_elements: 2_000_000,
};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum NativeGraphicKind {
    Part = 0,
    Assembly = 1,
    Drawing = 2,
}

fn native_graphic_kind(value: u8) -> NativeGraphicKind {
    match value {
        0 => NativeGraphicKind::Part,
        1 => NativeGraphicKind::Assembly,
        2 => NativeGraphicKind::Drawing,
        other => panic!("{other}; expected part0, assembly1 or drawing2"),
    }
}

struct NativeGraphicOracle<'a> {
    index: u32,
    kind: NativeGraphicKind,
    model_version: ArchiveVersion,
    archive_version: u32,
    expected_bytes: usize,
    sample_name: &'a str,
    source_digest: &'a [u8],
}

fn next_graphic_oracle<'a>(cursor: &mut &'a [u8]) -> NativeGraphicOracle<'a> {
    let index = framing::take_u32(cursor).unwrap();
    let kind = native_graphic_kind(framing::take(cursor, 1).unwrap()[0]);
    let model_version = ArchiveVersion::new(framing::take_u32(cursor).unwrap());
    let archive_version = framing::take_u32(cursor).unwrap();
    let expected_bytes = framing::take_u32(cursor).unwrap() as usize;
    let label_length = framing::take_u32(cursor).unwrap() as usize;
    let sample_name = std::str::from_utf8(framing::take(cursor, label_length).unwrap()).unwrap();
    let source_digest = framing::take(cursor, 32).unwrap();
    assert_eq!(framing::take_u32(cursor).unwrap(), 0);
    NativeGraphicOracle {
        index,
        kind,
        model_version,
        archive_version,
        expected_bytes,
        sample_name,
        source_digest,
    }
}

fn native_graphic_context(
    document: &Document,
    case: &NativeGraphicOracle<'_>,
) -> DisplayArchiveContext {
    let kind = match case.kind {
        NativeGraphicKind::Part => DisplayDocumentKind::Part,
        NativeGraphicKind::Assembly => DisplayDocumentKind::Assembly,
        other => panic!("{other:?}; expected a part or assembly display"),
    };
    let context = DisplayArchiveContext::from_document(
        document,
        kind,
        SOURCE_CURRENT_FILE_VERSION,
        SOURCE_TESSELLATION,
        NATIVE_TESSELLATION_LIMITS,
        32 * 1024 * 1024,
    )
    .unwrap();
    assert_eq!(context.version.value(), case.archive_version);
    context
}

fn native_display_empty_result(document: &Document, case: &NativeGraphicOracle<'_>) -> Vec<u8> {
    let raw = document.read_stream("Contents/DisplayLists").unwrap();
    assert_eq!(raw.len(), case.expected_bytes);
    let context = native_graphic_context(document, case);
    let source = DisplayArchive::from_bytes(&raw, context.clone(), NATIVE_LIMITS).unwrap();
    assert!(source.entity_visual_states().unwrap().is_empty());
    let rebuilt = DisplayArchive::new(source.contents().clone(), context, NATIVE_LIMITS).unwrap();
    assert!(rebuilt.entity_visual_states().unwrap().is_empty());
    let actual = rebuilt.to_bytes().unwrap();
    assert_eq!(actual, raw);
    assert_eq!(source.to_bytes().unwrap(), raw);
    actual
}

fn native_drawing_empty_result(document: &Document, case: &NativeGraphicOracle<'_>) -> Vec<u8> {
    assert_eq!(case.model_version.value(), case.archive_version);
    let raw = document.read_stream("Contents/VBLists").unwrap();
    assert_eq!(raw.len(), case.expected_bytes);
    let source = DrawingBucketArchive::from_document(document, NATIVE_LIMITS).unwrap();
    assert!(source.entity_visual_states().unwrap().is_empty());
    let rebuilt =
        DrawingBucketArchive::new(source.contents().clone(), case.model_version, NATIVE_LIMITS)
            .unwrap();
    assert!(rebuilt.entity_visual_states().unwrap().is_empty());
    let actual = rebuilt.to_bytes().unwrap();
    assert_eq!(actual, raw);
    assert_eq!(source.to_bytes().unwrap(), raw);
    actual
}

fn native_graphic_document(root: &Path, case: &NativeGraphicOracle<'_>) -> (Vec<u8>, Vec<u8>) {
    let original = std::fs::read(root.join(case.sample_name)).unwrap();
    let mut document = Document::from_bytes(&original).unwrap();
    let (stream, actual) = match case.kind {
        NativeGraphicKind::Drawing => (
            "Contents/VBLists",
            native_drawing_empty_result(&document, case),
        ),
        _ => (
            "Contents/DisplayLists",
            native_display_empty_result(&document, case),
        ),
    };
    document.replace_stream(stream, actual.clone()).unwrap();
    let whole = document.to_bytes().unwrap();
    assert_eq!(whole, original);
    let reopened = Document::from_bytes(&whole).unwrap();
    assert_eq!(reopened.read_stream(stream).unwrap(), actual);
    (actual, whole)
}

fn append_native_graphic_export(
    output: &mut Option<Vec<u8>>,
    case: &NativeGraphicOracle<'_>,
    raw: &[u8],
) {
    let Some(output) = output else {
        return;
    };
    output.extend_from_slice(&case.index.to_le_bytes());
    output.extend_from_slice(case.source_digest);
    output.extend_from_slice(&u32::try_from(raw.len()).unwrap().to_le_bytes());
    output.extend_from_slice(raw);
}

fn verify_native_graphic_case(
    root: &Path,
    case: &NativeGraphicOracle<'_>,
    output: &mut Option<Vec<u8>>,
) {
    let (actual, _) = native_graphic_document(root, case);
    append_native_graphic_export(output, case, &actual);
    println!(
        "VISUAL_NATIVE_GRAPHIC\t{}\t{}\t{}",
        case.index,
        case.kind as u8,
        actual.len()
    );
}

#[test]
#[ignore = "requires all1106 native graphics documents, Source oracle packet and BESSEMER_SOLIDWORKS_SAMPLES"]
fn native_graphic_visual_results_are_complete_empty_and_force_exact_whole_documents() {
    let root = std::env::var_os("BESSEMER_SOLIDWORKS_SAMPLES")
        .expect("expected an injected native sample root");
    let packet = std::fs::read("target/entity-visual-graphics-source-spans.bin").unwrap();
    let path = std::env::var_os("BESSEMER_NATIVE_GRAPHIC_VISUAL_OUTPUT");
    let mut output = path
        .as_ref()
        .map(|_| [b"BVG1".as_slice(), &1106_u32.to_le_bytes()].concat());
    assert_eq!(
        verify_native_graphic_oracles(Path::new(&root), &packet, &mut output),
        [910, 176, 20]
    );
    if let (Some(path), Some(output)) = (path, output) {
        std::fs::write(path, output).unwrap();
    }
}

fn verify_native_graphic_oracles(
    root: &Path,
    packet: &[u8],
    output: &mut Option<Vec<u8>>,
) -> [usize; 3] {
    let mut cursor = packet;
    assert_eq!(framing::take(&mut cursor, 4).unwrap(), b"BVD1");
    assert_eq!(framing::take_u32(&mut cursor).unwrap(), 1106);
    let mut counts = [0; 3];
    for index in 0..1106 {
        let case = next_graphic_oracle(&mut cursor);
        assert_eq!(case.index, index);
        verify_native_graphic_case(root, &case, output);
        counts[case.kind as usize] += 1;
    }
    framing::require_finished(cursor).unwrap();
    counts
}
