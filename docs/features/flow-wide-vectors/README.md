# Flow record 148 wide vectors

Issue: #241. Reference: `../inventor-interface/solidworks/flowauxrecords.py`.

Record 148 has a raw version-zero body. A nonnegative signed vector count precedes
the complete table of signed vector lengths. All lengths are nonnegative and
appear before any vector contents. The array bodies then follow in physical
vector order as raw 64-bit patterns, with exact EOF after the final body.

A zero vector count differs from one or more stored empty vectors. Empty vectors
still occupy length-table entries and retain their physical ordinals, even when
their body intervals share an offset. The baseline preserves that distinction
without assigning engineering roles to the values.

The owned FWV1 packet contains 394 complete cases and 7,858,997 uncompressed
bytes. Each case stores the original payload, version, count and exact EOF, plus
every vector's length, body interval and raw bytes. Generated cases cover empty
and ragged vectors, all-empty tables, different populated positions, counts up to
4,096 vectors and individual lengths up to 65,536 patterns. They also cover every
64-bit single bit and complement, all-ones values, negative zero, infinities and
distinct NaN patterns.

The native FLD sample has one vector with one stored pattern in a 20-byte payload.
The freeze verifies its whole-file digest, physical record extent, payload digest
and complete original projection against the dedicated native manifest. Generated
cases provide the larger and ragged layouts.

Run:

```text
python -m unittest discover -s docs/features/flow-wide-vectors -p test_vectors.py
cargo test --locked --test flow_wide_vector_fixtures
```

Python checks independent byte-length equations, literal length-table order,
empty-vector presence, malformed prefixes and suffixes, invalid counts and
lengths, unsupported versions, record-wrapper checks and native provenance.
It regenerates the owned packet and proof under a temporary root. Rust rebuilds
the complete payload with all lengths before all bodies, checks every physical
interval and verifies independent bit-pattern and count coverage.

Source and fixture digests are recorded in `baseline-verification.json`.

`FlowCarrier::record_148_wide_vectors` opens a complete immutable `FlowWideVectors`
view with independent payload-byte, actual-vector and total-value limits. Every
length and the complete body extent are validated before allocating immutable
vector metadata. A zero value limit admits stored-empty vectors within the other
bounds. `vector` returns a checked `FlowTableWideArray`, including present-empty
arrays; `length_table_interval` identifies all stored length words.

`replace_pattern` replaces one existing vector element on a copy, preserves every
length word and completely reopens the result. It returns
`FlowSimpleTableEdit<FlowWideVectors>` with original and validated checkpoints and
full physical carrier history. Coverage reports exact raw framing, actual vectors,
total values and stored-empty vectors. Diagnostics state the runtime, engineering
and application exclusions.

Tests read every owned vector and raw pattern. They edit every value in vectors
of at most 257 elements, including the native value, and the first, middle and last
values of larger vectors. Each edit has a literal expected byte patch, unchanged
length-table bytes, exact no-op preservation, full physical history and
a fresh whole-carrier reversal. Other checks cover all three exact resource
boundaries, zero-value budgets with stored-empty vectors, aggregate value limits,
malformed layouts, source isolation, public thread traits, eight-worker operation
agreement and byte-for-byte replay in two fresh processes.

Native runtime population, engineering roles and SolidWorks application
acceptance remain unverified.
