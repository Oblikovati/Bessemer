# Resolved-feature root snapshots (#201)

`PartResolvedFeaturesArchive::root_graph` exposes every physical root in stored
order, including nulls and aliases. Each occurrence retains its revision-scoped
field handle, exact object tag, class/schema and loaded target identity. Local
targets identify their physical defining tag, including nested definitions;
inherited targets retain the map identity loaded by the configuration.

The snapshot owns the complete immutable source and copied physical-root history.
It returns unchanged completion, source schema coverage, exact stored fidelity,
zero disagreement in copied values and three explicit semantic exclusions. An
empty list completes unchanged. Root selection rejects foreign, changed-revision
and nonroot field handles. This is stored list and object-map structure; it does
not infer feature dependencies, evaluation order, geometry or external documents.

## First complete operation

The owned five-root graph has two sketch definitions, two aliases and one null.
Its literal tag words are `FFFF, 0062, 0000, FFFF, 0064`; local object identities
are 98 and 100 after the inherited 97-slot map. The earlier frozen original Source
check in [spline equation state](../spline-equation-state/alias-verification.json)
verified these complete graphs through EOF, primitive coverage and shared-object
identity. The new snapshot matches the independent root-order and tag literals,
retains the complete source bytes and links aliases to their defining fields.

The focused tests also cover an inherited configuration object, a nested local
handle definition, empty lists, foreign/stale selectors, source isolation and
128 concurrent snapshots. Five public types are checked for Send and Sync.
The first nested test incorrectly chose a fixture containing only a root handle;
the retained failed log identifies that choice. The final test uses the existing
array fixture with an actual nested definition; no production fallback was added.

## Remaining acceptance

Fresh original Source exports must compare all projected root metadata and links.
The complete retained native corpus, resource/failure matrices and combined local
and exact-head remote gates remain required before publication. Higher-level
part/assembly/drawing model views and structural edits remain open in #2. This
read operation does not provide geometry or a structural edit stub.


## Native structural gate

The [native proof](native-verification.json) covers all 906 retained carriers and
26,789 roots: 984 inherited targets and 25,805 local targets. This corpus contains
no null roots; the owned graph supplies that case. Every physical tag, resolved
class/schema, defining-field link and selector passed, while all configuration
and feature bytes stayed unchanged. Two processes produced identical case and
total records, retained losslessly in `native-case-results.tsv.gz`.

This comparison uses Bessemer's admitted fields and loaded map identities. It does
not replace the pending independent Source metadata comparison or establish
feature dependency order, geometry or native application behavior.


## Reproduce actual native metadata exports

The manual native test can write the actual projected root values, including
framing, record kind, object identity, class/schema and local/inherited target
identity. It keeps implementation-specific field indexes and revision tokens in
the Rust ownership tests; these are not Source wire values.

Use an unused result directory and the existing retained BRF1 packet. Each Cargo
invocation starts a fresh process. The kernel schema remains an explicit input:

```powershell
$resultRoot = Join-Path (Get-Location) 'target/resolved-root-actual-results'
if (Test-Path -LiteralPath $resultRoot) { throw 'Expected an unused result directory' }
New-Item -ItemType Directory -Path $resultRoot | Out-Null
try {
    foreach ($iteration in 1..2) {
        $env:BESSEMER_ROOT_RESULT_PACKET = Join-Path $resultRoot "results-$iteration.bin"
        cargo test --lib every_retained_native_root_preserves_physical_tags_and_loaded_target_links -- --ignored --nocapture
        if ($LASTEXITCODE -ne 0) { throw "Native export process failed: $LASTEXITCODE" }
    }
} finally {
    Remove-Item Env:BESSEMER_ROOT_RESULT_PACKET -ErrorAction SilentlyContinue
}
python -B -X utf8 docs/features/resolved-root-graph/verify_actual_reference.py
```

The verifier copies Source before importing it. A profiling observer records
original object-method calls, reader dispatch and returns; it leaves every
method and decoded result unchanged. It retains the defining archive and object
identity so memory-address reuse cannot misclassify inherited objects, and it
captures class/schema before a body can reference itself. The owned null/alias
graph checks those observer roles separately from the native population.

This is a scoped comparison of stored root metadata and loaded identities. It
does not replace or weaken the independent known-byte accounting test in #177,
which remains unresolved. Source field/geometry meanings and native application
behavior remain separate requirements.


The [actual-output proof](actual-output-verification.json) records a completed
comparison of all 26,789 native roots with the frozen original Source reader.
Both reference tests passed, including complete feature EOF and the import/content
guard over 6,673 Python files. The two fresh Rust packets each contained 1,508,863
bytes and matched byte for byte. This closes the scoped stored-metadata comparison;
the known-byte accounting defect, publication gates and broader model/geometry
requirements remain open.


## Malformed streams and source budgets

The [boundary proof](boundary-verification.json) records all 3,686 proper
prefixes and all 256 single-byte append variants of the owned alias stream being
rejected before root publication. The accepted source and its snapshot remain
unchanged after those failures. Exact byte and field caps pass, while one below
either cap fails. The focused suite passed 11 tests. These additions cover those
two source resource dimensions; other decoder caps retain their existing tests.
