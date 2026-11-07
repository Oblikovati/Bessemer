# Complete ordinary linear patterns

The private part codec now reads the complete ordinary `moLPattern_c` body:
inherited feature fields, header, ordered maps, embedded shape data, variation
records, tail and derived suffix. The context is the documented ordinary
external-XML/current-runtime branch. Other pattern classes and legacy runtime
branch selection remain outside this reader.

Shape records retain zero, known and unrecognized signed type values with their
documented conditional dimensions, point/reference choice and angular fields.
Variation records preserve raw missing/name flags, ignored pointer tags, nested
increments, original dimensions and duplicate component-feature references.
Direct maps and arrays allocate no enclosing tagged object. Matrix-index helpers
reuse the same complete object-array framing.

The independent packet contains 714 cases and 132,479 scalar expectations over
14 version boundaries. It includes every shape branch, populated and empty
variation collections, noncanonical flags, signed extrema, compact/extended
counts, real display dimensions, matrix helpers and both line-reference kinds.
An edit matches independently assembled bytes and preserves the original;
source overwrite and concurrent writes preserve the result. Map and variation
budgets are counted independently: six/fifteen entries pass, five/fourteen fail.

Bug #46's line-role regression failed before the finite-family fix. The declared
line-reference field also accepts `moTempAxisRef_w`, whose complete serializer
extends the line/curve grammar. Loaded identities and fourteen independently
assembled new-object cases verify it. Unrelated classes and class-name suffixes
remain rejected; the display-list role policy stays unchanged.

The native gate now reconstructs all 906 streams exactly, across 842 document
paths. This establishes stored-archive interoperability for that frozen corpus.
It does not establish geometry validity, native application acceptance or
coverage of every documented model class. The codec remains private while public
result/history contracts and selected-configuration diagnostics are completed.
Assembly, route-assembly, drawing and separate feature archives remain in #2.

The snapshot and verification report retain source clauses, fixture and log
hashes, exclusions and the earlier 903/906 role-failure checkpoint. Humanizer was
unavailable; prose remains for maintainer review.
