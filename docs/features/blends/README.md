# Blend and blend-cut records

Bessemer reads, edits and writes complete ordinary modern `moBlend_c` and
`moBlendCut_c` bodies above version `0x27CB` in non-DB external-XML model archives.
Both classes use the complete body-feature base and the same direct blend-data
serializer. The data body differs from sweep data despite similar member offsets.

The declared signed count is preserved independently of the primary array count.
Negative declared counts reject; large nonnegative values do not drive allocation.
The loaded `array_330` count determines four numeric columns: unsigned words,
another unsigned word column, point triples and signed words. Complete columns
appear consecutively, rather than as interleaved rows. Their 36-byte-per-row extent
is checked before reading; each row charges six numeric entries to the aggregate
budget. The shared object-list reader now returns its already validated count,
while existing callers retain their previous fields and bytes.

Direction pointers use each signed control word's low byte. Values 2, 0x102 and
-254 select a pointer; other bits remain unchanged. The optional dimensions use
a separate nonzero byte and full-word equality with 3. A selector of 0x103 does
not select the second dimension. All raw byte fields, stored double bits and
signed/unsigned column domains remain unchanged. Independent direct arrays and
lists retain nulls, duplicate references, order and normal MFC map identities.

Independent readers verified 68 complete cases and 37,370 primitive observations.
Three versions cover the modern lower boundary, current owned streams and the
full u32 version domain. Cases include both classes, empty/populated collections,
noncanonical control values, shared direction/dimension objects, mismatched
declared/loaded counts, signed/unsigned extrema, negative zero and NaN payload bits.
Mismatch cases establish load framing; native store loops assume consistent
counts, so these fixtures do not establish native writer validity.

Four owned streams import the blank part's actual 97 slots and finish with
101, 109, 109 and 109 slots. An independently constructed edit changes the second
entry in the signed column while preserving all other columns and shared targets.
Public tests reconstruct whole documents, compare that edit and verify input
isolation after success and rejection. Private tests compare every field/byte/slot,
reject every truncated prefix and trailing bytes, reject negative declared counts
and unrelated cached references, and require exactly 18 entries for three numeric
rows. Python tests regenerate every fixture and check independent condition tables.

`reference.json` binds stable before/after sibling reader hashes, generators,
reused builders, primitive oracles and owned fixtures. The schema snapshot retains
the shared serializer and its native condition evidence.

Two fresh native processes retain all 650 prior complete configurations and add
25, reaching 675/906. Their outcomes and causes agree exactly; the remaining 231
configurations still stop explicitly. An independent reader audit reaches exact
EOF for all 675 accepted streams, totaling 38,935,930 feature bytes. Every case
matches all four original input hashes. The audit used a verified immutable copy
of 1,116 approved sibling Python files, checked before and after decoding; the
temporary copy was removed after its process exited.

The staged Windows x64 `cargo xtask pr` gate passed with Rust 1.99.0: 526 library
tests, four xtask tests, every integration suite and 265 doctests. Three external
corpus gates remain ignored by default; both native replays above ran explicitly.

Newly encountered nested
classes remain explicit dependencies. Raw meanings, populated auxiliary collection
semantics, native conversions, native application acceptance, geometric validity,
DB/deferred archives, historical layouts and full documented schema parity remain
unproven. No dependency or package version changed. The humanizer skill is
unavailable; its prose pass remains with the maintainer.
