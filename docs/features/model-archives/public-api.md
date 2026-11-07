# Public part configuration operations

`model_archives::PartConfigurationArchive` reads the complete lightweight and
resolved sections of an ordinary modern part configuration. The caller declares
the model wire version and the ordinary external-XML/current-runtime dialect.
The runtime layout is the documented 19000 branch; model wire versions remain
independent. Drawing, assembly, route-assembly, legacy runtime overrides and
separate feature archives are outside this API.

`ConfigurationId` retains an unsigned configuration number. Its stream parser
accepts canonical `Contents/Config-N` paths and rejects signs, leading-zero
aliases, suffixes and overflow. `from_document` reads that configuration's actual
header once, checks the header/model outer versions and applies explicit stream,
field, object, depth and aggregate-entry budgets. Lower-level `from_bytes` takes
an immutable `PartConfigurationContext`; its header must belong to those bytes.

Stored fields retain their physical order, raw integer and floating-point bits,
CString encodings, transforms and resolved shared-map identities. Imported model
slots resolve to a header reference paired with its owning immutable header.
`from_fields` derives section lengths and checks the complete typed reparse.
Unsupported nested bodies fail the operation; no unresolved remainder is
published. Serialized kernel bodies require `CompressedKernelContext` with an
explicit base schema and expansion/record budgets. Suppressed bodies need none.

`edit_fields` validates under the source budgets and returns
`PartConfigurationEdit`. Its result includes changed/unchanged completion, exact
stored-field fidelity, whole-stream coverage, explicit geometry/external-document
exclusions, unchanged-field counts and structural field history. Changed fields
are derived, created or deleted by path occurrence. History does not infer
geometric merges, splits or healing. Every parent/result handle is revision-bound;
clones and equivalent edits retain handles, while changed edits and fresh decodes
do not. Published snapshots are immutable and `Send + Sync`.

Errors identify the selected stream, including failures before its first field.
Construction mismatches report the first differing field, supplied/expected
paths, types and compact object identities. Caller-provided diagnostic text is
bounded; kernel payloads are not dumped. The source remains isolated after both
successful and failed edits.

The runnable [name-edit example](../../../examples/edit_part_configuration.rs)
targets the owned ordinary Config-0/version-19000 baseline:

```powershell
cargo run --example edit_part_configuration -- tests/fixtures/blank-part.SLDPRT target/bessemer-model-name-edited.SLDPRT "Bessemer Ω"
```

It edits the root name, writes a complete document and reports the checked stream
length. The independent expected configuration is 24,038 bytes. Tests reparse
that result, reopen the whole document, preserve unrelated streams and revert
exactly. Public API tests also cover all five budget categories, header ownership,
stale handles, failed edits, source overwrite and concurrent edits.

The first public-API corpus run reconstructs all 906 frozen streams exactly.
Native application acceptance is narrower: SOLIDWORKS 2026 revision 34.2.1 opened
the owned baseline and edited part read-only with zero errors/warnings. Both had
17 features, zero bodies and one configuration. Neither file was saved; their
hashes stayed unchanged, and the same three pre-existing documents remained open.
`public-native-application.json` records that measurement. It is not a geometric
certification or a native-open result for the entire corpus.

Humanizer was unavailable; prose remains for maintainer review.
