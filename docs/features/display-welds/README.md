# Display welds

Feature #54 implements the complete ordinary `moDisplayWeld_c` body documented
in 11.345 / C-387 and its guarded supported composition in 11.347 / C-389.
The installed `slddetailsu.dll` primary table `0x180085A00` selects serializer
`0x180047730..0x180047ADD` and proves display-item ancestry. The reader reuses
the complete existing display-object and display-item bodies, then consumes
three f64 vector components, raw f64 +5A8, signed i32 +5D8, CString +5B8 and
generic tagged object pointer +5C0. Every native load gate holds in the supported
modern ordinary context. Runtime callbacks and repair add no archive fields.

This is a display item with a weld suffix. It has no display-dimension text,
units, attachments or dimension-specific suffix. Existing dimension-only roles
continue to reject the class. The payload pointer follows the native generic
object operator and retains null, self, fresh and cached identities.

Independent fixtures cover 162 complete graphs, 32,724 primitive observations
and 21 owned streams. Nine versions cross inherited display-item point/vector
gates; six identity styles cover distinct roots, repeated roots and shared
payload objects. Three raw-value sets retain signed zero, NaN payloads,
infinities, subnormal values and signed-word extrema. Strings retain short
ANSI, Unicode with embedded NUL and a surrogate pair, and a 260-unit Unicode
CString. Three edited goldens change only the signed weld word.

Seven private tests check all independent fields, slots and bytes, every
truncation and extra byte, old context rejection, suffix extents, exact numeric
bits, separation from display dimensions and inherited component-role rejection.
Three public tests verify complete document reconstruction, isolated edits and
input isolation on success and rejected edits.

Run `cargo test --lib display_weld_tests`, `cargo test --test resolved_display_welds`
and `cargo xtask pr`. Regenerate with
`python -B -X utf8 -m unittest discover -s docs/features/display-welds`.
Other symbol/component classes, historical/internal contexts, numeric meanings,
geometry validity, application acceptance and whole-schema parity remain
incomplete. Humanizer is unavailable; maintainer prose review remains open.

The final local PR gate passed 859 library tests, all integrations and 274
doctests. Two fresh native processes matched all 906 outcomes and causes,
retaining all 818 complete reconstructions. Cases 547, 615 and 905 advance to
an unimplemented component-detail reference; none receives completion credit.
The other 88 inputs remain explicit failures, so both aggregate corpus commands
exited 101. Independent readers verified EOF for all 818 accepted streams,
91 recorded prefix roots across three native source graphs and nine native
weld payloads, including exact numeric bits, signed words and stored strings.
