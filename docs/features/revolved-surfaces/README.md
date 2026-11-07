# Revolved reference surfaces

Bessemer reads, edits and writes `moRevolvRefSurf_c`, `moRevolvSurfData_c` and
`moRevolvRefSurfIdKeeper_c` in ordinary non-DB external-XML part archives newer
than `0x27CB`, with archive flag `0x10` clear and the supported shared surface
context. Specification 11.175 / C-203 binds all three native primary dispatches,
their bases and the revolved class's zero-valued inline-surface selector.
`native-ancestry-reference.json` retains the evidence and its source hash.
All three installed module hashes were checked against the supplied evidence.

The feature reuses the complete reference-geometry and inline surface readers.
It then stores a nullable data pointer and an exact `moLineRef_w` axis pointer.
A nonzero backend marker does not externalize the stored sheets. The data record
keeps two angular-dimension pointers, each followed by its own signed word,
then a nullable revolution end specification. The pointers may share a cached
dimension. Native post-load cleanup does not remove fields from the archive.

The keeper has no ordinary binary body. A tagged keeper still reserves its
class/object map slots, while native allocation of an absent runtime keeper
adds no archive bytes. Explicit inherited keepers and the complete inherited
chooser owner are accepted for this concrete surface class. Other keeper or
owner names receive no inferred support.

The independent reference reader consumed 253 complete graphs through exact
EOF, producing 61,722 primitive observations. The corpus covers seven versions,
the `0x2BCD` surface-word and `0x2F85` reference-geometry boundaries, all three
classes, empty/fresh/shared/rich graphs, signed-word extremes, and distinct equal
data and keeper objects. It includes nonzero backend markers, suppressed sheets
and extra bodies, shared inline attributes, and a complete typed serialized
kernel body. Fifteen containing streams begin with the actual document-derived
map of 97 slots. Two are independent one-word edit goldens.

Seven private tests compare every primitive span, emitted byte and map count;
reject every incomplete prefix and trailing byte; enforce the ordinary-context
boundary; check each declared pointer and the nonnull inherited keeper; and
verify the 299/14/zero-byte modern minimum bodies. The axis test explicitly
rejects a cached temporary-axis subclass, matching the documented exact role.
The kernel test requires an explicit schema and rejects invalid sheet counts.
Four public tests reconstruct every containing document, match independent edit
goldens, preserve equal-but-distinct records and shared references, test source
mutation isolation, and preserve inputs after rejected field-type or missing
kernel-context operations. Two Python tests regenerate all new fixtures;
21 prior regeneration tests preserve the reused geometry builder's default path.

The source tests report null angular dimensions in the eight native roots;
populated dimension identity is independently exercised by synthetic fixtures.
Raw mode meanings, geometric validity, application acceptance, historical/DB and
special contexts, and complete documented-schema parity remain unproven.
No dependency, exported Rust API or package version changed. The humanizer skill
is unavailable; its prose pass remains with the maintainer.

Two fresh native processes agree on every outcome and cause across all 906
configurations, with a stable executable hash. They preserve all earlier 720
complete cases and complete five more. Three others advance to later explicit
dependencies and receive no completion credit. The remaining 181 cases retain
failures, so the broad complete-corpus assertion still fails. An independent
immutable snapshot of 1,192 reference Python modules verifies all 725 accepted
streams through exact EOF: 44,067,070 feature bytes, with all four original input
hashes checked per configuration. The temporary source copy and audit script
were removed after their process terminated. Reports retain the snapshot
identity, source hashes, original case hashes, outcomes and exclusions.

The local `cargo xtask pr` gate passed: 586 library tests, four xtask tests,
all integration tests and 265 doctests. Three native opt-in tests remain ignored
in the default gate; both complete-corpus replays above ran explicitly.
