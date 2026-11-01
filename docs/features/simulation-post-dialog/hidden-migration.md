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
targeted and full evidence replay gates. The typed projection below keeps its
operation acceptance separate. The remaining feature166 obligations remain
pending.

The targeted freeze passed 18 checks in 6.119 seconds. Strict replay passed the
same 18 checks in 17.073 seconds and exited successfully after snapshot recording
and cleanup. Both retain the two original migration tests and all eight original
catalog/style/color checks. Full integrated replay and publication acceptance
remain separate gates.

## Typed projection

`hidden_parts_migration_inputs` copies the archive's explicit version and
optional original signed 1F0. It does not create an absent stored 1EC or rewrite
the archive. `project_legacy_hidden_parts` first requires version700006..840007,
then requires the original word. Outside that interval it returns a typed
unsupported-scope error, including for later direct-read contexts. An unknown
word inside the interval returns a typed missing-input error.

Completed results retain both derived runtime words and the exact store kind.
The coverage distinguishes one four-byte store, one eight-byte store and two
four-byte stores. It records zero additional wire bytes and zero local pattern
error, with no native pointer or full visibility validation. Immutable derived
history retains the original version and 1F0 even when its runtime copy becomes
zero or one. Original byte payloads remain unchanged.

The operation corpus contains 4,336 version/presence/signed-pattern cases.
Complete parent replay retains all 425 owned contexts and 276 native contexts,
plus 140 full 1F0 mutations. Each parent keeps its actual version and eight
explicit original, missing or hypothetical word contexts. These hypotheses do
not promote nonzero migrations into native sample observations.

The [operation record](hidden-migration-operation-verification.json) and
[fresh-process proof](hidden-migration-process-verification.json) retain exact
executed checks and remaining gates. The earlier raw-class conversion, later
direct reads and whole hidden-parts visibility decision remain separate.

All eight owned Rust checks and the complete native-parent check passed,
including eight-worker comparisons. All eight public types satisfy `Send + Sync`.
The strict Source suite passed 20 checks in 5.956 seconds and exited successfully
after snapshot recording and cleanup. Clippy passed for all targets and features
with warnings denied. Two fresh Rust processes matched all 90,194 independent
expected words byte for byte. Full integrated replay and publication validation,
including ThreadSanitizer, remain separate gates.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
