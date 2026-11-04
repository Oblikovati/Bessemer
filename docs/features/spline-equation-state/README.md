# Stored spline equation state (#199)

The current codec preserves equation fields but does not expose their documented
stored roles together. Feature #199 will add a complete archive/document
read-edit-reopen operation over those fields. The original equation runtime and
geometry work in #2 and #54 remains open.

## Wire roles and decision

Use the role bindings in the [retained API/runtime audit](../spline-equation-transform/equation-api-runtime-schema-audit.json)
and its source snapshot. Keep raw `ArchiveString` values for slots 0–5: saved X,
Y and Z expressions, declared-character text, and range-start and range-end text.
These are saved strings, not converted BSTRs or evaluated doubles.

Retain `unknown_1e0` as a full signed32 option despite the API Boolean formal.
Keep the preceding `equation_word` separate and unchanged unless a later explicit
contract admits its edit. The selected equation-path start/end locks use
`flags_32c` masks `0x10` and `0x20`. They are not F4 flags. Each lock edit replaces
only its selected mask and preserves the equation-presence bit and every other
stored bit. No native setter cascade, update trigger or transform is simulated.

## First complete operation

Reuse `PartResolvedFeaturesArchive` field handles, validation, edit history and
document replacement. Bind a physical spline inside its actual sketch definition
and collection; a matching path suffix alone is insufficient. Resolve selection
and saved version once. Require the equation-presence guard and exact adjacent
field roles/types before publishing a state. Enumeration of a valid archive with
no equation splines completes empty; a wrong or foreign selection fails explicitly.

Build the first owned complete feature graph with the existing independent
`WireMap`, ordinary sketch builder and direct spline components. The existing
single-span fixture wrapper demonstrates the complete sketch/feature observation
route; reuse that route rather than inventing an archive stub. Observe the full
result through a frozen original Source copy with EOF and primitive coverage.
Prove read, one selected replacement, document reopen and exact reversal before
extending the matrix.

## Required acceptance

Compare raw strings and option/flag images with literal independent expectations.
Cover both supported direct spline prototypes and saved-version contexts,
duplicate physical occurrences, aliases, absent equations, foreign/stale handles,
full signed option boundaries and raw string encodings. Keep complete unselected
fields, stream bytes and history unchanged. No-ops retain identity.

Return completion, diagnostics, declared validation scope, history and achieved
stored-value error through the existing operation contracts. Assert public types
are Send/Sync, prove input isolation after success and failure, compare eight
workers with serial results, and compare complete fresh-process exports against
Source. Preserve native corpus outcomes and all earlier fixtures. A full local
PR gate and exact-head remote checks precede publication. Publication remains
serialized behind PR #191; this work creates no additional branch or PR.

Runtime expression parsing/evaluation, public API string/double conversion,
provider contexts, transform interpretation, loaded-object reuse and native
geometry acceptance are separate requirements. Their absence is not covered by
this stored-state operation.
