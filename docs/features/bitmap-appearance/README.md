# Stored bitmap color and ink settings

Feature [#189](https://github.com/Oblikovati/Bessemer/issues/189) implements
published section 11.1657 / C-3341. Canonical schema1 bitmap definitions
expose signed raw64 matching-color-percentage, unsigned raw68 transparent
color and signed raw80 ink-markup texture settings. The ink word is absent
before version0x2BD3 and the typed view reports that absence explicitly.
All bits are preserved. No percentage bound, packed color format, Boolean
restriction, effective transparency or runtime refresh is inferred.

`bitmap_appearance_states()` enumerates physical definitions in wire order.
`bitmap_appearance_state()` binds the same leading raw70 identity used by
the existing canonical transparency admission. `edit_bitmap_appearance()`
changes one stored word and returns the existing complete archive result,
including diagnostics, exact stored coverage and derived history. Foreign
and stale identities are rejected before mutation. An ink edit on an archive
without raw80 returns a typed error instead of adding a field or a default.

Preparation matched all 24 publication pins and retained the original report
byte for byte. The native span proof covers twelve setting words in four
canonical bodies, 48 bytes jointly. No new accounting credit is claimed.

Independent literal fixtures contain 825 complete streams with two physical
owners and a shared alias each. Signed extrema, unsigned color boundaries,
three plane-presence encodings and distinct owner values cover the three
supported complete-context versions11000,18000 and19000. The historical
contexts use hash-bound installed original files and their actual imported
map slots and MBSM eligibility; the current context uses the owned blank part.
Raw bitmap tests separately retain the exact0x2BD3 body boundary. The fixture
freezer rewrites bytes only with its explicit initialization flag; strict
regeneration compares every byte without rewriting the fixtures.

The initial current-context gate passed 11,250 edits over all375 owned streams.
The corrected four-test gate passed all825 streams and24,000 edits, plus typed
foreign/stale rejections, absent-ink errors, schema qualification and compile-
time Send/Sync checks. Every edit matched a literal whole-stream patch and
complete expected fields, history, coverage and diagnostics. Input archives
remained unchanged. Historical-context tests require the explicit installed
kernel schema and original pinned files and remain ignored in ordinary runs;
the owned current-context and schema tests run with `cargo test`.

The first fixture attempt paired historical feature versions with modern
configuration bytes and failed before reaching the new API. The next attempt
retained an ineligible MBSM tail for historical contexts and failed exact EOF.
The corrected fixtures use the independently selected supported configuration
versions and actual per-context map/tail metadata. Both failed logs are
retained; no production parser, manager admission or assertion was relaxed.

Fresh original Source validation passed all825 complete streams against an
immutable copy, with4,800 scalar checks and39,275 primitive reads. Class/map,
alias, primitive-partition, exact EOF and signed/unsigned expectations matched.
The frozen Source content and import-origin audit passed; the process exited
successfully. These checks remain separate from the Rust gate. The later
operation, document and full-gate results are recorded below. The earlier raw70/60/6C
transparency roles and later SDK transparency contracts remain separate work.
The humanizer skill is unavailable; the prose pass is left to review.

The existing transparency suite passed all seven ordinary tests against the
appearance-enabled library. Both native transparency tests also passed after
sharing the kernel fixture helper:108 archive requests and12 document edits
with12 exact reversals. Their independent span, field, stream and whole-file
assertions were preserved.

Two fresh six-test processes passed the mixed concurrency gate. For all825
streams, each process compared43,200 serial/worker visits and216,000 changed,
unchanged, foreign and stale requests. The owned current-only gate added
20,250 visits. Every worker matched full bytes, fields, coverage, diagnostics,
completion and resolved history. The final library and test Clippy checks and
function-size audit passed.

Their24,000 primary edit exports match byte for byte:19,680 changes and4,320
no-ops. An independent checker accepted every complete literal patch, version
frame and exact EOF. The export hash and all process/log evidence are retained.
Both actual-output Source replays passed all24,000 outputs, with all171
consumed modules identical between the immutable snapshots. Complete
graph/map fingerprints, floating-point bits, spans, EOF and original unknown
totals and sketch/detail signatures matched the independently patched baseline.

Native byte binding independently matched twelve setting words in four
canonical bodies, including all four NEW_CLASS headers and absent alpha
planes. Ordinary installed-module verification matched seven named exports,
nine code ranges totaling2,746 bytes, all21 retained leaf instructions,
the actual primary serializer slot and six typed archive-operator calls.
These are PE/disassembly checks, not REA Evidence or native target execution.
The initial disabled Capstone-detail attempt is retained in the proof; its
correction changed no expected instruction, export or operator binding.

Two fresh native-file processes each passed60 archive requests:52 changes
and8 no-ops. All complete outputs matched independent literal patches and
each other byte for byte. Twelve document edits and12 fresh typed reversals
per process preserved every unrelated stream and restored the complete
original containers through their retained builders. Both native actual-output
Source replays passed all60 outputs, including complete graph/map/floating-
point fingerprints, exact spans and EOF, and unchanged accounting signatures.
All170 consumed Source modules matched between the two immutable snapshots.

The whole-document fixture matrix passed in two fresh processes. Each covered
4,800 forward edits and4,800 fresh typed reversals across all825 streams.
The owned current-context gate added2,250 edits and2,250 reversals. Reopened
fields and bytes, every unrelated stream and its order, complete original
container restoration and source isolation all matched. Native and document
modules are now wired into the ordinary Cargo integration target, with the
installed-input cases explicitly ignored unless selected.

The full local `cargo xtask pr` gate passed at6102fb2f379fe2787685bc1c0852d73053de865a:
3,956 tests passed, zero failed and212 were ignored across315 summaries.
That source cut predates the newly wired test modules; the final source gate
remains required. Exact-head remote publication and ThreadSanitizer, native
application/writer equivalence and whole-schema parity remain unproven.
