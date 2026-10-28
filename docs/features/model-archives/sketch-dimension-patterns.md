# Sketch dimensions and patterns

Feature #44 adds the documented modern sketch dimension collections, logical
constraints, isoparametric states and linear/circular patterns. Valued dimensions
use tagged objects and their shared identities. Logical dimensions, states and
patterns use direct bodies without allocating enclosing map slots. Collection
counts are literal u16 values, including 65535; they do not use MFC's extended
count encoding.

The readers retain stored flags, signed scales, transform presence, help-data
aliases and version-specific fields. They cover both boundaries of the G2
legacy-word interval and the logical-group index gate. Isoparametric states
accept width zero or typed u16 entries at width two. Other widths stop explicitly,
and counts are checked against available bytes and the operation budget.

The independent packet contains 632 cases and 21,164 scalar expectations across
eight versions. It exercises standalone bodies, populated collections, aliases,
empty collections and all stages composed in wire order. Every case reconstructs
exactly; all truncated prefixes and trailing bytes fail. An integer-scale edit
matches independently assembled bytes, preserves the original and reparses with
the expected fields. Every case survives source-buffer overwrite, and parallel
writers reproduce the same owned pattern bytes.

Bug #86 / C-395 corrected the shared valued-dimension transform from compact
restore to the fixed 30-word archive operator. The original packet remains frozen
as negative replay evidence. All 632 positive cases were independently regenerated;
the 104 corrected transforms now expose raw words instead of compact projections.
See [the framing correction](../fixed-valued-transforms/README.md).

Some pattern cases explicitly seed note and count-dimension object identities.
All 72 cases that reference these identities reject missing or wrong-class
targets. The remaining 32 seeded cases do not reference them and are excluded
from that assertion. This tests reference handling; it does not supply bodies
for these classes or claim their independent serialization. New nonnull bodies
still require implemented readers. Logical help arrays reuse the existing
typed object-list grammar with the documented `su_CObArray` class.

The nine dimension/pattern tests, all-target Clippy and all 51 Python tests
passed, along with all 224 model tests. All three fixtures reproduced exactly
in two fresh reference processes. The native run retained all 906 outcomes:
866 accepted and 40 rejected, so the complete corpus gate still fails.
The schema snapshot retains eight clauses from edition 0.353 with checked byte
hashes. The verification manifest records broader tests and the native corpus
run separately. The complete sketch tail and root remain pending, as do public
operation results/history and native application acceptance. No partial sketch
root is exposed. Humanizer was unavailable; prose remains for maintainer review.
