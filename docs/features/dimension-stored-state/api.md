# Stored dimension API

`PartConfigurationArchive::dimension_state` and
`PartResolvedFeaturesArchive::dimension_state` accept a revision-bound handle
for the physical dimension-base I32 `mask` field. They return an owned
`ModelDimensionStoredState` after checking the same-scope adjacent
`am_object_id`, `is_design_table_dimension`, `mask` and `unknown_5c` words.
Direct and nested dimension bases use the same typed check. Tolerance masks,
unrelated fields, incomplete groups and foreign or stale handles fail.

The view exposes the complete signed `mask()`, the stored
`enforce_bend_angle_range()` flag, `fillet_ref_point_change_number()` in zero
through seven, `mask_unresolved_bits()` and signed `redundant_id()`. Unresolved
bits mean bits outside this generic view's documented roles; a derived class may
give some of them another role. Reading preserves the raw words and does not
resolve native handles, impose ID constraints or apply native setters.

Both archive types offer `edit_dimension_state`, returning their existing
complete edit result with validation coverage, diagnostics, completion and
ordered field history. The request is `ModelDimensionStoredEdit`:

- `EnforceBendAngleRange(bool)` replaces only bit 1.
- `FilletRefPointChangeNumber(u8)` replaces bits 2 through 4. Inputs above seven
  return an Invalid error containing the offending value and expected range.
- `RedundantId(i32)` replaces only the neighboring signed word.

For example, given a handle selected from a complete resolved-feature archive:

```rust
use bessemer::model_archives::{
    ModelDimensionStoredEdit, ModelFieldHandle, PartResolvedFeaturesArchive,
};

fn clear_stored_counter(
    source: &PartResolvedFeaturesArchive,
    mask_field: &ModelFieldHandle,
) -> bessemer::Result<Vec<u8>> {
    let dimension = source.dimension_state(mask_field)?;
    let edit = source.edit_dimension_state(
        &dimension,
        ModelDimensionStoredEdit::FilletRefPointChangeNumber(0),
    )?;
    edit.archive().to_bytes()
}
```

No-op requests preserve bytes and revision identity. Changed results have new
revision-bound handles and one derived field relation with the original parent.
Every other field and bit remains unchanged. A view retains its source revision
after a result is produced; applying it to the changed result fails. Views,
requests and published archive results remain immutable and Send/Sync.

This is stored-file interoperability. Native counter increments, bend-angle
range enforcement, angle units, pointer allocation, application opening and
geometry validity are outside the operation.
