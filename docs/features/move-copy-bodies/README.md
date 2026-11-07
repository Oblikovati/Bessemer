# Move/copy body archives

Bessemer composes the complete ordinary `moMoveCopyBody_c` body from its
inherited body feature and the documented move/copy suffix. Dimension pointers
and raw signed words follow their separate `0x3B0B` and `0x42CC` gates.
The nested payload retains shared wrapper references, a direct object array,
the fixed 30-word archive transform, points, vectors, stock and plane pointers,
and the direct mesh-tolerance helper in native field order.

The ordinary direct pointer array contributes zero bytes and no map slot.
Conformal data appears after `0x2F69`. Positive and negative zero scales retain
their original bits; flag `0x200` adds two more zero-scale multivectors.
Nonzero or NaN scales, function-object flag `0x100`, and undocumented special
transform kinds return typed unsupported errors with the offending value.
Native ownership fixups are not applied to persisted values.

The independent reader accepted 162 graphs across three classes, nine versions
and six value styles, with 30,708 primitive observations and 20 owned documents.
The fixtures include independent feature-word and mesh-word edit goldens.
Six focused Rust tests and three public save/reopen, edit and input
isolation tests passed. The pinned source reader regenerated every fixture,
and its native containing-root regression checked retained payload fields.
The optional-multivector regression exercises both branches with nonzero and
NaN scales.

Two fresh native processes matched all 906 statuses and causes. Exact typed
reconstruction increased from 851 to 854 configurations, adding 130, 354 and
631 without regressions.
The independent source reader confirmed exact EOF for all 854 accepted inputs,
covering 68,387,843 resolved bytes and all four source-stream hashes.
The aggregate corpus still returns failure because 52 configurations remain
incomplete. Whole-schema parity, geometric semantics
and native application acceptance remain unfinished.

The full `cargo xtask pr` gate passed: formatting, Clippy, build, 1,013 library
tests, all integration tests and 277 doctests. Three external-fixture tests
remain intentionally ignored by the default gate.

The humanizer skill was unavailable; its prose review is left to the maintainer.
