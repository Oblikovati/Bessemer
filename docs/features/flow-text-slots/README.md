# Flow record 311 text slots

Issue: #241. Reference: `../inventor-interface/solidworks/flowauxrecords.py` and
the strict string transfer in `flowmetadataprefix.py`.

Record 311 has an arbitrary signed header followed by exactly fifty text slots.
The header does not dispatch a version. Parsing requires explicit native UTF-16
mode zero. Each slot stores a nonnegative signed code-unit count followed by
little-endian UTF-16 units. A positive count requires a final NUL unit. Zero
stored units, a stored NUL and a NUL followed by hidden text remain distinct
serialized forms.

The native text projection stops at the first NUL. Unpaired surrogates, hidden
suffixes, byte-order characters and all other raw units remain preserved. Unicode
normalization and lossy decoding would change the documented result.

The apply operation takes fifty existing text values. A nonempty native prefix
replaces the corresponding value. An empty native prefix retains the complete
previous value, including any embedded NUL, hidden text or unpaired surrogate in
that previous value. It does not clear the slot merely because a serialized
string is present.

The owned FTS1 packet contains 755 complete cases and 3,643,920 uncompressed bytes.
Each case stores the complete payload and signed header. Every slot retains its
ordinal, physical intervals, stored count, raw units, native prefix, previous text
and applied text. Text state is encoded as raw UTF-16 bytes so unpaired surrogates
remain comparable without requiring a valid Unicode scalar string.

Generated cases cover all fifty positions, every UTF-16 code unit, every signed
header bit and complement, empty and NUL-only strings, hidden suffixes, combining
forms, surrogate pairs and unpaired surrogates. Stored lengths reach 65,537 units.
The native FLD sample has a 644-byte body with header zero, fifty slots and 220
stored units. Its source digest, record extent, payload digest and complete
projection are checked against the dedicated native manifest. Mode zero remains
explicit; native owner-mode provenance is not claimed.

Run:

```text
python -m unittest discover -s docs/features/flow-text-slots -p test_slots.py
cargo test --locked --test flow_text_slot_fixtures
```

Python checks independent code-unit prefixes, exact spans, previous-state
preservation, malformed strings, unsupported mode arguments, fixed slot counts,
record wrappers and native provenance. It regenerates the packet under a temporary
root. The Rust baseline checks the same raw reconstruction and state projections
without converting unpaired surrogates into Rust strings.

Source and fixture digests are recorded in `baseline-verification.json`. The
typed reader, isolated editor and existing-state projection API remain to be
implemented. Slot purposes, native runtime updates and SolidWorks application
acceptance remain unverified.
