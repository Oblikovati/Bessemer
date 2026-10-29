# Direct configuration visual maps

Issue #54; behavior version 1. Section 11.321 / C-363 documents the +B50
map in ordinary modern configurations. A signed count precedes ordered entries
of signed key and direct, versioned visual-property body. There is no MFC
class tag or object slot around these values. The existing material reader
provides the body, including its texture, shader, appearance and DSPBR gates.

Nonpositive counts retain their stored values and consume no entries. Positive
counts are bounded before iteration. Duplicate keys stay in wire order;
replacing them as the native runtime tree does would lose source bytes.

Independent Source observations cover 54 direct maps across nine archive
versions and six count styles, with 1,980 primitive observations. Seven complete
CMgr fixtures include duplicate keys, negative counts, signed extremes and an
independent key-edit golden. Each complete stream ends at exact EOF and adds
zero object slots. Regeneration passed from an immutable Source copy. A separate
Source replay reproduced the native four-entry visual map and its 3,992-byte
key/body interval, and the native duplicate-key/no-slot regression passed.

Seven focused Bessemer tests and three public document tests passed. Two
independent native replays agreed on every status and failure cause: 901 of
906 configurations passed, up from 900, with no regressions. The added case is
the Coping Saw manager. Source consumed all 82,274,281 resolved-feature bytes
for those 901 accepted configurations at exact EOF. Five native cases remain
unsupported; the corpus is incomplete. The full local `cargo xtask pr` gate passed: 2715 tests, including
1177 library tests and 277 doctests; three external-resource tests are
explicitly ignored in that gate.

This feature does not establish complete configuration, part, assembly, drawing or installed-project semantics. The
maintainer humanizer skill was unavailable; prose review remains with review.
