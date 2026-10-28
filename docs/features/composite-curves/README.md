# Composite-curve interoperability

Feature #54 implements the complete ordinary `moCompositeCurve_c` body in
clause 11.236 / C-278 for external-XML parts above version `0x27CB`.
The exact native dispatch calls the same reference-curve base used by helices:
reference geometry, strict kernel wire-body framing, a signed wire-node word
and a direct attribute list. The existing base implementation is reused.

The suffix retains a signed word, one nullable tagged `suObArray` pointer,
one direct object array and four direct DWORD arrays. Only the tagged array
introduces an object-map identity. All five direct counts remain independent;
DWORDs retain unsigned high bits. Existing count and remaining-byte budgets
apply before reading entries. No scalar or array-element meaning is inferred.

Independent fixtures contain 132 complete graphs across eleven versions,
30,384 primitive observations and fourteen owned streams. Present and suppressed
kernel bodies, inherited version gates, rich owners, self cycles, shared cached
arrays, nulls, duplicates, escaped counts and unequal array sizes are covered.
Two independent edit goldens change one signed word in the first of two
distinct roots, preserving their shared array and repeated first-root reference.
The actual scoped source reader verifies EOF, byte partitions, map slots and
object identities. Its permissive expected-class parameter is not a role oracle;
native dispatch/ancestry establishes the exact class, and Rust rejects a cached
curve supplied to the tagged array role.

Six private tests cover all observed fields and bytes, every truncated prefix,
extra bytes, old contexts, independent unsigned arrays, invalid roles, impossible
counts and required kernel context. Three public tests cover complete document
reconstruction, independent edits and input isolation after accepted/rejected edits.

Run `cargo test --lib composite_curve_tests`,
`cargo test --test resolved_composite_curves` and `cargo xtask pr`.
Regenerate with `python -B -X utf8 -m unittest discover -s docs/features/composite-curves`
against a stable approved source tree.

Historical/internal contexts, scalar semantics, geometric validity, application
acceptance and all-schema parity remain outside this evidence. The humanizer
skill is unavailable; maintainer prose review remains open.

The final local PR gate passed with 801 library tests, all integrations and 274
doctests. Two fresh native processes agree on all 906 outcomes and causes. Cases
611 and 747 now reconstruct completely, raising the accepted set from 806 to 808
without losing an earlier success. The aggregate native assertion still fails
explicitly for 98 unsupported cases. An immutable source snapshot independently
checks all 808 accepted EOFs and four original stream hashes, covering
53,294,976 feature bytes. Under the same snapshot, all recorded root
intervals/classes in the two original composite configurations match; the three
composite roots retain their exact hashes, scalar, independent counts and present
kernel state across 163,300 bytes. Reports are retained in
`frontier-checkpoint.json`, `accepted-reference.json`,
`native-sentinel-reference.json` and `local-verification.json`.
