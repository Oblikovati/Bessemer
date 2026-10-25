# Early display roots and view collections

Complete part and assembly roots cover display versions 7000–19000 except
`0x1FAE`–`0x1FCE`. Those versions store an uncounted window sequence that the
reference also rejects; they continue to return an explicit unsupported error.
This interval admission does not establish complete clause 13 schema parity.

The early fixtures bracket named-view, spatial-view, annotation-view, old
annotation-ID, annotation-string and simple-text layout transitions. There are
76 full or minimal roots with 23,692 independent field expectations and 70 view
bucket roots with 12,559 expectations. A separate full-root oracle compares
28,338 nested named, spatial and annotation view fields. The minimal roots
include 100 consecutive 12-byte annotation records at versions `0x1C4A` and
`0x1C4B`; collection budgets charge actual stored entries.

Bug #37 affected already admitted version 8000. A complete-stream regression
failed before the reader retained the temporary float present for
`0x1C04 < version < 0x1FB5`. The fixtures retain signaling NaN bits `0x7F800042`
and independently encode an edit to `23.0`. Tests compare edited bytes, reopen
them, and verify that the original archive remains unchanged.

Run `cargo test --test display_earlier_range --test display_earlier_buckets` and
`python -B -X utf8 docs/features/display-lists/verify_early_views.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe` from the
repository root. The process check regenerates frozen fixtures with the sibling
reference, then requires identical reconstruction from two Rust processes.
These synthetic layout checks do not establish SolidWorks application acceptance.

The humanizer skill was unavailable; prose remains subject to maintainer review.

On Windows x64 with Rust 1.99.0, all 146 new fixtures rebuilt identically in two
processes. The existing transition, component, configuration and file-context
process checks also passed. The native audit retained all 1,086 accepted documents
and 1,849,563 independent field comparisons, with no missing files or failures.
The accepted corpus manifest hash remains
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.

`cargo xtask pr` passed, including 176 documentation tests. The final additional
malformed-input regression passed separately, followed by a clean all-target
Clippy check. Cross-platform CI has not yet run for this checkpoint.
