# Ordinary component legacy references

Feature #206 reads the complete direct payload selected by every nonzero stored
`unknown_84` in ordinary component prefixes. The old signed field path remains
unchanged. The payload contains a generic tagged object pointer and one raw
`u32` `time_word`. The `moOldRef_c` wrapper has no enclosing tag or map slot.

Null and shared pointers keep their identities. A fresh supported target adds
only its own normal class/object slots, and later aliases resolve to that same
target. The time word retains high bits and the all-one sentinel without native
runtime widening, calendar conversion, presence clearing or temporary-object
cleanup. A nonzero presence magnitude does not select multiple wrappers.

The prefix/presence read is shared. Ordinary `base_fields` reads the payload;
Simulation's existing `base_head` continues to reject it. Historical Simulation
dialects, ordinary version admission and later prefix gates remain unchanged.
The separate direct reference-info branch is still unsupported. No global
tagged reader for `moOldRef_c` is registered.

The independent original-reader oracle contains 270 complete bare profiles and
27 complete owned streams. Ten versions span the shared-prefix gates; cases
cover every signed presence extreme, null/self/fresh targets, raw time extremes,
following-field alignment and aliases after the containing object. Both guarded
reference runs passed and reproduced every reviewed fixture byte. The expected
bare record/alias sequences reject every proper prefix; invalid pointers and
unsupported contexts also fail.

The pre-change public regression failed at the explicit nonzero legacy stop.
Its log remains retained. The malformed historical fixture that changed presence
without supplying a body still fails, now as invalid pointer framing. Existing
zero-presence baselines are retained unchanged. Bessemer's focused/native and
full-gate results are recorded separately; original-reader acceptance alone is
not typed implementation acceptance.

The implementation passed all three public tests and nine component tests,
including all 270 bare profiles, exact budgets and 12 complete document edits
with fresh reversals. All 188 older component cases retained their 5,402 expected
fields. Eleven Simulation tests passed and retained nonzero-legacy rejection;
one existing external-corpus test remains ignored. All 906 retained native
configurations passed forced typed reconstruction. Clippy and formatting passed.
`local-verification.json` records revisions, source pins, log hashes and pending
full-gate/platform/integration checks.

These generated positive bodies do not establish installed positive population,
native application opening, global semantic accounting or geometry validity.
Complete documented-schema and Corliss interoperability parity remains open.

```text
python -B -X utf8 docs/features/component-legacy-reference/freeze_reference.py
cargo test --test model_component_legacy_reference
```
