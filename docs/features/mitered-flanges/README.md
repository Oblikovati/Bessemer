# Mitered-flange archives

Bessemer composes the complete body-feature and bend-group prefixes before the
ordinary mitered-flange suffix. It retains the profile, dimensions, nonmonotonic
scalar order, three direct object arrays and two direct DWORD arrays. Direct
members add no container tag or object-map slot.

Four independent handle groups reuse the existing signed-count helper. A zero
count stores no sketch pointer. A positive count stores one sketch occurrence
followed by that many tagged handles, preserving nulls, repeated identities and
the original sketch assignment before native ownership repair. Negative counts
and counts impossible within the remaining bytes fail before iteration.

The independent fixtures cover 54 graphs across nine versions and six value
styles, with 35,945 primitive observations and seven owned document fixtures.
They exercise complete inherited bodies, populated and empty direct collections,
all four handle groups and an independent trailing-word edit golden. Four focused
Rust tests, three public save/reopen and isolation tests, and immutable fixture
regeneration passed. The source reader's two-file native regression also passed,
checking source/root hashes, scalar values, collection classes, DWORD values,
handle counts, sketch classes and complete list endpoints.

Fixture generation initially detected a changing upstream source tree and
rejected the run. The retained fixtures were regenerated from an immutable copy.
The updated source reader observes inherited packed-array payloads as raw blocks;
the comparator checks each retained 64-bit word at its exact offset in that block.
Dedicated regressions prove byte order and reject changed words. Other scalar
kinds and partition boundaries keep their existing checks.

Native inputs exposed two pointer-role mistakes during implementation. Sketch
bends now reuse the established curve family, with complete fresh/cached line
regressions under Bug #46. The mitered profile field requires the documented
`moProfileRef_w` wrapper; its initial profile-feature fixture was corrected and
retained separately as an explicit wrong-class rejection regression.

Two fresh native processes matched all 906 statuses and causes. Complete typed
reconstruction increased from 861 to 863 configurations, adding 589 and 844
without regressions. The independent reader reached exact EOF for all 863
accepted inputs, covering 69,563,297 resolved bytes and all four stream hashes.
The aggregate corpus still fails because 43 configurations remain incomplete.
The final `cargo xtask pr` gate passed formatting, Clippy, build, 1,035 library
tests, all integration tests and 277 doctests. Three external-fixture tests remain
ignored by the default gate. Stored
engineering domains, whole part/assembly/drawing/project-schema parity and native
application acceptance remain unfinished.

The humanizer skill was unavailable; its prose review is left to the maintainer.
