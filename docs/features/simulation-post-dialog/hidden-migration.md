# Legacy hidden-parts load migration

Feature166's C856 evidence binds specification14.476 to the complete installed
PostStyle reader. GNU decoding and an independent Ghidra capture agree on all
227 instructions and the complete 1,010-byte body. The exact named export,
version branches, decompilation hash and store widths are retained. No target
code executes.

For explicit-field versions 700006 through 840007, the reader derives runtime
words 1EC and 1F0 from the original stored 1F0. Original zero produces `(0, 0)`.
Original 1, 2 or 3 produces `(1, 0)` through a 64-bit store of one at 1EC. Every
other signed32 value produces `(1, 1)` through two separate 32-bit stores.
The middle branch overwrites both adjacent words; retaining the original 1F0
requires separate history.

These transformations consume no additional stream bytes. The structural 1EC
field stays absent in the migration interval, and original 1F0 bytes retain their
offsets and values. At version840008 and later, 1EC is read directly instead.
The earlier raw-class conversion path is excluded. This migration does not
establish the complete hidden-parts visibility policy or all meanings of 1F0.

All 276 original accepted contexts remain in replay: 116 migration contexts and
160 later direct-read contexts. The 116 observed old words are all zero, so
nonzero migrations remain static and synthetic evidence. All eight rejected
candidates and the original prefix-only/version qualifications remain intact.
All 425 owned parents are retained without inferring defaults or rewriting
their bytes. Derived migration values add no stored-byte coverage credit.

The [verification record](hidden-migration-verification.json) records the
targeted and full replay gates. A typed migration result and the remaining
feature166 obligations remain pending.

The targeted freeze passed 18 checks in 6.119 seconds. Strict replay passed the
same 18 checks in 17.073 seconds and exited successfully after snapshot recording
and cleanup. Both retain the two original migration tests and all eight original
catalog/style/color checks. Full integrated replay and publication acceptance
remain separate gates.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
