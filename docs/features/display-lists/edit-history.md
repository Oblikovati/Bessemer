# Stored-archive edit outcomes

`DisplayArchive::edit_with_history` returns the validated archive together with
completion, diagnostics, exact validation coverage, achieved stored-field fidelity
and structural history. The existing `with_contents` API remains available.
This operation meets the archive part of #28's result contract; it does not edit
CAD topology or infer native entity names.

Construction first encodes every supplied typed field, reparses the complete
stream with the supplied limits and shared MFC map, and checks exact typed-field
equality. Failed schema, graph or budget checks return a typed error and publish
no report. An unchanged edit completes successfully with an empty change list.
Coverage counts the complete stream, fields, loaded objects and checked aliases.
Its explicit check mode is `ExactStoredArchive`.
The achieved fidelity is exact stored-field equality, including floating-point
bits, rather than a geometric error bound.

History compares structural paths in parser order. Repeated paths are paired by
their occurrence within that path. Changed values have a derived relation and
one source parent; removed fields have a deleted relation; added fields have a
created relation and no parent. Unchanged fields are counted separately. Source
relations are sorted by path and occurrence, followed by newly created paths in
the same order. These are wire locations, not evidence that a geometric entity
survived an array reorder.

Parents and results use immutable revision-bound field handles. Clones retain
their revision; separately decoded archives and changed edits have distinct
revisions, even when their field positions match. Handles retain their revision
token after an archive is dropped. `resolve_field` rejects a foreign revision
before accessing its field position. Adjacent edit reports can be followed through
their shared intermediate revision without inventing an identity remap.

Geometry validation is explicitly outside this serialization operation. If the
archive contains external face references, the report also records that their
target blocks were not checked. Applications must use the existing complete
directory/block link check to establish those links. Neither exclusion is reported
as a successful geometric or cross-stream check.

The focused tests use owned blank and shared-font archives, external facet
references and independently generated named-view edits. They cover exact bytes,
creation/deletion/derivation, complete parent resolution, unchanged edits, failed
edits, source isolation and deterministic concurrent reports. Native application
acceptance and geometry acceptance remain separate.

```rust
# fn example(source: &bessemer::display_lists::DisplayArchive, limits: bessemer::display_lists::DisplayArchiveLimits) -> bessemer::Result<()> {
let outcome = source.edit_with_history(source.contents().clone(), limits)?;
assert!(outcome.changes().is_empty());
let bytes = outcome.archive().to_bytes()?;
# Ok(())
# }
```

The humanizer skill was unavailable; prose remains subject to maintainer review.
