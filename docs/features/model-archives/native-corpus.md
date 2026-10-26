# Native part corpus acceptance

The latest [linear-pattern checkpoint](linear-patterns.md) accepts all 906
configurations across 842 document paths, following 903 at the
[fillet/thread checkpoint](fillet-threads.md) and 899 at the
[profile checkpoint](profiles.md),
following 893 at the [body-feature/plane checkpoint](body-features-planes.md),
891 at the [role-boundary checkpoint](remaining-role-fixes.md) and 890 at the
[sketch-role checkpoint](sketch-role-fixes.md). Earlier first-stop distributions remain
historical evidence. The
[dimension checkpoint](dimension-records.md) raised acceptance from 851 at the
[explosion checkpoint](explosion-records.md), 850 at the
[geometric/kernel checkpoint](geometric-kernel-references.md) and 834 at the
[group/relation checkpoint](groups-relations.md). The original distribution and measured runs
below are retained as historical evidence.

The first Bessemer run over the frozen reference-supported 906 configurations
rejected every case at a typed unsupported class. Permanent-mark records and
annotation views were the most common first stops. Implementing their complete
documented bodies raised exact reconstruction to 632 configurations from 585
documents. There are 842 document paths in the complete corpus. Accepted model
versions are 19000 (628 configurations) and 18000 (four).

Acceptance here means a full typed read with the matching configuration header,
exact encode, derived-size construction, schema/identity reparse and another
exact encode. The harness does not copy an unread remainder. It does not grant
corpus-wide independent scalar, edit/history, whole-document or native
application acceptance. Those remain required under #44.

The remaining 274 configurations stop at these first failures. Later failures
may appear after these serializers are implemented; this table is a work list,
not a complete count of missing classes.

| First stop | Configurations |
|---|---:|
| `moCompFace_c` | 75 |
| `moMateReferenceGroupFolder_c` | 44 |
| `moRelation_c` | 36 |
| `moLengthParameter_c` | 33 |
| `moFaceRef_c` | 30 |
| `moCompSolidBody_c` | 20 |
| `moEdgeRef_c` | 19 |
| `moCompSurfaceBody_c` | 14 |
| `moVertexRef_c` | 2 |
| Derived component-reference pointer role (#46) | 1 |

The new owned permanent-mark and annotation-view suite has 32 cases and 1,196
independent scalar expectations. It covers signed mark IDs, both compact matrix
branches, all four annotation suffix gates and the base view's late word. Every
shortened prefix and extra trailing byte fails. Two fresh reference processes
reproduced the fixture bytes. Nested classes still need their own complete
serializers; the owned annotation cases use null suffix pointers.

Prepare and run the local acceptance gate:

```powershell
python -B docs/features/model-archives/prepare_native_corpus.py
cargo test --offline --lib native_corpus_tests -- --ignored --nocapture
```

Preparation requires the original native files and the frozen reference report
at `target/current-part-model-reference-corpus.json`. It checks every model
stream hash, records each matching header hash, preserves the frozen version and
binds the entire input packet. The packet stays under `target/`; native files
are not added to the repository. The test is explicitly ignored in the normal
owned-fixture gate because these external corpus inputs are required. Running
it explicitly now passes its 906-of-906 assertion. Ignored and partial runs do
not count as full corpus acceptance.

`native-corpus-checkpoint.json` binds both measured runs, accepted indices,
first failures and the 32,844,304-byte input packet. Bug #46 tracks exact-name
pointer checks rejecting a derived reference class; its fix requires reviewed
inheritance evidence and negative tests. Bug #47 tracks errors that still name
`Config-0` for other selected configurations. Current result rows retain the
actual input label separately. Neither contract is declared complete.

The private codec checkpoint targets the candidate branch. Public operation
contracts remain in #44. Humanizer was unavailable; the prose pass remains for
maintainer review.

At the earlier expansion checkpoint, `cargo xtask pr --offline` passed with 120 library tests,
one explicitly ignored external-input test and 204 doctests. A subsequent
test-only helper extraction added a result-classification test for exact bytes,
mismatches and typed failures; that test and all-target Clippy passed. A fresh
process reproduced all 906 corpus result rows exactly, including the 274
rejections. Production sources did not change after the full gate.
`native-expansion-verification.json` binds both test-source states.
