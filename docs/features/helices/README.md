# Helices and wire-edge identifiers

Bessemer reads, edits and writes `moHelix_c` and `moWireEdgeIdRep_c` in ordinary
non-DB external-XML part archives newer than `0x27CB`. Specification 11.184 /
C-212 binds the helix and reference-curve primary dispatches, ancestry, direct
list constructor import, strict threshold and sign mask. It also binds the
wire-edge representation serializer to the existing common representation body
and one signed word. The supplied evidence is retained with its source hash;
the installed `sldmodu.dll` hash and exact threshold/mask bytes were checked.

The helix uses the complete reference-geometry base, an explicit suppressed or
typed serialized kernel wire, a retained signed node ID and a direct attribute
list. The list and inline helix data add no class tags or object-map slots.
Variable pitch stores five independently framed dynamic double arrays. Width
zero omits the descriptor remainder; width eight retains two metadata words,
its own count and raw floating-point bits. Other widths and impossible extents
fail explicitly through the existing typed array reader.

Any nonzero variable-pitch byte selects those arrays. The dimension table is
selected only by byte value one; noncanonical bytes remain stored unchanged.
The optional main dimension uses strict `abs(raw_28) > 1e-6`, so equality and NaN
omit it. The original bits remain stored, including signed zero and NaN
payloads. Each table row has four dimension pointers, followed by one terminal
pointer only when the table is nonempty. Null, fresh and cached dimension
references remain independent occurrences.

Wire-edge identifiers retain complete feature and external-feature references
and their signed suffix. Fixtures include real containing helices, shared
references, distinct equal identifier objects and repeated aliases through
declared entity-representation owners. The helix's inherited chooser owner is
admitted only through its documented concrete reference-geometry ancestry.

The independent reference reader consumed 243 complete graphs through exact
EOF and produced 110,808 primitive observations. The corpus covers three
versions, 14 exact threshold bit patterns crossed with four table flags, eight
additional pitch/wire/list scenarios with separate equal helix objects, and
three identifier modes across the full signed-word extrema. Fifteen owned
streams start with the complete document-derived map of 97 slots, including
two independent one-word edit goldens.

Eight private tests compare every primitive span, byte and map count; reject
every prefix and trailing byte; check the old-context boundary; use an
independent integer-order threshold oracle; reject invalid array/table extents
and wrong declared pointer classes; check minimum bodies; and require an
explicit kernel schema for present wires. Five public tests reconstruct all
containing documents, compare isolated edits with independent goldens, preserve
equal records and aliases, and verify input isolation after source mutation or
rejected type, missing-context and dependent-branch edits. Two Python tests
regenerate all fixtures and observations.

The native source report contains seven helix roots with decoded present wires,
absent variable pitch and absent dimension tables. Populated optional branches
are independently exercised by synthetic fixtures; no native sample credit is
claimed for them. Synthetic kernel fixtures establish stored framing without
claiming valid wire topology. Native geometric regeneration, raw mode meanings,
historical pitch migration, special contexts, application acceptance and full
documented-schema parity remain unproven. No dependency, exported Rust API or
package version changed. The humanizer skill is unavailable; its prose pass
remains with the maintainer.

Two fresh native processes agree on every outcome and cause across all 906
configurations, with a stable executable hash. All earlier 725 complete cases
remain complete. Seven configurations advance from the helix to a later
unsupported component-reference curve; none receives additional completion
credit. The remaining 181 cases retain explicit failures, so the broad
complete-corpus assertion still fails. An independent immutable snapshot of
1,197 reference Python modules verifies all 725 accepted streams through exact
EOF: 44,067,070 feature bytes, with all four original input hashes checked per
configuration. The temporary source copy and audit script were removed after
their process terminated. Reports retain the snapshot identity, source hashes,
original case hashes, outcomes and exclusions.

The local `cargo xtask pr` gate passed: 594 library tests, four xtask tests,
all integration tests and 265 doctests. Three native opt-in tests remain ignored
in the default gate; both complete-corpus replays above ran explicitly.
