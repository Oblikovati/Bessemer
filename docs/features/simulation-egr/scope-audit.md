# EGR schema scope audit (#124)

The existing `simulation-egr-flat-v1` implementation covers the documented
conditional operations in specification14.514–14.522. This audit adds no runtime
behavior and does not turn the conditional profile into a complete native EGR
schema. The exact clauses remain in [schema snapshot](schema-snapshot.md).

| Clause | Existing implementation | Checked scope |
|---|---|---|
|14.514 |[names](../../../src/simulation/egr/names.rs) |All33 known names, matched-null source, unknown/no-write and bounded destination projection; no GEN field inference |
|14.515 |[row](../../../src/simulation/egr/row.rs),[types](../../../src/simulation/egr/types.rs) |Explicit32-by-four-byte loaded profile, complete128-byte rows and conditional second-word solver code; no filename-based transport assumption |
|14.516 |[decode](../../../src/simulation/egr/decode.rs),[header](../../../src/simulation/egr/header.rs) |Complete selected table framing, header equations, exact endpoint and raw regions |
|14.517 |[profile](../../../src/simulation/egr/types.rs),[archive](../../../src/simulation/egr/mod.rs) |Caller-established constructor-bound flat CFile profile at byte zero; no invented transfer prefix |
|14.518 |[allocation](../../../src/simulation/egr/allocation.rs),[identities](../../../src/simulation/egr/ids.rs) |Unique live mappings, holes, deleted links, complete live/deleted partition and unallocated capacity |
|14.519 |[groups](../../../src/simulation/egr/groups.rs),[row](../../../src/simulation/egr/row.rs) |Stored group identifier, eight signed parameters, wrapping arithmetic, output-only permutation and explicit skipped/incomplete name states |
|14.520 |[decode](../../../src/simulation/egr/decode.rs),[ranges](../../../src/simulation/egr/ranges.rs) |Loaded positive widths and cumulative field starts; raw tags and row tails retain unknown meanings |
|14.521 |[descriptor projection](../../../src/simulation/egr/descriptor.rs) |Only tag-one/original-width assignments, retaining original descriptors and alignment; complete native save remains false |
|14.522 |[ranges](../../../src/simulation/egr/ranges.rs),[edits](../../../src/simulation/egr/edit.rs) |Revision-bound existing-live-row ranges, exact-width isolated edits, complete history and typed Missing for holes |

The current combined library passed all18 focused tests, including the explicit
export test, with zero failures or ignored cases. Two subsequent fresh processes
also passed all18 tests and produced the same3,693,682-byte output as the
unchanged historical baseline. The corpus includes14 complete owned tables,
3,992 field ranges/edits/no-ops,183 solver-name contexts and936 EGROUP contexts.
Thirty public types retain Send/Sync assertions. See
[process verification](scope-audit-process-verification.json).

The current frozen Source replay passed all eight checks with `--verify-rust`,
including comparison of the entire emitted Rust packet against the actual
reader functions. All three reader hashes match the earlier proof. The fresh
[Source verification](scope-audit-source-verification.json) and snapshot remain
separate from the unchanged historical snapshot and baselines.

All25 EGR-specific runtime/test/fixture hashes match the original complete proof.
Two surrounding support files changed afterward: `simulation/mod.rs` added
other module declarations/reexports, and `.gitattributes` added other fixture
globs. Their diffs do not change EGR operations or its binary fixtures. No hash
exception was added to a reader, verifier or baseline.

Native complete EGR fixtures remain zero in the three-file retained census;
that census does not establish global absence. Native cache/save behavior,
complete field meanings and universal transport/context selection remain
unproven. The descriptor assignment projection is not a native writer round trip.

MaterialList14.458/14.512–14.513 is a separate remaining problem, tracked in
[feature175](https://github.com/Oblikovati/Bessemer/issues/175). Its70 verified
header prefixes and exact candidate partitions retain `complete=false`;
installed identifier spellings do not bind the historical body serializer.
They receive no completion credit from this EGR audit.

Humanizer is unavailable; maintainer prose review remains pending.
