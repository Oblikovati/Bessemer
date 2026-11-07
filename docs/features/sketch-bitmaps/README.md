# Sketch bitmap archives

Bessemer reads the complete ordinary sketch-bitmap feature and its nested data
object. It retains the model-feature base, signed 32-bit transform presence and
compact restore transform. Metadata follows the documented nonmonotonic field
order; reference presence is unsigned and selects both tagged pointer occurrences.

A nonzero plane-presence byte selects exactly width times height raw bytes.
The reader checks both signed dimensions, multiplication, remaining bytes and
the aggregate entry budget before reading the plane. Negative dimensions remain
stored metadata when no plane is selected. Each plane byte is an owned typed
field, so edits reconstruct the complete array without retaining source slices.
This framing does not assign pixel, alpha or channel meanings. Archive condition
`0x20` and legacy layouts remain outside the declared ordinary context.

Independent fixtures cover 60 graphs across two classes, five versions and six
value styles, with 5,899 primitive observations and 14 owned documents. They
exercise signed presence extremes, high unsigned bits, compact transforms,
six-byte and zero-byte planes, and two independent word-edit goldens. Five
focused Rust tests, three public save/reopen and isolation tests, and immutable
fixture regeneration passed. The native source regression checked a complete
containing-root span, metadata, point, string, plane hash and transform presence.

The primitive comparator checks every typed plane byte against its exact offset
in the independent reader's raw-array observation. Scalar kinds and complete
partition boundaries retain their existing checks elsewhere.

Two fresh native processes matched all 906 statuses and causes. Complete typed
reconstruction increased from 858 to 861 configurations, adding indices 675,
717 and 739 without regressions. The aggregate corpus still fails because 45
configurations remain incomplete. The independent reader confirmed exact EOF
for all 861 accepted inputs, covering 69,268,963 resolved bytes and all four
source-stream hashes. The full `cargo xtask pr` gate passed formatting, Clippy,
build, 1,022 library tests, all integration tests and 277 doctests. Three
external-fixture tests remain ignored by the default gate.
Whole part, assembly, drawing and project-schema parity
and native application acceptance remain unfinished.

The humanizer skill was unavailable; its prose review is left to the maintainer.
