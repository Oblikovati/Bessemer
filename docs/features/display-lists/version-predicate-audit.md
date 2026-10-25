# Executed version predicates

`audit_version_predicates.py` inventories selected owned complete part and
assembly roots, verifies their persisted hashes and contexts, and forces exact
reconstruction in two Rust processes. It then parses each root to EOF using the
reference reader from `../inventor-interface/solidworks`.

An in-memory AST probe records the outcomes of version comparisons in six
reference modules. The probe returns the original comparison value and preserves
short-circuit evaluation. It never edits the reference files or writes their
bytecode cache. Unit tests cover instrumentation, source loading, bounded framing
and corrupted manifests. Source hashes and observed contexts are persisted in
`version-predicate-audit.json`.

The recorded run covers 11,183 roots in 134 groups. It recognizes 391 version
comparisons, with both outcomes observed for 358. The remaining 33 are preserved
as one-sided or unobserved entries rather than counted as covered.

This measures execution of recognized version comparisons, not schema acceptance.
It excludes negative roots, separate typed archives and drawing-only contexts.
It does not prove every value or interaction within an observed branch. A
comparison with both outcomes still needs field oracles and mutation tests.
Unobserved and one-sided comparisons remain visible for review.

The first inventory exposed missing fixture formats. Adding version-range,
legacy-bucket and fixed-context roots recovered existing modern item and 3D view
witnesses. The audit also exposed a real full-root witness gap for named-view
suffixes, now covered by [the named-view integration fixtures](named-branch-layouts.md).

Several remaining single-sided comparisons have narrower callers. Root scenes
start above 3934, so older scene and concept branches cannot be reached through
that root field. Assembly configurations start at 9220, excluding the helper's
older active-configuration branch. Annotation visibility is called only above
the helper's early-return boundary. The light module delegates line-width
decoding to `dl_ui`, leaving its fallback unexecuted. `read_view_record` is a
helper rather than the main reader's inlined view loop. These observations do
not waive other callers or unresolved payloads.

Decal lists start above 4055 in the body, part and assembly component readers,
so their texture reader always takes its suffix above 3869 in these roots.
The facet predicate involving archive mode 90 short-circuits in this inventory;
drawing VBLists has a separate acceptance corpus. The uncounted-window and
unresolved inline-facet intervals are negative cases, excluded from this positive
inventory and tested as explicit failures elsewhere.

From the repository root, reproduce the checks with:

```text
python -B -X utf8 tests/test_display_predicate_audit.py
cargo build --locked --examples
python -B -X utf8 docs/features/display-lists/audit_version_predicates.py target/debug/examples/rebuild_display_archives
```

Use the `.exe` suffix on Windows and the configured target directory when one
is set. The reference checkout is required for the audit, but not its unit tests.

Non-null bitmap payloads, legacy PW material branches, unresolved inline facet
layouts and drawing DisplayLists still require their own acceptance decisions.
The audit is evidence for that review, not a declaration of completion.
