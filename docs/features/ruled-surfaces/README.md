# Ruled-surface interoperability

Feature #54 implements ordinary `moRuledSrfFromEdge_c` and
`moRuledSrfFromEdgeData_c` bodies from clause 11.229 / C-271, for external-XML
parts above version `0x27CB`. Native dispatch and ancestry establish both exact
classes. The feature reuses the complete reference-geometry and inline surface
base, then reads a direct object array and the pointed-to data body directly.
The data body adds no tag, presence word or map slot. The constructor's separate
embedded data at runtime `+7D8` and array at `+70` add no wire fields.

Data retains a required feature owner, two independently counted object arrays,
an independently counted u16 WORD array, distance/angular dimensions, signed
words, binary64 values, direction and three vector components in native order.
The owner accepts the feature base and the reviewed ruled-surface class; nulls
and unrelated cached objects fail. Dimension and direction pointers keep their
declared roles and stored nulls. Raw scalar meanings remain unresolved. Existing
array budgets and strict kernel validation apply before publishing results.

Independent fixtures cover 156 complete graphs across thirteen versions,
99,634 primitive observations and fourteen owned streams. They exercise direct
data and tagged data with complete feature owners, inline kernel bodies,
inherited version boundaries, owner cycles, shared dimensions, nulls, repeated
roots, independent array sizes, escaped counts, unsigned WORD high bits,
signed extrema and exact floating-point bits. Two edit goldens change one signed
word while retaining the other root, shared references and original inputs.
The actual scoped source readers verify EOF, map slots, identities and primitive
partitions under stable hashes. Their permissive expected-class arguments are
not role oracles; native declarations and Rust negative tests establish roles.

Eight private tests cover the complete oracles, every truncated prefix, extra
bytes, old contexts, direct-body framing, WORD widths, invalid/null owner roles,
unrelated dimension/direction classes, impossible counts and kernel context.
Three public tests cover complete document reconstruction and accepted/rejected
edits with input isolation. Regeneration uses
`python -B -X utf8 -m unittest discover -s docs/features/ruled-surfaces`.
Run `cargo test --lib ruled_surface_tests`,
`cargo test --test resolved_ruled_surfaces` and `cargo xtask pr`.

Historical/internal contexts, scalar semantics, geometric validity, application
acceptance and all-schema parity remain outside this evidence. The humanizer
skill is unavailable; maintainer prose review remains open.

The local PR gate passed with 809 library tests, all integrations and 274
doctests. Two fresh native processes agree on all 906 outcomes and causes. Cases
746 and 748 now reconstruct completely, raising the accepted set from 808 to 810
without losing an earlier success. The aggregate native assertion still fails
explicitly for 96 unsupported cases. An immutable source snapshot independently
checks all 810 accepted EOFs and four original stream hashes, covering
53,644,440 feature bytes. Under the same snapshot, all recorded prefix
root intervals/classes in the original ruled-surface configuration match; its
8,261-byte ruled root retains its exact hash, actual self owner, scalar/vector
values and independent counts. The old partial prefix receives no completion
credit. Reports are retained in `frontier-checkpoint.json`,
`accepted-reference.json`, `native-sentinel-reference.json` and
`local-verification.json`.
