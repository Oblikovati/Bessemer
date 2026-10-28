# Additional valued-dimension interoperability

Feature #54 implements the exact native bindings in clause 11.343 / C-385:
`sgSplineTanMagnitudeDim` uses the existing G1 body, `sgArcLenDist` the G3 body,
and `sg3DPlane3DPlaneDist` the G2 body. Each primary Serialize slot selects its
base serializer's identical address, and native RTTI independently proves the
stated ancestry. These classes add no derived wire suffix. The stored class
identity is retained rather than renamed to a base class.

The existing G1/G2/G3 collection guards admit their reviewed concrete subclass
while retaining all earlier members, nonnull entries and unrelated-class checks.
Shared entity handles, plane handles, fixed archive transforms and dimension
identities reuse the existing complete readers. The plane-distance transitional
u16 remains present only when `0x2F79 < version < 0x2FA0`. Arc-length quadrant
words retain zero and 65535 without emulating runtime normalization.

Independent fixtures contain 306 complete graphs across seventeen versions,
56,073 primitive observations and 21 owned streams. They include standalone
classes, populated/null handle fields, repeated handles, distinct equal roots,
repeated first roots, complete sketch collections and cached seeded dimensions.
Three independent edit goldens change the common `dcm_type` word while retaining
other roots and source inputs. The earlier 442 builder hashes remain frozen;
their explicit pre-C-395 replay is separate from current positive expectations.

The actual complete reference readers and local additional-class installer
verify EOF, map slots, primitive partitions and object identities. Native primary
slots/ancestry establish serializer reuse; no class binding is inferred from its
name. Eight private tests check the native regression, complete oracles, every
prefix, extra bytes, old contexts, minimal suffix lengths, actual class identity,
the transitional word, raw quadrants, cached identities and rejected collection
entries. Public document tests cover exact reconstruction and accepted/rejected
edits with input isolation.

Run `cargo test --lib additional_valued_dimension_tests`,
`cargo test --test resolved_additional_valued_dimensions` and `cargo xtask pr`.
Regenerate against stable sources with
`python -B -X utf8 -m unittest discover -s docs/features/additional-valued-dimensions`.

Historical/internal contexts, unresolved numeric meanings, geometric validity,
application acceptance and whole-schema parity remain outside this evidence.
Humanizer is unavailable; maintainer prose review remains open.

The local PR gate passed with 831 library tests, all integrations and 274 doctests.
Two fresh native processes agree on all 906 statuses and causes. Cases 152 and
774 now reconstruct completely, raising the accepted set from 814 to 816 without
losing an earlier success. Case 343 reaches a later explicit
`moSkIsoCurveInfo_c` stop and receives no completion credit. The aggregate native
assertion still fails for 90 unsupported cases. A verified immutable source
snapshot independently checks all 816 accepted EOFs and four original stream
hashes across 54,306,681 feature bytes. Under the same snapshot, all
recorded root spans, class identities, unknown counts and hashes match across the
five original configurations, including all five newly bound dimension objects.
Original partial prefixes receive no completion credit. Reports are retained in
`frontier-checkpoint.json`, `accepted-reference.json`,
`native-dimension-reference.json` and `local-verification.json`.
