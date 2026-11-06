# Flow record 432 stored state

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord432.py`.

`FlowCarrier::record_432_stored_state` reads the complete raw record for every
nonnegative signed version. It preserves the three signed members in physical
order: native offsets `0x5e0`, `0x5d8`, then `0x5dc`. Versions one and two
also contain one discarded signed member. Complete payload lengths are twenty
bytes for those two versions and sixteen bytes for all others. Truncations,
extensions and negative versions fail instead of producing partial results.

`FlowStoredStateMember` names the existing stored slots. `replace_member`
patches a copy, completely reparses it and returns both typed checkpoints with
carrier history. The version is structural and has no edit operation. Editing
an absent discarded member fails. No-ops retain the original checkpoint;
reopening and reversing a changed member restores the complete carrier bytes.
Duplicate and unrelated records remain unchanged.

The reference marks versions zero and one for native recomputation of the
three members. The view exposes that version rule and preserves the original
stored values. It does not run recomputation or claim that the stored values
are the eventual native runtime values. Coverage reports exact raw framing,
member counts and zero serialized approximation error. Diagnostics retain the
native runtime, engineering and SolidWorks application exclusions.

The source-derived FST2 baseline is owned under
`tests/fixtures/flow-state-tables/record432.bin.gz`. Its 746 cases include the
audited native CPT payload, nonnegative version bits and complements, and
signed numeric patterns. Source hashes and complete raw field expectations are
recorded in `../flow-state-tables/baseline-verification.json`.

Run `cargo test --test flow_state_table_fixtures` for source comparisons,
independent signed-slot expectations, full edits, no-ops, fresh reversals,
malformed input, limits, occurrence identity and input isolation checks.
Published types are checked for `Send + Sync`; concurrent workers edit shared
immutable inputs independently. Eight workers each perform 32 edits and compare
their complete carrier bytes with serial results. Two fresh processes replay
all source-checked states and edits byte for byte. ThreadSanitizer coverage is
pending the next required CI matrix; the earlier open PR does not include this API.
