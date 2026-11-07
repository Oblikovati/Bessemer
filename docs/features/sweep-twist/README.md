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

The local production reader now admits schema 1 in modern ordinary-part contexts.
It reads exactly 17 body bytes and preserves the raw byte and both double bit
patterns. Unknown schemas and older/nonordinary contexts retain explicit errors.
The existing pointer role and resolved-feature read/edit functions are callers;
there is no new public API. Full gates and the remaining focused validation still
precede integration.

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

A pinned run against 6,957 original Python source files passed nine tests. It
parsed 12 complete sweep/cut streams against the actual blank part configuration
map, then parsed 36 independently assembled field edits. Original functions read
the surrounding features, inherited context and MBSM tail; the operation-local
fixture callback reads only the documented twist primitives. Shared pointers keep
object identity and cached-class pointers create distinct objects. Unextended
original parsing rejected all 12 new streams. This remains fixture-only evidence.
The initial source snapshot and verification record preserve this historical scope.

A later pinned run passed 11 tests, including byte-identical regeneration of all
160 older sweep records and seven owned fixture sets. It retained 48 new complete
streams (12 originals and 36 field edits) in a 69,612-byte packet. Bessemer's focused
runtime checks passed the 6,144 bare cases, and public document checks passed exact
stream/container reconstruction, reopen, 36 independent edits and fresh reversals.
An eight-worker test exercised concurrent edits of the 12 original immutable
archives and confirmed source isolation. ThreadSanitizer and full-gate acceptance
for this new reader remain pending; this stress test does not replace them.

The surface extension passed 12 pinned Source tests with 6,978 Python files.
Forty retained surface streams cover all six existing forms, fresh twist objects,
shared twists, cached-class twists and three independent field edits per original.
The unextended Source reader rejected all 40. Separate constructor checks kept all
162 older bare surface graphs and 19 owned streams byte-identical.

The public document tests now cover 88 complete streams and 66 edits, fresh
reversals and revision-bound history results. They check completion, exact stored
fidelity, coverage and diagnostics together. Context regressions reject
unspecified, Simulation, assembly and routing-assembly archives. These later tests
are excluded from the full gate running at 02b5676e and need the next full gate;
remote matrix and ThreadSanitizer acceptance remain pending.
