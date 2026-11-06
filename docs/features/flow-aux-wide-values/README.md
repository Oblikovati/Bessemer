# Flow record 444 auxiliary wide values

Issue: #241. Reference: `../inventor-interface/solidworks/flowauxrecords.py`.

Record 444 stores one complete zlib stream. The decoded body begins with a signed
version of zero or one. Version zero then stores a discarded signed word. Both
versions store a nonnegative signed count followed by exactly that many raw
64-bit patterns. No bytes may follow the array. The documented native reader
discards the values; preserving their serialized bits does not reproduce native
runtime population or assign engineering meaning.

The owned FAW1 baseline has 559 complete payloads and projections, totaling
5,475,611 uncompressed packet bytes. It stores compressed and decoded images,
optional legacy-member presence, every raw pattern, exact decoded EOF and the
native-discard qualification. The source and packet digests are recorded in
`baseline-verification.json`.

Both versions cover every 64-bit single bit and complement, all-ones values,
negative zero, infinities and distinct NaN patterns. Legacy members cover every
32-bit single bit and complement. Array lengths range from zero to 65,536.
Compression cases cover levels 0–9, five strategies and two window sizes, with
identical decoded projections checked independently of compressed-byte identity.

The native FLD sample has a 32-byte compressed payload and a 1,452-byte decoded
version-zero body: discarded member 6 and 180 zero patterns. The freeze checks
the complete source digest, record extent, stored payload digest and full schema
projection against the dedicated corpus manifest. Version one has generated
coverage only.

Run:

```text
python -m unittest discover -s docs/features/flow-aux-wide-values -p test_wides.py
cargo test --locked --test flow_aux_wide_value_fixtures
```

Python checks independent length equations, bit coverage, all small decoded
prefixes, extra decoded bytes, invalid versions and counts, record-wrapper
identity and size, and exact compressed stream boundaries. It also checks native
provenance and regenerates the packet under a temporary root. Rust reconstructs
all decoded bytes, verifies exact zlib consumption and checks the same independent
bit and count coverage.

`FlowCarrier::record_444_aux_wides` opens a complete `FlowAuxWideValues` view with
explicit decoded-byte and actual-value limits. A zero value limit admits an empty
array. `discarded_legacy` distinguishes a physically stored zero from version-one
absence, and `values` exposes every pattern through `FlowTableWideArray`.
`native_values_discarded` reports the documented native observation while the
library retains the bytes for round-trip edits.

`replace_pattern` and `replace_discarded_legacy` return
`FlowSimpleTableEdit<FlowAuxWideValues>` with the original checkpoint, completely
reopened result and full physical carrier history. Coverage reports exact zlib
framing, stored and decoded extents and the raw-wide count. Diagnostics leave
native runtime population, engineering roles and application acceptance explicit.

Tests read every owned pattern. They edit every stored legacy member and every
value when the count is at most 257, plus all 180 native values. Larger generated
arrays receive first, middle and last value edits. Each edit has an independent
expected byte patch, compressed no-op preservation, complete history and a fresh
decoded reversal. Reversals do not require another compressor's original byte
encoding. Tests also check resource boundaries, malformed streams and bodies,
absent fields, source isolation, public thread traits, eight-worker operation
agreement and byte-for-byte replay in two fresh processes.

Native runtime population, engineering roles and SolidWorks application
acceptance remain unverified.
