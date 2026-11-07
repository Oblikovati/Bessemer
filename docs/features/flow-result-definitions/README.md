# Flow result definitions

Issue: #241. Reference: `../inventor-interface/solidworks/flowresultdefinitions.py`.

Record 500's documented profile is one exact zlib stream containing byte version
one and five collections. The first collection stores every signed identifier
before its GUID/body pairs. The remaining four collections must be empty. IMS
versions zero through three conditionally store one, two or four shared arrays;
versions two and three also store an allocation byte. Shared arrays require
marker one, an unsigned count within the signed native-transfer range and raw
32-bit values. Complete decoded EOF is required.

The owned FDF1 packet contains 1,136 cases and 6,834,825 uncompressed bytes. It
preserves stored payloads, full decoded bodies, all collection/object/array
intervals, physical identifier order, array presence and raw values. Generated
cases cover every scalar bit/complement, all allocation bytes, duplicate and
negative identifiers, object-count boundaries, lengths through 65,536 and
compression variants.

The installed CFLD sample has one IMS version-two definition, allocation byte
zero, array lengths 3,876 and 4,056, and 31,798 decoded bytes. Its 6,767-byte stored
payload occupies record offsets 65,378 through 72,157. The freeze checks the
original source hash, record extent, payload hash and full normalized reference
projection against the native pair manifest.

Rust reconstructs every decoded byte independently, checks all intervals and
version-dependent presence, and requires exact compressed-stream consumption.
Python checks independent size equations, every GUID bit mutation, strict counts,
markers and versions, all proper prefixes of a small complete body, compressed
framing, duplicate order and deterministic regeneration under a temporary root.

```text
python -m unittest discover -s docs/features/flow-result-definitions -p test_definitions.py
cargo test --locked --test flow_result_definitions_fixtures
```

Digests and focused results are recorded in `baseline-verification.json` and
`local-verification.json`. The typed reader/editor and paired record-501 values
remain to be implemented. Native runtime initialization, engineering meaning and
SolidWorks application acceptance remain unverified.

The humanizer skill is unavailable; maintainer prose review remains pending.
