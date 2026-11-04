# Stored spline equation state (#199)

Feature #199 adds `spline_equation_states`, `spline_equation_state` and
`edit_spline_equation` to the complete resolved-feature archive. They expose the
six saved string roles, full signed option and selected endpoint-lock masks with
an immutable source snapshot, copied field history, completion, exact stored
coverage, achieved stored-value error and semantic exclusions. The first complete
typed lock edit matches the independent Source golden, reopens the whole document
and reverses it byte-for-byte. Broader acceptance remains in progress. The
original equation runtime and geometry work in #2 and #54 remains open.

## Wire roles and decision

Use the role bindings in the [retained API/runtime audit](../spline-equation-transform/equation-api-runtime-schema-audit.json)
and its source snapshot. Keep raw `ArchiveString` values for slots 0–5: saved X,
Y and Z expressions, declared-character text, and range-start and range-end text.
These are saved strings, not converted BSTRs or evaluated doubles.

Retain `unknown_1e0` as a full signed32 option despite the API Boolean formal.
Keep the preceding `equation_word` separate and unchanged unless a later explicit
contract admits its edit. The selected equation-path start/end locks use
`flags_32c` masks `0x10` and `0x20`. They are not F4 flags. Each lock edit replaces
only its selected mask and preserves the equation-presence bit and every other
stored bit. No native setter cascade, update trigger or transform is simulated.

## First complete operation

Reuse `PartResolvedFeaturesArchive` field handles, validation, edit history and
document replacement. Bind a physical spline inside its actual sketch definition
and collection; a matching path suffix alone is insufficient. Resolve selection
and saved version once. Require the equation-presence guard and exact adjacent
field roles/types before publishing a state. Enumeration of a valid archive with
no equation splines completes empty; a wrong or foreign selection fails explicitly.

Build the first owned complete feature graph with the existing independent
`WireMap`, ordinary sketch builder and direct spline components. The existing
single-span fixture wrapper demonstrates the complete sketch/feature observation
route; reuse that route rather than inventing an archive stub. Observe the full
result through a frozen original Source copy with EOF and primitive coverage.
Prove read, one selected replacement, document reopen and exact reversal before
extending the matrix.

## Required acceptance

Compare raw strings and option/flag images with literal independent expectations.
Cover both supported direct spline prototypes and saved-version contexts,
duplicate physical occurrences, aliases, absent equations, foreign/stale handles,
full signed option boundaries and raw string encodings. Keep complete unselected
fields, stream bytes and history unchanged. No-ops retain identity.

Return completion, diagnostics, declared validation scope, history and achieved
stored-value error through the existing operation contracts. Assert public types
are Send/Sync, prove input isolation after success and failure, compare eight
workers with serial results, and compare complete fresh-process exports against
Source. Preserve native corpus outcomes and all earlier fixtures. A full local
PR gate and exact-head remote checks precede publication. Publication remains
serialized behind PR #191; this work creates no additional branch or PR.

Runtime expression parsing/evaluation, public API string/double conversion,
provider contexts, transform interpretation, loaded-object reuse and native
geometry acceptance are separate requirements. Their absence is not covered by
this stored-state operation.

## Owned baseline

The [preparation proof](preparation-verification.json) records 18 complete Source
feature graphs with 22,282 bytes and 6,016 primitive reads, plus 28 direct cases.
The frozen Source tests and import/content guard passed. Two Bessemer tests using
the existing archive API reconstructed the saved fields, matched the independent
lock edits, reopened complete documents and reversed them byte-for-byte. The
owned documents require the saved-manager context used by the Source generator;
the earlier blank-manager failure is retained rather than hidden.

These fixtures pin existing behavior before the new role API. The first typed
projection and edit operation is implemented; the complete acceptance matrix and
publication gates remain unfinished. No new runtime or geometry coverage is claimed.

Reproduce the owned baseline without changing its files:

```powershell
python -B -X utf8 docs/features/spline-equation-state/freeze_owned_reference.py
```

The command regenerates every fixture in a temporary root and requires exact
agreement. The small source-snapshot index identifies a deterministic gzip that
retains every byte of the full import/hash record; no provenance is discarded.


## Expanded stored-value checks

The [matrix proof](matrix-verification.json) records 906 owned edit requests:
48 lock requests, 120 signed option requests, 720 text replacements and 18 count
prefix transitions. Expected bytes come from literal words and CString frames,
with offsets assembled independently of Bessemer. The 14-test suite passed;
eight workers also replayed 384 edits against serial bytes and preserved their
shared inputs. This concurrency check does not replace ThreadSanitizer.

The frozen Source replay regenerated every fixture exactly. Its source copy
added 40 Python files and changed none of the earlier files. The earlier full
snapshot remains in `preparation-owned-source-snapshot.json.gz`; the current
snapshot includes the replay identity. Malformed ownership cases, fresh actual
edit exports, native outcomes and combined publication gates remain open.


All 906 requests also passed complete-document reopening, preservation of
unrelated streams and byte-exact reversal. Five private scope tests cover
admitted category masks, alias owners, counts, malformed paths, missing extents
and repeated paths under separate physical sketch definitions. The scope tests
and the expanded integration suite passed Clippy. These owned checks do not
establish every saved-version context or native application behavior.


## Actual edited output reference check

The matrix can export each actual result as a complete document, its resolved
feature stream and the nine projected saved values (six strings and three words).
Run the four matrix tests with one test thread when exporting; packet order is
then deterministic. Choose an unused result directory so an earlier packet
cannot be appended to a new run:

```powershell
$resultRoot = Join-Path (Get-Location) 'target/spline-equation-actual-results'
if (Test-Path -LiteralPath $resultRoot) { throw 'Expected an unused result directory' }
New-Item -ItemType Directory -Path $resultRoot | Out-Null
try {
    foreach ($iteration in 1..2) {
        $env:BESSEMER_EQUATION_RESULT_PACKET = Join-Path $resultRoot "results-$iteration.bin"
        cargo test --test model_spline_equation_state matrix:: -- --test-threads=1
        if ($LASTEXITCODE -ne 0) { throw "Matrix process failed: $LASTEXITCODE" }
    }
} finally {
    Remove-Item Env:BESSEMER_EQUATION_RESULT_PACKET -ErrorAction SilentlyContinue
}
python -B -X utf8 docs/features/spline-equation-state/verify_actual_reference.py
```

The verifier copies Source before importing it. It compares both fresh-process
packets byte for byte, opens all 906 actual documents through the original
container and feature readers, requires complete feature EOF, and compares all
nine saved values. This checks stored roles; it does not execute the native
setter, expression evaluator or geometry providers.


## Physical occurrences and aliases

The [alias proof](alias-verification.json) covers two physical sketches with two
equation splines each, two shared-object aliases and a null root. Source checked
five complete independent graphs through EOF and primitive byte coverage. The
normal reproducer regenerated every earlier fixture and each new graph exactly.

Bessemer enumerated four physical states across the five roots. Each selected
lock edit matched its independent whole-graph golden, preserved every unselected
field and alias, reopened the complete document and reversed exactly. Eight
workers replayed 128 edits against those goldens without changing their source.
The full typed suite passed 17 tests. This adds owned alias acceptance; saved
version contexts beyond 19000, native outcomes and publication gates remain open.
