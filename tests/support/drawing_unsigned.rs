//! Unsigned model markers retain the documented modern drawing predicates.
use super::*;

fn unsigned_cases() -> Vec<(ArchiveVersion, &'static [u8], &'static [u8])> {
    let roots = frames(include_bytes!("../fixtures/drawing-buckets/unsigned.bin"));
    let fields = frames(include_bytes!(
        "../fixtures/drawing-buckets/unsigned-fields.bin"
    ));
    let versions = include_bytes!("../fixtures/drawing-buckets/unsigned-versions.bin");
    roots
        .into_iter()
        .zip(fields)
        .zip(versions.as_chunks::<4>().0)
        .map(|((raw, fields), version)| {
            (
                ArchiveVersion::new(u32::from_le_bytes(*version)),
                raw,
                fields,
            )
        })
        .collect()
}

#[test]
fn high_model_versions_match_independent_fields_and_complete_forced_bytes() {
    assert_eq!(unsigned_cases().len(), 25);
    let mut fields = 0;
    for (version, raw, expected) in unsigned_cases() {
        let archive = DrawingBucketArchive::from_bytes(raw, version, LIMITS).unwrap();
        assert_eq!(drawing_bucket_trace::trace(&archive), expected);
        fields += expected.iter().filter(|byte| **byte == b'\n').count();
        assert_eq!(archive.model_version(), version);
        let forced =
            DrawingBucketArchive::new(archive.contents().clone(), version, LIMITS).unwrap();
        assert_eq!(forced.to_bytes().unwrap(), raw);
    }
    assert_eq!(fields, 695);
}

#[test]
fn signed_model_aliases_resolve_once_and_whole_document_edits_reopen() {
    let mut document = Document::from_bytes(include_bytes!(
        "../fixtures/drawing-buckets/unsigned-document.bin"
    ))
    .unwrap();
    let source = DrawingBucketArchive::from_document(&document, LIMITS).unwrap();
    assert_eq!(source.model_version().value(), u32::MAX);
    let mut contents = source.contents().clone();
    replace_edited_fields(&mut contents, &parsed(cases()[3]));
    let edited = source.with_contents(contents, LIMITS).unwrap();
    assert_eq!(edited.to_bytes().unwrap(), cases()[3]);
    let mut reopened = replace_reopen(&mut document, &edited);
    check_unsigned_revert(&mut reopened, &source);
    assert_eq!(source.to_bytes().unwrap(), cases()[2]);
}

fn check_unsigned_revert(reopened: &mut Document, source: &DrawingBucketArchive) {
    let result = DrawingBucketArchive::from_document(reopened, LIMITS).unwrap();
    assert_eq!(result.model_version().value(), u32::MAX);
    assert_eq!(result.to_bytes().unwrap(), cases()[3]);
    let reverted = result
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    let reverted = replace_reopen(reopened, &reverted);
    assert_eq!(
        DrawingBucketArchive::from_document(&reverted, LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        cases()[2]
    );
}
