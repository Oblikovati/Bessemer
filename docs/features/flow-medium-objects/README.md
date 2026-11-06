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

This independent fixture baseline predates the typed body API below. Its scope
is complete serialized body framing under explicit native UTF-16 mode zero.
Complete backend/outer-reference profiles, engineering interpretation, curve
evaluation and native runtime/application acceptance remain separate requirements.

## Typed body API

`FlowMediumBackendPrefix::first_kind_one_body` requires the first-table kind-one
boundary and explicit positive limits on consumed body bytes, total wide patterns
and stored name units. Following bytes do not spend the body-byte bound. Physical
array extents and wide-word budgets are checked before taking each borrowed span.
The decoder reuses the metadata UTF-16 dialect and existing raw wide-array views.
It allocates fixed curve metadata rather than one object per numeric value.

`FlowMediumKindOne` retains its immutable backend and carrier checkpoints. It
exposes the base byte and key, lossless name, all seven curve views, shared arrays,
final scalars and complete stored state byte. `FlowMediumWideField` selects a
curve scalar, a physical curve-array role, one of six shared arrays or the final
four scalars. Array intervals use record-payload offsets; curve intervals are
relative to the body. Inactive extra arrays are absent selections, while active
zero-count arrays are completed empty views.

Existing base/key/state bytes, wide patterns, curve flags and name units can be
edited. Flag replacements retain their zero/nonzero presence branch, and name
edits retain counts and terminal NULs. Each edit reopens the complete carrier,
metadata prefix, backend boundary and body, checks every structural interval and
the complete patched payload, and retains typed before/after checkpoints with all
physical history relations. Exact no-ops preserve identity; fresh reversals
reproduce the original full carrier. Count and version framing are read-only.

The focused suite matches every field and interval across all 912 views. Every
existing field in small arrays/strings and both endpoints of larger ones receives
no-op, independent patch, full history and fresh reverse checks. This includes
all 1,611 native wide patterns, six name units, three base/key/state fields and
seven flags in each native body view. Read-only version/count framing is checked
against the independent fixtures and malformed-input controls.

The complete native record also composes through its actual metadata and backend
boundaries: the body occupies payload bytes 138 through 13,111, leaving 80 bytes
untouched. Name edits recompute first-NUL text without normalizing surrogates.
Stored state zero remains zero even though the explicit native loaded-state
projection returns one. Tests cover independent exact/one-below/zero limits,
small truncations, all unsupported body/curve version bytes, invalid counts,
wrong kinds, missing arrays, invalid indices, terminal-NUL removal and branch
changes. Eight public types are asserted Send/Sync; eight workers match complete
reads and edited carriers after caller input mutation. Two fresh processes
reproduce the entire independent FMO1 packet byte for byte.

Coverage reports complete body framing and `full_record_schema = false`.
Engineering meanings, numeric evaluation, following references/outer collections,
native runtime state and application acceptance remain explicitly unchecked.
