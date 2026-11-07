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

Source and fixture digests are recorded in `baseline-verification.json`.

`FlowCarrier::record_311_text_slots` opens an immutable `FlowTextSlots` view under
`FlowTextSlotOptions`. The caller declares `NativeUtf16`; payload-byte and total
stored-unit bounds include hidden suffixes and terminators. A zero unit bound
admits fifty stored-empty strings. The signed header remains uninterpreted.
Checked borrowed slots expose complete raw bytes, stored units, physical intervals
and the separate native prefix. No Unicode scalar conversion is required.

`FlowTextSlotState::from_units` takes ownership of exactly fifty `Vec<u16>` values
under an actual-unit bound and publishes immutable state. It exposes no mutable
unit references. `project` validates an independent result-unit bound before
constructing output, then retains immutable previous values for empty prefixes
and copies nonempty source prefixes into owned UTF-16 values. The returned
`FlowTextSlotProjection` retains the source record, previous state, result state,
coverage and fifty physical-ordinal history relations. A nonempty prefix remains
an applied-source relation even when its units equal the previous value.

`replace_header` and `replace_unit` return `FlowSimpleTableEdit<FlowTextSlots>`
after copying and reopening the complete carrier. Unit edits preserve stored
counts and revalidate terminal NULs. Interior-NUL changes recompute native
prefixes and subsequent state projection. No-op edits retain all wire bytes;
fresh reversals restore the original raw carrier with full physical history.

Tests read every stored unit and verify every frozen prior/result state. They
exercise every header and every native-sample unit, all units in generated strings
of at most 64 units, and first/middle/last positions in larger strings. Terminal
units receive no-op and rejected nonzero-edit checks. Other checks cover literal
byte patches, exact resource boundaries, malformed framing, foreign and stale
identities, caller-buffer mutation/drop, public thread traits, eight-worker
operation agreement and byte-for-byte replay in two fresh processes.

Slot purposes, native runtime updates and SolidWorks application acceptance remain
unverified. Diagnostics retain the explicit mode assumption and those exclusions.
