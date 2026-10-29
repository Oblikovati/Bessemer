use super::{ModelArchiveContext, ModelDecoder};
use crate::header::ArchiveVersion;
const OUTER: &[&str] = &["moDecalProperties_c"];
const INNER: &[&str] = &["moUnitComponent_c"];
fn scope_state_decoder() -> ModelDecoder<'static> {
    ModelDecoder::new_in_context(
        &[],
        ArchiveVersion::new(19000),
        super::super::annotation_view_tests::LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap()
}
#[test]
fn fresh_class_scopes_restore_both_nested_and_outer_selection_on_success() {
    let mut reader = scope_state_decoder();
    reader
        .scoped_fresh_objects(OUTER, "outer", |reader| {
            assert_eq!(reader.fresh_class_scope, Some((OUTER, "outer")));
            reader.scoped_fresh_objects(INNER, "inner", |reader| {
                assert_eq!(reader.fresh_class_scope, Some((INNER, "inner")));
                Ok(())
            })?;
            assert_eq!(reader.fresh_class_scope, Some((OUTER, "outer")));
            Ok(())
        })
        .unwrap();
    assert_eq!(reader.fresh_class_scope, None);
    assert!(reader.finish().unwrap().is_empty());
}
#[test]
fn fresh_class_scopes_restore_selection_before_poisoning_a_failed_operation() {
    let mut reader = scope_state_decoder();
    let result: crate::Result<()> = reader.scoped_fresh_objects(OUTER, "outer", |reader| {
        let failure: crate::Result<()> = reader.scoped_fresh_objects(INNER, "inner", |_| {
            Err(super::super::invalid(
                "fixture failure; expected an aborted operation",
            ))
        });
        assert!(failure.is_err());
        assert_eq!(reader.fresh_class_scope, Some((OUTER, "outer")));
        failure
    });
    assert!(result.is_err());
    assert_eq!(reader.fresh_class_scope, None);
    assert!(reader.finish().is_err());
}
