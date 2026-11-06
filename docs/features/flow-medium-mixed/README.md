# Complete mixed-kind medium records

Issue: #241. Reference: `../inventor-interface/solidworks/flowmediumliquids.py`.

The FML1 packet freezes 3,753 complete record-424 profiles. It includes the
installed kind-one record and every earlier complete kind-one case, all active
outer versions 2 through 127, all six kind-two model layouts, both first-table
orders and duplicate physical entries. Additional cases cover every stored
state byte, the metadata string corpus, populated nested arrays, kind-two-only
bodies, first tables without references and empty reference lists of either kind.

Each first-table entry stores a kind, a sixteen-byte table key and its complete
body. The packet embeds independently decoded FMO1 or FMO2 fields rather than
only recording opaque body bytes. Rust and Python rebuild all nested fields,
check every entry boundary and compare the complete record byte for byte.

Reference bodies select the first-table map by their declared kind. Equal keys
may occur in both maps, but a key present only in the other kind cannot resolve
a reference. Kind-one reference bodies store a state byte; kind-two bodies do
not. Outer references resolve against the combined second table. Duplicate
physical entries and repeated ordered references remain present; these checks
do not assign a native duplicate-key lookup identity.

Python checks all proper prefixes of compact examples for each model in both
table orders, appended bytes, cross-kind and missing outer references, invalid
count/tag/version framing, populated final collections and explicit fresh-state,
active-outer and string-mode requirements. The packet is regenerated in a
temporary directory to verify reproducibility.

The production entry point is `FlowCarrier::record_424_mixed_profile`. Pass
`FlowMediumMixedOptions` containing the existing fresh-profile options plus
independent per-body and total raw 32-bit-word limits. All configured bounds
must be positive. Body bytes, wide patterns and name units retain their existing
per-body and total limits; empty collections spend zero budget.

`first_objects()` borrows ordered immutable entries with independent table keys
and a `FlowMediumBody` enum containing the complete kind-one or kind-two view.
`reference_objects()` retains each declared kind, ordered typed references and
an optional state byte. Kind two has no stored reference state. Coverage reports
`ExactFreshMixedRecord`, complete supported framing and zero serialized error,
alongside the explicit engineering and native exclusions.

Existing key, reference, state and attribute edits reparse the whole record.
Changing a key cannot leave a dangling reference in its typed domain. The
profile also composes existing metadata and either body edit, checking the
current checkpoint and selected body offset first. Completed edits retain the
unchanged source, complete result and physical carrier history. Tests cover
all existing profile fields and base/state composition for every nested body,
with exact no-ops, independently patched carriers and fresh reversals.

The 77 focused Rust checks include all earlier regressions, complete FML1 field
projections, eleven independent bound axes, compact proper prefixes and malformed
framing, stale/foreign composition, ten public Send/Sync types and eight-worker
read/edit transcripts after overwriting input buffers. Two fresh processes match
an independently expected transcript of typed entry projections and edited
carriers; the complete numeric field oracle is checked separately. Measurements
in `api-cost-verification.json` compare the existing kind-one reader with the
mixed reader on their common accepted corpus, with explicit workload limits.

Installed kind-two acceptance, engineering/model evaluation, native runtime
behavior and application acceptance remain unverified. The complete profile
requires an explicit fresh backend and zero final collection counts, matching
the documented reader's accepted scope.
