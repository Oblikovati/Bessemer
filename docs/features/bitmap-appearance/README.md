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
successfully. These checks remain separate from the Rust gate. Native setting operations,
whole-document reversal, fresh-process exports, concurrency, full Cargo and
exact-head remote publication acceptance remain open. The earlier raw70/60/6C
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
Actual-output Source replay, native setting operations, whole-document checks
and integrated local/remote gates remain pending.
