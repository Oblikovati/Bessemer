use super::*;

#[test]
fn reused_bucket_failures_keep_the_physical_entity_class_and_original_cause() {
    let (mut raw, cases) = cases();
    let graph = parse(&raw);
    let error = graph
        .edit_value(cases[0].page, cases[0].offset, b"bad")
        .unwrap_err();
    assert_eq!(error.entity, "bucket page 2");
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("replacement length 3"));
    raw[8 * 4096 + 16..8 * 4096 + 18].copy_from_slice(&0u16.to_be_bytes());
    let error = HashFileGraph::from_bytes(&raw, context(raw.len(), 4096)).unwrap_err();
    assert_eq!(error.entity, "bucket page 2");
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("checkpoint"));
}
