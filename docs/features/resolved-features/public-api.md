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
