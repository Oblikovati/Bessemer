use super::*;

fn native_database_bytes(bucket: &HashWriterBucket) -> Vec<u8> {
    let mut raw = NATIVE_STREAMS[2].to_vec();
    raw[8192..12288].copy_from_slice(&bucket.to_bytes());
    for page in bucket.to_overflow_pages() {
        let start = page.identifier.get() as usize * 4096;
        raw[start..start + 4096].copy_from_slice(&page.bytes);
    }
    raw
}

fn bucket_from_database(raw: &[u8], case: &BucketCase) -> HashWriterBucket {
    let pages: Vec<_> = case
        .pages
        .iter()
        .map(|p| {
            let start = p.identifier.get() as usize * 4096;
            OverflowPageImage {
                identifier: p.identifier,
                bytes: raw[start..start + 4096].to_vec(),
            }
        })
        .collect();
    HashWriterBucket::from_bytes(
        &raw[8192..12288],
        &pages,
        bucket_context(case),
        case.free.len() + case.cells.len(),
    )
    .unwrap()
}

fn check_bucket_document(case: &BucketCase, cell: &ExpectedCell) {
    let source = parse_bucket(case).unwrap();
    let original = crate::documents::containing_document();
    let value: Vec<_> = cell.value.iter().map(|b| b ^ 0x5a).collect();
    let edit = source.edit_value(cell.offset, &value).unwrap();
    let mut output = original.clone();
    output
        .replace_stream(
            "Contents/3DExperienceExchange2",
            native_database_bytes(edit.result()),
        )
        .unwrap();
    let fresh = reopened_bucket(case, &original, &output);
    assert_eq!(fresh, *edit.result());
    reverse_bucket_document(&original, output, fresh, cell);
}

fn reopened_bucket(
    case: &BucketCase,
    original: &bessemer::Document,
    output: &bessemer::Document,
) -> HashWriterBucket {
    let reopened = bessemer::Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    crate::documents::assert_unselected_streams(original, &reopened);
    bucket_from_database(
        &reopened
            .read_stream("Contents/3DExperienceExchange2")
            .unwrap(),
        case,
    )
}

fn reverse_bucket_document(
    original: &bessemer::Document,
    mut output: bessemer::Document,
    fresh: HashWriterBucket,
    cell: &ExpectedCell,
) {
    let reverse = fresh.edit_value(cell.offset, &cell.value).unwrap();
    output
        .replace_stream(
            "Contents/3DExperienceExchange2",
            native_database_bytes(reverse.result()),
        )
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), original.to_bytes().unwrap());
}

#[test]
fn every_native_mixed_cell_reopens_complete_owned_container_and_reverses_from_fresh_bucket() {
    let cases = fixture_cases();
    let case = &cases[121];
    for cell in &case.cells {
        check_bucket_document(case, cell);
    }
}
