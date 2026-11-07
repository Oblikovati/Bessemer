# Archive-backed dimension state

The proposed behavior profile is `dimension-stored-state-v1`. It adds stored
value projection and edits while keeping the existing decoder, raw field paths,
accepted archive versions and complete field-edit behavior unchanged. No file
migration or package version change is part of this work.

The view will own the signed mask, signed redundant identifier and revision-bound
handles for both physical words. It must preserve the raw mask alongside the
bend-angle flag, three-bit counter and unresolved bits. A selected field must be
the I32 mask in a dimension-base group: adjacent same-scope I32
`am_object_id`, `is_design_table_dimension`, `mask` and `unknown_5c` fields.
This signature comes from the existing typed decoder. It distinguishes the
dimension group from tolerance masks and unrelated fields without restricting
projection to a short list of most common dimension classes. Direct and nested
dimension bases need the same check. Existing archive revision validation runs
before projection or replacement construction.

The edit input will select one stored role: bend-angle range flag, fillet
reference-point change number or redundant identifier. Counter inputs zero
through seven replace only bits 2 through 4; values above seven fail with the
offending value and expected range. Flag edits change only bit 1. Identifier
edits replace only the adjacent signed I32. Zero counters remain zero when
requested, independently of the native incrementer's transition. There is no
implicit native setter, range enforcement, unit conversion or handle lookup.

Both configuration and resolved-feature entry points must delegate complete
replacement fields to their existing edit operation, including its validation,
coverage and ordered structural history. A no-op preserves bytes and identity.
A changed result gets a distinct revision and the existing derived relation.
Source and result views remain immutable; stale and foreign handles fail before
any replacement is published. Missing, mismatched or wrong-scope fields fail
projection instead of being accepted as an empty or partial dimension.

Acceptance must check every owned physical definition and all unselected fields
against the original primitive partition. Literal-offset edits provide full
expected images independently of the Rust implementation. Tests must cover both
flag values, all eight counter values, all remaining u8 counter requests as
failures, signed identifier limits and exact no-ops. Scope/type guards, inherited
configuration isolation and loaded-class/instance-alias identity remain part of
the operation. Configuration edits also need complete document reopening,
unrelated stream preservation and byte-exact reversal.

All 906 retained native configuration and feature carriers must reconstruct
without changing the earlier gate. The typed projection needs its own complete
denominator, including nested dimension bases, with original Source values or
independent raw-word comparisons. Published types need compile-time Send/Sync
checks. Serial, concurrent shared-snapshot and two fresh process results must
agree in full bytes, fields and history, with expected failures included.
These checks establish file-operation acceptance; they do not certify native
application behavior or geometry.

The humanizer skill is unavailable; its prose pass remains for maintainer review.
