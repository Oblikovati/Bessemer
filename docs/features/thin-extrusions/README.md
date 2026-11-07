# Thin extrusion and revolution features

Feature #54 implements the exact `moExtruThin_c`, `moBossThin_c`, `moCutThin_c`,
`moRevolutionThin_c` and `moThinSpec_c` bindings in clauses 11.191 / C-219,
11.207 / C-235, 11.208 / C-236 and 11.236 / C-278. Scope is ordinary external-XML
parts above version `0x27CB`. The composite-curve portion of C-278 has its own
existing reader and is outside this change.

The three extrusion variants reuse the complete ordinary extrusion body, then
read a required thin specification and a plain u16 bend count. Each bend record
contains three direct u32 values. The count is bounded against complete remaining
12-byte records and the aggregate budget before reading entries. `0xFFFF` means
65,535 records; it never introduces a wider count. Order, duplicate triples and
all unsigned bits are retained. Thin revolutions reuse the complete ordinary
revolution body, then read their required specification and end immediately.
There is no revolution bend count or array.

The thin specification retains two independent display-dimension pointers,
its exact signed-word/double wire order and a separately typed radial dimension.
Raw options do not suppress dimension pointers. Doubles retain negative zero,
NaN payloads and infinities. Native owner updates and historical migrations
consume no additional modern stored fields and are not executed offline.

Independent fixtures cover 360 complete graphs across twelve versions, 256,702
primitive observations and 35 owned streams. They include empty/populated
inherited bodies, cached owners, shared dimensions/specifications, distinct roots,
repeated root identities, raw scalar extrema and five isolated edits. A separate
786,424-byte selected payload proves all 65,535 bend triples through the actual
source reader and records its exact hash. It is a count-boundary fixture, without
whole-document completion credit.

All graph fields, bytes and map slots are checked. Continuing prefix regressions
cover all thirty latest-version class/style combinations; the inherited readers
retain their own exhaustive version-boundary corpora. An initial local run also
checked every prefix of all 360 graphs. Budget tests cover the full u16 count,
bytes, fields and aggregate entries. Owned-document tests verify reconstruction,
shared-target edits and input isolation after successful and rejected edits.

Concurrent source changes invalidated three direct freezes. The retained fixture
reports instead bind a verified immutable copy: source hashes before copying,
copied hashes and source hashes after copying agree, and copied hashes remain
unchanged through generation and independent regeneration. Source module-origin
checks remain enabled. `source-pin-reference.json` identifies the approved source
and snapshot. The source reader's permissive expected-class argument is not a
pointer-role oracle; native bindings and negative Rust tests provide that check.

Run `cargo test --lib model_archives::thin_extrusion_tests`,
`cargo test --test resolved_thin_extrusions` and `cargo xtask pr`.
Fixture regeneration uses
`python -B -X utf8 -m unittest discover -s docs/features/thin-extrusions` against
a stable source tree; the recorded run used the retained hash identity's immutable
copy to avoid concurrent edits.

The local PR gate passes with 774 library tests, all integration tests and 274
doctests. Two fresh native processes agree on all 906 outcomes and causes. Fourteen
inputs now reconstruct completely, raising the accepted set from 786 to 800 with
no earlier success lost. Case 300 reaches a later `moOneBend_c` stop and remains
uncredited; the aggregate corpus assertion still fails for the 106 unsupported
inputs. The same immutable source pin independently verifies all 800 accepted
EOFs and four original stream hashes, covering 51,449,131 feature bytes.

Sixteen original configuration replays reproduce all recorded root intervals and
classes through nineteen containing roots' thin specifications, dimension
classes, raw words/doubles and bend triples. Recorded thin-cut root hashes also
match. These source sentinels establish selected framing without claiming that
partial prefixes complete in Bessemer. Reports and exclusions are retained in
`frontier-checkpoint.json`, `accepted-reference.json`,
`native-sentinel-reference.json` and `local-verification.json`.

Historical contexts, engineering meanings, geometry validity, application
acceptance and full schema parity remain open. The humanizer skill is unavailable;
maintainer prose review remains open.
