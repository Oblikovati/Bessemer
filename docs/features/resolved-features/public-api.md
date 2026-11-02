# Ordinary part resolved-feature API

`PartResolvedFeaturesArchive` reads a supported complete
`Contents/Config-N-ResolvedFeatures` stream. `from_document` resolves the selected
model header and configuration once, reads complete CMgr state with its own
object map, then parses the feature stream using the configuration's inherited
map. The ordinary CMgr runtime layout is 19000; model wire versions remain
separate. Unsupported classes or contexts return typed errors, never partial
successful archives or opaque tails.

`from_bytes` accepts feature bytes, an owned `PartConfigurationArchive`, CMgr
bytes and explicit limits. The caller supplies these matching inputs together.
The archive owns its context and fields; later changes to caller buffers do not
change published state. Clone shares immutable state. `to_bytes` reconstructs
and reparses all fields against that retained context before returning bytes.

`fields`, `root_count` and `object_slots` expose the stored graph. Map counts
include reserved null slots. `coverage` reports exact stored-archive checks;
its imported-slot count refers to the selected header imports, while
`object_slots` reports the complete inherited model map and final feature map.
`diagnostics` explicitly excludes geometry validation and external-document
checks. A successful wire pass is not native application acceptance.

Run the example against the owned baseline:

```powershell
cargo run --example rebuild_part_resolved_features -- tests/fixtures/blank-part.SLDPRT 19000 0 target/rebuilt-owned-part.SLDPRT
```

The example replaces only the selected feature stream. It supplies no kernel
schema, so documents requiring serialized kernel bodies fail explicitly. Library
callers can provide `CompressedKernelContext`.

`edit_fields` validates an owned complete replacement against the retained
context. Its result returns completion, coverage, diagnostics, exact stored-field
fidelity and structural history together. `field_handle` and `resolve_field`
bind occurrences to a revision. Changed results have new revisions; unchanged
results and clones preserve handle ownership. Failed edits leave inputs intact.
Created, deleted and derived field relations are structural, not geometric history.

The `edit_resolved_plane_token` example demonstrates an isolated stored-token
edit on the owned Config-0/version-19000 fixture and checks independent expected
bytes. Logical document naming stays an application policy; the example does
not apply the separate header naming policy established by the native matrix.

Owned integration tests establish exact feature bytes, 17 roots, inherited/final
map counts of 97/161, complete document no-op reconstruction, input ownership,
typed failures and deterministic concurrent reconstruction. Compile-time checks
assert Send and Sync. Full documented schema parity, broad native acceptance
and race-detector runs remain separate requirements. The humanizer skill is
unavailable in this session; its prose review remains with the maintainer.

## Error context

After the model context is resolved, feature-operation failures identify
`part resolved features` and the selected `Contents/Config-N-ResolvedFeatures`
stream. Failure classes and causes
are retained, including CMgr dependency locations. See the
[error-context regression](../resolved-feature-errors/README.md) for coverage.

## Current retained-corpus replay

The November2 replay accepted all906 retained cases in two fresh Rust
processes. Each case requires complete private/public typed reconstruction,
public field equality, complete configuration decoding and byte-for-byte CMgr
and feature reconstruction. Both processes produced identical outcome rows. All nine public
owned tests also passed, including isolated edits, complete document
reopen/revert and unrelated-stream preservation.

[Retained-corpus verification](retained-corpus-refresh-verification.json)
records the exact packet, binaries, source hashes and terminal logs. The packet
matches its original input manifest; this was not fresh extraction or a new
Python-source run. Earlier failed Rust checkpoints and reference-only results
remain historical evidence. Other archive/version contexts, source-wide parity,
native application and geometry acceptance remain separate requirements.

The unchanged current Python regression was also replayed from frozen sources.
It failed a known-byte accounting assertion before completing its corpus loop:
84,979 observed bytes versus51,395 in the pinned audit for the first stream.
The other two original tests passed. [Current source verification](current-source-verification.json)
retains the failed run, exact inputs and immutable snapshot; Bug
[#177](https://github.com/Oblikovati/Bessemer/issues/177) tracks the discrepancy.
The asserted906-stream aggregate is not a measured passing source result.
No parser, expected result or historical baseline was changed to hide it.

A separate diagnostic mode measures every original row without replacing that
failing test. Run `freeze_current_resolved.py --measure` for the measurement;
the default command continues to run the original tests. Two fresh diagnostic
processes completed all906 frames and produced byte-identical outcomes. Every
case's model header, configuration, CMgr and feature hash matches the Rust
retained-input manifest:3,624 exact stream hashes.

All906 known-byte counts differ from the older audit. The current measured
sum is75,659,073 bytes,21,758,736 above its53,900,337-byte expectation.
[Measurement verification](current-source-measurement-verification.json)
retains every row, both source snapshots and logs. This establishes the scope
of Bug #177; it does not review or adopt the changed semantic accounting.
The original failure and historical baselines remain intact.
