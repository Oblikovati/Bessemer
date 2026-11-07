# Historical complete display roots

Feature #28 now admits display versions 6000–19000 except the documented
uncounted-window interval `0x1FAE`–`0x1FCE`. Earlier versions and unresolved object
bodies remain outside the proven schema coverage. Native application acceptance
and complete clause 13 parity remain unfinished.

The owned fixtures independently encode 48 complete part/assembly roots with
14,223 field expectations and 48 complete bucket roots with 8,271 expectations.
They bracket every 6000-range gate in the reference document, tessellation,
light/scene and UI readers: sky shader `0x17DD`; component view type `>0x17F3`,
assembly feature type `>0x17F5` and belt flag `>0x183D`; circle/sphere geometry
`>0x1827` and the circle's final double `>=0x1842`; floor flattening and size
`>0x1827`, height `>0x1841`, accurate lighting `>0x185E` and transform `>0x186C`;
pen word `>=0x185A`; halo-repair list `>0x184C`.

Generators consume the reviewed modern source completely, record nonoverlapping
spans for absent fields and restore all reference-reader hooks afterward. Both
whole-stream readers must reach EOF. Independent scene, material, display-item,
component and bucket traces compare stored values; forced reconstruction compares
every byte. Layout omissions stay absent rather than becoming default fields.
The bucket budget omits four aggregate entries when its single halo polygon is
absent: one list entry and three coordinates. Empty lists contribute zero.

Tests exercise each boundary, exact and one-short budgets, every truncated prefix
of the oldest part and assembly, independent ambient edits, reopening, source
mutation and concurrent edits. Synthetic contexts use native file version 19000;
this does not prove SolidWorks accepts arbitrary edited geometry.

Run `cargo test --test display_earlier_range --test display_earlier_buckets` and
`python -B -X utf8 docs/features/display-lists/verify_historical_roots.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe`.
The latter regenerates all frozen fixtures and compares two isolated Rust processes.

The humanizer skill was unavailable; prose remains subject to maintainer review.

Validation on Windows x64 with Rust 1.99.0 passed `cargo xtask pr`, including all
176 documentation tests. The corrected lower-context test passed separately.
All 96 historical fixtures matched forced bytes in two processes; all earlier
root, bucket and file-context process checks passed unchanged. The native audit
retained 1,086 documents and 1,849,563 independent field comparisons, with corpus
hash `94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
Cross-platform CI remains pending the complete feature PR.
