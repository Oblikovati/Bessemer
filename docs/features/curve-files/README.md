# Curves from stored files

Bessemer composes the complete inherited reference-curve body, including its
kernel wire and direct attributes, before reading the stored filename and point
list. The filename remains archive text; parsing does not open or resolve it.
The signed declared count is retained. Nonpositive counts select no points;
positive counts are bounded by remaining bytes and the entry budget before
iteration. Point coordinates preserve all binary64 bits.

The point-reference occurrence exists only after version `0x42CD`, including
when null. It uses the existing finite point-reference family and shared map.
Older versions omit the occurrence entirely.

Independent fixtures cover 42 graphs across seven versions and six value styles,
with 6,508 primitive observations and seven owned document fixtures. Five focused
Rust tests, three public save/reopen, point-edit and input-isolation tests, and
immutable fixture regeneration passed. The independent native source regression
also reproduced all three complete streams, including their points, root hashes,
counts, stored path and unsuppressed inherited wire.

Two fresh native processes matched all 906 statuses and causes. Complete typed
reconstruction increased from 863 to 866 configurations, adding 33, 263 and 264
without regressions. The aggregate corpus still fails because 40 configurations
remain incomplete. The independent reader confirmed exact EOF for all 866
accepted inputs, covering 70,284,213 resolved bytes and all four stream hashes.
The full `cargo xtask pr` gate passed formatting, Clippy, build, 1,040 library
tests, all integration tests and 277 doctests. Three external-fixture tests remain
ignored by the default gate. Whole-schema parity, engineering semantics and native application
acceptance remain unfinished.

The humanizer skill was unavailable; its prose review is left to the maintainer.
