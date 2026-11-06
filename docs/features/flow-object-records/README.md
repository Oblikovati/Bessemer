# Flow record 304 object records

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord304.py`.

Record 304 stores a raw signed version and nonnegative row count. Versions
0 through 7 have row strides of 36, 36, 36, 36, 40, 24, 25 and 81 bytes.
Each row has a signed member and a 16-byte block. Versions below 5 also store
four discarded signed words; version 4 and later store another signed member.
Versions 6 and 7 add a raw state byte, and version 7 adds 56 opaque bytes.
Earlier versions project a native state byte of zero without storing that byte.

Versions below 5 require a 20-byte trailer whenever the stored count is positive.
Its signed member and 16-byte block remain separate from the counted rows.
The reference projects `native_appended_object` only when the count is one;
this observation does not reproduce native object population.

The owned FOB1 baseline contains 2,690 payloads and complete schema projections.
It retains every row field, optional-field presence, physical interval, trailer
field and completion qualification. Generated cases cover all versions, every
signed single bit and complement, every state byte, and row counts from zero
through 1,024. Independent stride equations and exact byte reconstruction check
the projections in both Python and Rust.

Two separately hashed native sources contain identical 64-byte version-3
payloads: one row followed by a trailer. The freeze checks whole-file hashes,
record extents, raw payload hashes and complete projections against the native
manifest. Other versions have generated coverage only.

Run:

```text
python -m unittest discover -s docs/features/flow-object-records -p test_objects.py
cargo test --locked --test flow_object_record_fixtures
```

Python also rejects every prefix of the small layouts, extra trailing bytes,
unsupported versions, negative or impossible counts, and invalid record wrappers.
It regenerates the packet and proof under a temporary root. The packet has
2,284,533 uncompressed bytes; its digest and source revisions are recorded in
`baseline-verification.json`.

`FlowCarrier::record_304_objects` opens a complete immutable `FlowObjectRecords`
view with explicit payload-byte and actual-row limits. A zero row budget admits
stored-empty payloads. `row` returns a checked borrowed row; `legacy_trailer`
retains physical trailer presence separately from the stored count and native
append predicate. Named accessors preserve every discarded word, optional signed
member, raw block, state byte and opaque extension.

Row and trailer `field_id` methods mint `FlowObjectFieldId` values for physically
stored signed words or bytes. The view's `signed` and `byte` methods check both
revision and encoding. `replace_signed` and `replace_byte` copy the complete
carrier, patch one existing scalar and reopen the whole payload. They return
`FlowSimpleTableEdit<FlowObjectRecords>` with the original checkpoint, validated
result, exact coverage, explicit diagnostics and physical carrier history.
No-op edits retain all bytes; fresh byte reversals restore the original carrier.

Tests read every field in every owned case. Edit checks cover every field in rows
when the stored count is at most 32, and the first, middle and last rows for
larger cases. Every present trailer field is edited. Both native rows and trailers
receive complete field-edit coverage. Tests also check malformed framing, exact
and one-below resource limits, absent fields, stale and foreign identities,
encoding mismatches, source isolation, public thread traits, eight-worker replay
and byte-for-byte agreement between two fresh processes.

Native identity-list binding, legacy flag lifecycle, runtime population,
engineering meanings and SolidWorks application acceptance remain unverified.
