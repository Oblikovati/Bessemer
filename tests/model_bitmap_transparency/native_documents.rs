use super::*;

fn native_bitmap_document_source(
    document: &Document,
    case: &NativeBitmapCase<'_>,
    kernel: &CompressedKernelContext,
) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        native_bitmap_id(case),
        case.version,
        Some(kernel.clone()),
        NATIVE_LIMITS,
    )
    .unwrap()
}

fn reopened_native_bitmap(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    case: &NativeBitmapCase<'_>,
    kernel: &CompressedKernelContext,
    role: usize,
) -> (Document, Document) {
    let state = native_bitmap_state(source);
    let result = source
        .edit_bitmap_transparency(
            &state,
            setting_request(role, [i32::MIN, i32::MAX, -1][role]),
        )
        .unwrap();
    verify_bitmap_result_contract(source, &result, true);
    reopen_native_bitmap_result(document, &result, case, kernel)
}

fn native_bitmap_stream(case: &NativeBitmapCase<'_>) -> String {
    format!(
        "{}-ResolvedFeatures",
        case.label.rsplit_once("::").unwrap().1
    )
}

fn reopen_native_bitmap_result(
    document: &Document,
    result: &PartResolvedFeaturesEdit,
    case: &NativeBitmapCase<'_>,
    kernel: &CompressedKernelContext,
) -> (Document, Document) {
    let stream = native_bitmap_stream(case);
    let mut output = document.clone();
    output
        .replace_stream(&stream, result.archive().to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let actual = native_bitmap_document_source(&reopened, case, kernel);
    assert_eq!(actual.fields(), result.archive().fields());
    crate::unselected_document_streams::assert_unselected_document_streams(
        document, &reopened, &stream,
    );
    (output, reopened)
}

fn reverse_native_bitmap(
    document: &Document,
    output: &mut Document,
    reopened: &Document,
    source: &PartResolvedFeaturesArchive,
    case: &NativeBitmapCase<'_>,
    kernel: &CompressedKernelContext,
    role: usize,
) {
    let actual = native_bitmap_document_source(reopened, case, kernel);
    let state = actual.bitmap_transparency_states().unwrap().remove(0);
    let reversal = actual
        .edit_bitmap_transparency(&state, setting_request(role, 0))
        .unwrap();
    assert_eq!(reversal.archive().fields(), source.fields());
    assert_eq!(reversal.archive().to_bytes().unwrap(), case.features);
    let stream = native_bitmap_stream(case);
    output
        .replace_stream(&stream, reversal.archive().to_bytes().unwrap())
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}

fn verify_native_bitmap_document(case: &NativeBitmapCase<'_>, kernel: &CompressedKernelContext) {
    let labelled = case.label.split_once(": ").unwrap().1;
    let original = std::fs::read(labelled.rsplit_once("::").unwrap().0).unwrap();
    let document = Document::from_bytes(&original).unwrap();
    let source = native_bitmap_document_source(&document, case, kernel);
    assert_eq!(source.to_bytes().unwrap(), case.features);
    for role in 0..3 {
        let (mut output, reopened) = reopened_native_bitmap(&document, &source, case, kernel, role);
        reverse_native_bitmap(
            &document,
            &mut output,
            &reopened,
            &source,
            case,
            kernel,
            role,
        );
    }
    assert_eq!(source.to_bytes().unwrap(), case.features);
    assert_eq!(document.to_bytes().unwrap(), original);
}

#[test]
#[ignore = "Requires original installed native files,906-case BRF1 corpus and explicit kernel schema"]
fn all_native_bitmap_documents_reopen_preserve_streams_and_reverse_original_bytes() {
    let packet = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let kernel = native_bitmap_kernel();
    for case in native_bitmap_cases(&packet) {
        verify_native_bitmap_document(&case, &kernel);
        println!("BITMAP_NATIVE_DOCUMENT\t{}\t3", case.index);
    }
}
