# Stored parameters (#202)

This work adds typed stored parameter snapshots and exact value edits over the
admitted complete part archives. Corliss's model parameter views are the design
reference. The existing Bessemer codecs already carry the parameter bodies;
the operation reuses their validation, snapshots, handles and history.

The first baseline covers density, length, angle, integer, attribute scalar and
attribute string parameters. The inherited node name and second parameter-name
pointer stay distinct, including nulls and aliases. Floating-point payloads,
signed integers, CString framing and alternate/override presence bytes retain
their complete stored domains. A stored alternate is not an evaluated effective
value. Units, equations, native setters and geometry are not inferred.

## Typed operation

Both archive types expose `stored_parameters()` in physical definition order
and `stored_parameter(&definition)` for a revision-bound object-tag selector.
Alias-only and null occurrences do not create duplicate parameter definitions.
Each immutable result owns its complete source and exposes the physical class,
both name pointers and their exact text, primary and alternate values, the raw
presence byte, copied field history, coverage, completion and diagnostics.
Name dependencies can belong to the inherited configuration; resolve their
handles through `state.source().resolve_field(...)`.

`edit_stored_parameter(&state, request)` replaces one primary value, an existing
alternate value or a raw presence byte through the complete archive editor.
Replacement values must retain their stored type. Presence edits must keep
the existing field shape: changing a nonzero byte to another nonzero byte is
supported; adding or removing a dependent alternate field is rejected.

```rust,ignore
use bessemer::model_archives::{ModelParameterStoredEdit, ModelParameterValue};
let state = archive.stored_parameter(&definition)?;
let changed = archive.edit_stored_parameter(
    &state,
    ModelParameterStoredEdit::Primary(ModelParameterValue::Integer(12345)),
)?;
let complete_bytes = changed.archive().to_bytes()?;
```

The owned tests cover exact edits, complete document reopen, byte-exact reversal,
wrong/foreign/stale selections, history, input isolation and concurrent edits.
Broader configuration-context fixtures for all six classes, fresh actual Source
exports, native outcomes and the full local/remote gates remain required before
publication. Higher-level model and geometry parity remain open in #2.

## Owned reference baseline

The original Source reader accepted 30 complete resolved-feature graphs:
24 combinations of six parameter classes and four pointer/value styles, plus
six independent primary-value edits. Each graph reached exact EOF with complete
primitive byte coverage and the expected inherited and final object-map sizes.
The checks compare both name pointers, including shared identity, nulls and
distinct names. Floating-point comparisons use their raw bytes, including NaN
payloads, signed zero, infinities and a subnormal value. Integer endpoints,
noncanonical presence bytes, embedded NULs, a surrogate pair and nonminimal
CString framing are retained in the owned wire fixtures.

Run `python -B -X utf8 docs/features/stored-parameters/freeze_owned_reference.py`
from the repository root to reproduce them. The script freezes every reference
Python file before importing readers, checks import origins and source hashes,
and compares generated fixtures byte for byte with the reviewed baseline.
The complete snapshot is retained as lossless gzip with a small JSON index.
The configuration operation currently has a complete blank-part density test;
the six-class independent wire baseline above is in resolved-feature context.
That distinction remains part of the acceptance scope.

## Native projection check

One complete retained-corpus run passed all 906 cases, with 4,430 physical
configuration parameters and 39,487 physical resolved-feature parameters.
The typed population matched a separate physical class scan, every projected
name/history dependency resolved and both complete archive streams reconstructed
byte for byte. The run took 306.06 seconds on Windows with the explicit installed
kernel schema. Counts and log hashes are retained in `native-verification.json`;
all per-case counts are retained as lossless gzip.

This is a read/projection and internal consistency check. Fresh original Source
comparison of actual parameter exports, native edits and whole native document
reopen/reversal remain open. It does not establish a class-by-context/version
population matrix or native application and geometry acceptance.

## Actual output comparison

The owned output check compares Bessemer's actual exported records with the
unchanged original reference methods. Two fresh Rust processes produced the
same 6,566-byte packet for all 30 owned graphs. The original readers matched
every physical definition and name tag, decoded text and CString frame, raw
numeric image, presence byte and alternate value. Profiling observes calls and
returns; it does not replace reader methods. Source hashes, method identities,
import origins and complete EOF remain checked.

The exporter reuses the root-graph packet helpers. After that extraction, a
fresh 906-case root run produced the same 1,508,863 bytes as the retained
pre-extraction output. The parameter decoder also has three literal and
malformed-input tests.

The complete native comparison also passed: every exported record for all
906 cases matched unchanged frozen original Source, including 4,430
configuration and 39,487 resolved-feature parameters. Two fresh Rust processes
produced identical 7,263,585-byte packets. The two Source tests completed in
323.748 seconds; the complete 6,733-file source snapshot and import evidence
are retained losslessly. This timing is a verification measurement, not a
release benchmark.

`native-actual-output-census.json` records the class/version/context populations.
All six classes occur in resolved features; four occur in configurations.
The native name-pointer population contains null and local pointers but no
inherited pointers. The independent positive inherited check below covers that
missing population. Broader native edits, whole native document
reopen/reversal and application/geometry acceptance remain separate. Bug #177's
unknown-byte accounting oracle is unchanged.

To reproduce the complete comparison, use a fresh unused
`target/stored-parameter-actual-results` directory, the retained
`target/resolved-native-probe.bin` packet and the explicit kernel-schema setup
used by the native corpus tests. For each run number `1` and `2`, set
`BESSEMER_PARAMETER_OWNED_PACKET` to the corresponding absolute `owned-N.bin`
path and run:

```text
cargo test --lib owned_parameter_exports_cover_every_class_name_style_and_independent_primary_edit
```

Set `BESSEMER_PARAMETER_RESULT_PACKET` to the corresponding absolute
`native-N.bin` path and run:

```text
cargo test --lib every_native_parameter_definition_matches_physical_population_and_keeps_complete_carriers -- --ignored --nocapture
python -B -X utf8 docs/features/stored-parameters/verify_parameter_exports.py
```

Run the Python comparison after both native exports finish. The `--owned-only`
option permits an early owned comparison while native exports are running.
Packet files append records, so reuse would invalidate the fresh-process check.

## Positive inherited name aliases

Two independently assembled 66-byte feature graphs retain both name pointers
as references to configuration object 5, the existing `Part1` name. The unchanged
original readers accepted both graphs and a fresh strict run regenerated every
fixture byte. The edited graph changes only the primary double from 1.25 to -9.5.
The original 30 graphs also regenerated unchanged after adding this separate
fixture directory; their earlier source provenance is preserved.

All 13 focused Rust tests pass. The inherited edit matches the independent
golden, preserves the shared configuration text handle and its copied history,
reopens the complete document and reverses to its original bytes. Two fresh
Rust processes produced identical 444-byte result packets. Those actual origin1,
name and value records matched a fresh unchanged original-reader run with
source-integrity, import-origin and method-identity guards.

Run `freeze_inherited_reference.py` in this directory for strict wire-fixture
regeneration. To reproduce actual-output comparison, set
`BESSEMER_PARAMETER_INHERITED_PACKET` to unused absolute `inherited-1.bin` and
`inherited-2.bin` paths under `target/stored-parameter-actual-results`, running
the following test in two fresh processes:

```text
cargo test --lib inherited_parameter_exports_match_both_independent_source_goldens
python -B -X utf8 docs/features/stored-parameters/freeze_inherited_reference.py --actual-exports
```

`inherited-verification.json` records the evidence and exclusions. These are
owned positive cases; the native corpus still has no inherited-name population.

The combined local PR gate at `d3c3e4c` passed 4,071 tests with zero failures.
That checkout contains the first parameter operation and 12 integration tests;
later exports and the 13th inherited test have separate focused evidence.
`full-local-gate-verification.json` records this boundary. Exact-head remote
publication and the broader native editing/application requirements remain open.

## First complete native edits

The first retained native case now passes one configuration-density edit and
one resolved-feature length edit at model version 11000. The unchanged original
reader supplies the primary double offsets 18891 and 17867 and defining object
IDs 360 and 505. Independent one-bit replacements are reparsed by Source; every
other projected parameter value, name and identity remains equal. A fresh run
regenerated the 72-byte span oracle exactly.

The typed editor matches each complete expected stream byte for byte. Both
edited documents reopen through the public archive readers, preserve all
unrelated streams and reverse through fresh typed states to the original file
bytes. Input isolation and one-parent changed-field history also pass. The
native test completed in 0.41 seconds; Clippy with warnings denied, formatting
passed. The initial function-size check ran before formatting completed and
missed three oversized test helpers. Bug #203 split those helpers and reran
the unchanged native test successfully in 0.45 seconds, then verified every
formatted body and file against the limits. `size-audit-correction.json` retains
the correction and original scope; the earlier size claim is superseded.
`native-first-edit-verification.json`
records the input hash, source snapshot, commands' log hashes and exclusions.

Regenerate the oracle with:

```text
python -B -X utf8 docs/features/stored-parameters/freeze_native_primary_reference.py
cargo test --lib first_native_parameter_edits_match_original_spans_reopen_and_reverse_complete_documents -- --ignored --nocapture
```

The native test requires the retained corpus packet, its original installed
sample file and the explicit kernel schema. This proves the first complete
native operation in both contexts. It does not cover the full parameter edit
population, other stored kinds, native application opening or geometric validity.
