# Flow record 12 counted bytes

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord12.py`.

Record 12 stores a nonnegative signed 32-bit count followed by exactly that
many raw bytes. It has no version prefix. Zero is a complete empty result;
negative counts, short bodies and any remainder are rejected. Bytes retain
all 256 possible values without text, boolean or enum conversion.

The FC12 baseline contains 681 complete schema results, including every
single byte, three-byte patterns, single bits and complements, ascending and
descending cycles, alternating high-bit values and lengths through 65,536.
Each case retains the complete payload, count, exact end and raw values.
The packet is regenerated from the reference schema in a temporary root and
checked independently against the literal count-plus-bytes layout.

The installed CFLD fixture contains one record-12 payload: 3,984 zero bytes
after the count, with exact EOF at 3,988 bytes. The freeze checks the original
whole-file hash, carrier selection, record extent, payload hash and complete
projection against the dedicated native manifest. Native evidence identifies
a consumer at member `+0x29` and a nested `+0x14` index; this does not establish
boolean meanings or an automatic correspondence between runtime objects and
stored positions. Source and payload hashes are in `baseline-verification.json`.

Run `python -m unittest discover -s docs/features/flow-counted-bytes -p test_counted_bytes.py`
and `cargo test --test flow_counted_byte_fixtures`. The Python checks include
all compact truncations, extensions, invalid counts and carrier guards. Rust
rebuilds every complete payload and checks raw value, length and native coverage.

The baseline precedes the typed read/edit API. That API must preserve exact raw
bytes, bound actual transfers before access, keep counts fixed in existing-byte
edits, reparse results completely and return immutable checkpoints with carrier
history. Runtime index mapping, byte-swapped stream modes, engineering meanings
and SolidWorks application acceptance remain outside this framing baseline.
