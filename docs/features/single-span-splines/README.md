# Single-span splines

Issue #54; behavior version 1 for the exact single-span prototype. Section
11.360 / C-402 proves that stored mask 0x80 selects the native mode-zero
prototype and then calls the complete common spline serializer. No new tagged
spline object or map slot is introduced. Other single-span mode masks remain
unsupported; the existing standard mask family stays available.

The parsed prototype is reused by the secondary-interpolation gate. A present
secondary block is consumed when control points exist or when the single-span
prototype's verified +1F0 callback returns true. Zero parameters still retain
the width and two settings. The derived runtime mode is not invented as a
serialized field. The shared fixed equation-transform correction is Bug #95.

The actual independent reader accepted and regenerated 800 complete direct
graphs with 75,720 scalar expectations. They cross inactive null and three
point-list classes, empty/populated controls, equation flags, five interpolation
modes, both secondary-presence paths and two versions. Seven complete owned
sketch streams include a separately assembled edit golden. Five focused Rust
tests, three public document tests and two immutable reference tests passed.
The actual reference reader also passed its two-source native regression.
Two fresh Bessemer processes matched all 906 outcomes and causes. Acceptance
remains 888/906 with no regressions; case 340 now passes the single-span records
and stops later at the unsupported sculpt feature. The aggregate corpus still
fails. The full local gate passed 1,107 library tests and 277 doctests,
with 2,615 passing tests across 190 executables and 3 intentional ignores. Unproven masks fail before the spline payload, and all truncated
prefixes, trailing bytes and old contexts fail.

Saved external unit components and scalar display dimensions from the same
schema clause remain separate work. Native application acceptance and raw
geometry meanings are not established by this framing support. The humanizer
skill was unavailable; prose review is left to the maintainer.
