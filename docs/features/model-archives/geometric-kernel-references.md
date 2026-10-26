# Geometric references and embedded kernel frames

Feature #44 now has private readers for eight documented geometric reference
classes. The owned packet exercises 74 cases and 4,788 independently projected
scalar values, including old and modern versions, optional frames, null
references and signed frame-mode boundaries. Every packet reconstructs exactly;
truncated and trailing input fails. The scalar oracle excludes compound kernel
contents, which have a separate typed oracle.

Embedded kernel frames require an explicit Parasolid base schema and resource
limits. The reader checks the frame length, type, compressed carrier, neutral
partition and exactly one BODY record. Expanded bytes and records consume the
operation's aggregate budgets. Suppressed bodies do not need a schema. Unknown
kernel encodings fail; no opaque body fallback is retained.

The codec reconstructs the complete typed transmit before encoding. Existing
compression bytes remain reusable when the validated contents are unchanged;
an edit replaces the expanded chunk and derives both carrier and frame lengths.
Owned tests cover independent expected edits, source isolation, parallel writes,
malformed frames, missing contexts and exhausted budgets. Bug #50's regression
also checks a length-changing edit reports its new expanded size immediately,
before encoding or reparsing.

Five native frames contain 236 records and 8,351 independently decoded value and
metadata rows. They match the reference oracle and reconstruct exactly with the
explicit installed `sch_13006.s_t` schema. Its hash, the source inventory and the
external packet hash are recorded in `native-kernel-frame-reference.json`.
Neither that installed schema nor the native input files are redistributed.

Bug #49 required an explicit embedded-partition title context: these native
frames use titles without a `(partition)` marker. The default transmit API
retains its strict title rules. Three context regressions and 28 existing
transmit integration tests passed. Contradictory delta titles still fail.

The native configuration corpus reached 850 exact reconstructions out of 906,
up from 834 at the preceding checkpoint. Its 56 remaining first failures are
dimension handles, an exploder and the two tracked role-validation failures.
This remains a failed acceptance gate. Kernel checks establish archive structure
and typed values; they do not establish geometry, topology or native application
acceptance. Public model APIs and full operation-result contracts remain pending.

All seven owned kernel tests passed after the #50 fix. The final-source
`cargo xtask pr --offline` gate passed: Clippy, build, 164 library tests,
all 76 integration suites and 204 doctests. Its two external-input tests were
run separately with the native kernel and corpus outcomes described above.
The pure Python oracle suite contains 50 passing tests. The captured clauses
come from edition 0.340. Humanizer was unavailable;
the prose pass remains for maintainer review.
