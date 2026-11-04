# Stored classification selector API

`ModelIntersectionReferenceStoredState::classification_selector()` copies
the saved u16 selector from a qualified ordinary intersection reference.
Every u16 value is retained. This API does not define a geometry enum or
evaluate native classification, matching, cache resolution or providers.
The existing `words().raw_state` access remains available.

`PartResolvedFeaturesArchive::edit_intersection_reference_classification`
replaces only that saved selector. The reference must belong to the current
revision. Signed raw20, span/backward words, object references and all other
fields are preserved. Complete archive validation, diagnostics, coverage,
completion and history use the existing model edit contract. No-ops retain
revision identity; later edits must reacquire views from a changed result.

```rust
# fn example(source: &bessemer::model_archives::PartResolvedFeaturesArchive) -> bessemer::Result<()> {
let references = source.intersection_reference_states()?;
if let Some(reference) = references.first() {
    let result = source.edit_intersection_reference_classification(reference, 65535)?;
    let feature_bytes = result.archive().to_bytes()?;
    assert!(!feature_bytes.is_empty());
}
# Ok(())
# }
```

The saved state has no offline validity restriction derived from native
producer stages or observed corpus values. The runtime32 member's high bits
are outside the saved u16 contract. Raw20 remains uninterpreted.

## Stored edge index

`intersection_edge_indices()` enumerates physical saved raw168 fields in
wire order. `intersection_edge_index(&field)` binds one qualified field to
an immutable `ModelIntersectionEdgeStoredIndex`; its `value()` is signed32
and `source_field()` retains the original revision identity.

`edit_intersection_edge_index(&index, value)` replaces only that I32 field.
The saved value is independent of the edge's container position. Negative
values and signed extremes are preserved; no count-derived validity domain
or correspondence encoding is imposed. All four preceding I32 header fields
must be adjacent and share the edge scope. The actual ordinary sketch,
marker0 body and canonical bounded container indices are also checked.

This operation returns the same complete archive edit contract and preserves
selectors, signed siblings and object references. It performs no native map
creation, matching, callbacks or geometry operations.

The remaining Source output, native/document/concurrency and publication
gates for this feature are still pending. Humanizer is unavailable; prose
review remains pending.
