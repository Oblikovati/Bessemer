# Component-detail references

Feature #54 implements the exact `moDetailItem_c` base and the internal
`moCompDetailItem_c` branch documented in the note-data clause and
11.345 / C-387, 11.348 / C-390 and 11.349 / C-391.
Behavior version: `C390-component-detail-v1`.

The detail-item primary table `0x4CEF9578` selects `0x4BA5EC30`. Its modern body
contains the existing lightweight node, u16 +48, generic owner +40, direct
attachment array +50, signed-count i32 array +60, i32 +78 and tagged attachment
+80. The direct array has no class header or mapped list object. Scalar values,
attachment order and all shared identities are retained without geometry claims.

The component-detail table `0x4CEF4C00` selects `0x4C956B10`. The internal branch
contains the complete existing component-reference prefix, two tagged detail
pointers +E8/+F8 and signed word +100. A null component selects this branch
without a current-part query. For a nonnull component, loading requires an exact
unit component, stored external-model presence zero and owner identity equal
to the selected current part. Other classes, external flags, null/different
owners, missing stored facts and missing root provenance fail explicitly.

A private root token is issued only by a completed parsed part archive. Its
identity comes from the validated new root tag and is passed with that archive's
inherited fields/map into resolved-feature decoding. Ordinary archive mode,
a class name or owner-shaped fields cannot issue this token. Fresh unit fields
are resolved locally; cached unit facts are read from the same complete saved
source. No global or thread-local owner state is used.

Independent fixtures cover 108 complete bare graphs, 6,682 primitive observations
and 28 owned streams. Nine inherited version boundaries and six identity styles
retain shared detail items, attachment pointers, repeated roots and null details.
Owned cases separately exercise null components, fresh current-part units and
cached inherited units. Four edit goldens change only the selected signed word.

Private tests compare all fields/slots/bytes, every prefix and extra byte, old
contexts, negative integer counts and unrelated detail/attachment roles. Focused
provenance tests verify the actual completed root, absent proof, wrong/null
owners, external presence, unrelated classes and inherited fields without proof.
Public tests verify complete document reconstruction, isolated edits and input
isolation; a wire-valid source with a null unit owner rejects the cached-unit
branch while preserving the independent null-component branch.

Run `cargo test --lib component_detail_`,
`cargo test --test resolved_component_detail_references` and `cargo xtask pr`.
Regenerate with
`python -B -X utf8 -m unittest discover -s docs/features/component-detail-references`.
Other detail subclasses, external component layouts, historical/internal contexts,
runtime repairs, geometry validity, application acceptance and whole-schema
parity remain incomplete. Humanizer is unavailable; maintainer prose review
remains open. The earlier [design note](design.md) and prefix-refactor evidence
record the prerequisite separately from this behavior addition.

The final local PR gate passed 874 library tests, all integrations and 274
doctests. Two fresh native processes matched all 906 outcomes and causes,
retaining every one of the 818 complete reconstructions. Cases 547, 615 and 905
now reach unimplemented weld-symbol detail subclasses and receive no completion
credit. All 88 remaining inputs fail explicitly; both aggregate corpus commands
therefore exited 101. Independent readers verified EOF and four-stream hashes
for all 818 accepted configurations, 91 recorded prefix roots across three source
graphs and nine native component-detail objects. Eight used a verified current-part
unit and one used the independent null-component branch.
