use super::*;
use crate::native_census::native_documents::{
    NativeDocumentInput, assert_unselected_document_streams, cached_document, document_sources,
    sample_root,
};
use crate::writer_buckets::bucket_documents::{read_selected_bucket, write_selected_bucket};
use bessemer::Document;

fn original_bucket(input: &NativeDocumentInput, case: &NativeBucketCase) -> HashWriterBucket {
    let source = read_selected_bucket(&input.raw, case.index as usize, &case.expected);
    check_cells(&case.expected, &source);
    source
}

fn edited_document(
    input: &NativeDocumentInput,
    case: &NativeBucketCase,
    bucket: &HashWriterBucket,
) -> (Document, HashWriterBucket) {
    let changed = write_selected_bucket(&input.raw, case.index as usize, bucket);
    let mut output = input.document.clone();
    output
        .replace_stream(&input.stream, changed.clone())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    assert_eq!(reopened.read_stream(&input.stream).unwrap(), changed);
    assert_unselected_document_streams(&input.document, &reopened, &input.stream);
    let fresh = read_selected_bucket(&changed, case.index as usize, &case.expected);
    assert_eq!(&fresh, bucket);
    (output, fresh)
}

fn reverse_document(
    input: &NativeDocumentInput,
    case: &NativeBucketCase,
    cell: &ExpectedCell,
    mut output: Document,
    fresh: &HashWriterBucket,
) {
    let reverse = fresh.edit_value(cell.offset, &cell.value).unwrap();
    let restored = write_selected_bucket(&input.raw, case.index as usize, reverse.result());
    assert_eq!(restored, input.raw);
    output.replace_stream(&input.stream, restored).unwrap();
    assert_eq!(output.to_bytes().unwrap(), input.original);
    assert_eq!(input.document.to_bytes().unwrap(), input.original);
}

fn check_document_cell(
    input: &NativeDocumentInput,
    case: &NativeBucketCase,
    source: &HashWriterBucket,
    cell: &ExpectedCell,
) {
    let value: Vec<_> = cell.value.iter().map(|byte| byte ^ 0x5a).collect();
    let edit = source.edit_value(cell.offset, &value).unwrap();
    let expected = expected_bucket_edit(&case.expected, cell, &value);
    assert_eq!(
        (edit.result().to_bytes(), edit.result().to_overflow_pages()),
        expected
    );
    let (output, fresh) = edited_document(input, case, edit.result());
    reverse_document(input, case, cell, output, &fresh);
    println!(
        "{{\"stream_ordinal\":{},\"bucket_index\":{},\"cell_offset\":{}}}",
        case.ordinal, case.index, cell.offset
    );
}

fn check_document_bucket(input: &NativeDocumentInput, case: &NativeBucketCase) -> usize {
    let source = original_bucket(input, case);
    for cell in &case.expected.cells {
        check_document_cell(input, case, &source, cell);
    }
    case.expected.cells.len()
}

#[test]
#[ignore = "requires explicit BESSEMER_OVERFLOW_SAMPLE_ROOT and full source-root preflight"]
fn all_native_mixed_bucket_cells_reopen_original_documents_and_reverse_exactly_with_checkpoints() {
    let root = sample_root();
    let sources = document_sources();
    let raw = census_bytes();
    let mut cursor = &raw[8..];
    let mut current = None;
    let mut total = 0;
    for _ in 0..1023 {
        let case = native_bucket_case(&mut cursor);
        let input = cached_document(&mut current, &root, &sources, case.ordinal);
        total += check_document_bucket(input, &case);
    }
    assert!(cursor.is_empty());
    assert_eq!(total, 13842);
    println!("NATIVE_MIXED_DOCUMENT_TOTAL\t13842\t1023\t440\t357");
}
