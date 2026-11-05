# Bidirectional sweep-twist schema work

Feature #211 closes the fresh `moTwistDataForBiDirectionalSweep_c` stop newly
specified in clause 11.156 / C-3524. The body is exactly 17 bytes: raw `u8` member
08, then raw binary64 members 10 and 18. Runtime allocation size 0x20 does not
supply file padding. The published descriptor has runtime schema 0x80000001;
the documented MFC wire schema is 1.

The retained native evidence and all 23 of its source hashes matched the supplied
artifacts, including the original binary. No native function or proof was rerun.
`schema-reference.json` records those checks; `schema-snapshot.md` retains the
literal section, and `native-schema-evidence.json` retains the supplied evidence.
Neither the native getter names nor the byte's fullword getter establish Boolean
normalization, numeric units or valid geometry.

The initial bare corpus crosses four modern versions, all 256 saved byte values
and six IEEE bit-pattern pairs: signed zeros, signed subnormals, infinities,
quiet/signaling NaN payloads and ordinary finite values. It contains 6,144 fresh
objects. Each body is independently assembled with Python wire primitives.

The oracle uses untouched original Source MFC framing and primitive readers with
one fixture-only reader added to an operation-local table. It verifies exact EOF,
three map slots and exact float bits. The unextended original production reader
still rejects the class before and after extended reads. This is a declared-wire
extension, not original production class-reader acceptance or native producer replay.
Source files are copied and hash-checked before imports; import origins and the
complete snapshot are retained in the compressed source reports.

Production admission is deliberately still pending. Before adding it, complete
owned sweep/cut graphs must verify fresh/cached/shared identities, actual inherited
context, independent expected edits, whole-container reopen/fresh reversal,
history, budgets, malformed input, isolation and concurrency. The older 160 sweep
fixtures must remain unchanged. Fresh unproved schemas and nonordinary/older
contexts retain explicit errors. No speculative public API is needed: the existing
sweep pointer role and resolved-feature read/edit functions are callers.

The new class must remain distinct from the original unsupported profile in the
verification record. There is no new native-population, byte-accounting, application,
geometry or full-parity credit. Full local and exact-head remote gates remain
required before serial integration. No dependency, package version or release tag
changed. Humanizer is unavailable; maintainer prose review remains pending.

The complete-sweep fixture builder now accepts an optional operation-local twist
pointer writer. Its default remains a null pointer. The pointer writer reserves
separate class/object slots for fresh records, one object slot for cached-class
records, and no slot for shared-object references. Focused tests check literal
schema-1 tags, inherited map offsets, isolation between operations, invalid plans,
and insertion into 16 complete sweep/cut bodies across the four version boundaries.
These constructor tests do not establish parser acceptance or document edit coverage.
