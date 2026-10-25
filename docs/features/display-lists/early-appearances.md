# Early appearance and display-item layouts

The appearance work admitted display versions 5000–19000 except the uncounted-window
interval `0x1FAE`–`0x1FCE`. This extends the complete-stream codec; it does not
establish full clause 13 parity or native application acceptance.

The [earliest scene work](earliest-scenes.md) later extended the lower bound to 4850.

The owned vectors contain 58 part/assembly roots with 18,804 independent field
expectations and 58 bucket roots with 9,721 expectations. Every fixture declares
native file context 19000. Appearance fixtures populate RV properties in both
document kinds; the blank assembly originally omitted that optional body.
The generator inserts an independently encoded body before converting the layout.

The roots bracket RV default dimensions `>0x1415`, LXP filename `>0x1421` and
edge rounding `>0x1462`; visual-property word `>0x144B`; node flags `>0x143A`;
face identity `>=0x1401`; parabola word `>0x13EC`; simple-text integer only at
`0x1448` and `0x1449`; center-of-mass fields `>0x146E` and `>=0x1473`;
and the bucket cross-hatch scalar `>0x147D`. Absent fields remain absent. Signed
integers, stored floating-point bits and string representations survive forced
reconstruction and validated edits.

Independent scalar expectations include each stored face identity in both kinds,
in addition to material, scene, item, component and bucket field traces. Tests
cover exact budgets, independent ambient edits, source isolation, reopening,
version boundaries and every truncated prefix at 5000, `0x1448` and `0x1449`.

Run `cargo test --test display_earlier_range --test display_earlier_buckets` and
`python -B -X utf8 docs/features/display-lists/verify_early_appearance.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe`.
The process verifier regenerates frozen fixtures, then compares forced bytes from
two isolated Rust processes. No geometry-validity claim follows from these checks.

The humanizer skill was unavailable; prose remains subject to maintainer review.

Validation on Windows x64 with Rust 1.99.0 passed `cargo xtask pr`, including all
176 documentation tests. All 116 new fixtures rebuilt byte for byte in two
processes, and every earlier root, bucket and file-context process check passed.
The native audit retained 1,086 accepted documents and 1,849,563 independent
field comparisons with unchanged corpus hash
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
Cross-platform CI remains pending the complete feature PR.
