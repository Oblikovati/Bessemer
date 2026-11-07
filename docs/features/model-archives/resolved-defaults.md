# Resolved managers and detail defaults

This checkpoint predates the [complete detail-definition body](complete-details.md).
The remaining-work list below records this checkpoint's original scope.

The private codec now reads the opening resolved managers, detail-default
prefix, character format and complete dimension-default group. The schema
snapshot records the applicable clauses from edition 0.306. This work remains
under #44 and does not export a complete model API.

| Owned native record | Stream span | Bytes | Independent scalar values |
|---|---|---:|---:|
| Resolved opening through embedded array | 2925..3003 | 78 | 11 |
| Detail-default prefix and character format | 3003..3790 | 787 | 143 |
| Ten dimension-default bodies and group prefix | 3790..8954 | 5164 | 931 |

The resolved size word at 2921..2925 declares 21,103 body bytes. Size reading
rejects negative and out-of-stream lengths. Bounded reads temporarily restrict
the cursor while retaining the same fields, scopes, failure state and aggregate
budgets. They restore the outer cursor after success or failure and require the
bounded body to reach its exact end. Tests cover nested and empty sections,
overreads, underreads, impossible sizes, ignored inner errors and a field budget
exhausted across successive sections.

The lightweight reader uses this same boundary mechanism. A composed native
test keeps one builder and one MFC map through byte 8,954. The relation manager
receives native object slot 56. Starting it with a fresh map produces different
identities even though its scalar bytes can parse; the tests check the actual
identity. Nested length-unit pointers in the dimension definitions use this
retained map, and the native group rejects a fresh map.

The atom manager rejects null atoms. Atom modes 0 and 2 retain their stored
fields, including the mode-2 body reference; embedded geometry in mode 1 fails
explicitly. Ghost records preserve all packed-count bits, duplicate integer
values, point coordinates and noncanonical presence words. Relation-manager
lists preserve aliases and cycles, and its cache preserves duplicate names and
stored double values. Unimplemented new nested classes remain unsupported.

Independent schema-built cases cover all 16 ghost flag nibbles, both supported
atom branches, relation-manager version gates and cycles, both detail-grid
layouts, string arrays, character-format gates and all ten dimension kinds.
The three suites contain 26, 22 and 11 cases with 400, 1,451 and 8,951 expected
scalar values. Rust reconstructs every case exactly and rejects every truncated
prefix and trailing byte. Source overwrites and four concurrent encoders also
leave the owned detail and dimension results unchanged. Two fresh reference
processes reproduced all nine fixture artifacts exactly.

The scalar oracles contain stored fields. Reference-only positions, definition
kind/member labels and unknown-byte totals are excluded. Ghost count and flag
views are derived from the preserved packed word and add no serialized fields.
The reference manifests retain source and generator hashes. No existing frozen
display or model baseline was changed.

Dimension defaults reuse the existing units grammar and a shared line-width
reader extracted from the display codec. The extraction changes only the
context accessor; a mechanical body comparison records that equivalence.
Existing display fixtures continue to gate its signed-short and custom-float
behavior. All character-format and raw numeric fields retain their stored
values before any native font, width or style normalization. Unknown member
meanings receive no semantic or geometry-validation claim.

The composed native test still has 15,074 unread bytes and requires final
publication to fail. Annotation, table, label and other detail definitions,
the detail tail, remaining model/part stages, derived-length full construction,
history, document isolation, corpus acceptance and native application checks
remain open. No prefix is registered as a complete `moDetailDefs_c` decoder.

No version, release, extra branch or PR is included. The humanizer skill is
unavailable; its prose pass remains for maintainer review.
