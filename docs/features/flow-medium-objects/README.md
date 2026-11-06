# Flow medium kind-one bodies

Issue: #241. Reference: specification section 14.54 and
`../inventor-interface/solidworks/flowmediumobjects.py`.

The complete serialized kind-one body starts with version zero, an uninterpreted
base byte, sixteen key bytes and a counted raw UTF-16LE name. Seven version-zero
curves follow. Each curve retains a raw binary64 scalar, a nonnegative signed
count, two arrays of raw binary64 patterns and a complete flag byte. Every
nonzero flag reads two more arrays of the same count. A nonzero flag with count
zero still reaches the extra-array branch; it is distinct from a zero flag.

Two arrays share the next count, four arrays share a second count, and four raw
binary64 scalars precede the final stored state byte. Native loading sets the
runtime state to one independently of that stored byte. The base byte does not
select a body version. Array columns and scalar engineering roles remain
unassigned. Numeric bit preservation includes NaN payloads, signaling NaNs,
signed zero, infinities and subnormals without numeric conversion.

The FMO1 baseline freezes 912 views. Generated cases cover all 256 base and state
bytes, every flag at both zero and nonzero counts, all individual binary64 bits
and their complements, special numeric patterns, empty and large arrays, zero-unit
versus stored-NUL names, embedded NULs, unpaired surrogates, noncharacters and a
65,536-unit string. All seven curve intervals and arrays are retained alongside
the name interval, raw text, native first-NUL projection and exact body boundary.
The packet also records the explicit extra-array gate, including active count
zero. Names are fixture framing; all decoded expectations come from the reference
reader and are checked by independent byte reconstruction.

Two native views retain the same complete body with and without its following
bytes. The body is 12,973 bytes; its original suffix contains 80 further backend
and outer-record bytes. Curve counts are 50, 50, 50, 50, 100, 100 and 100. Only
the first native curve has extra arrays. The following shared counts are 50 and
100, and the name stores six UTF-16 units.

Rust rebuilds every body byte from the frozen fields, checks every interval,
recomputes first-NUL text without Unicode normalization, and appends the exact
tail. Python independently reconstructs all cases, verifies reproducible packet
generation, rejects all proper prefixes of small and native bodies, checks every
unsupported version at the body and seven curve positions, and rejects negative
or physically impossible counts, invalid string modes and missing terminators.

This is a fixture baseline for the typed body implementation. It establishes
complete serialized body framing under explicit native UTF-16 mode zero. It
adds no production body reader or editor, complete backend/outer-reference
profile, engineering interpretation, curve evaluation or native runtime and
application acceptance. Those remain required work toward schema parity.
