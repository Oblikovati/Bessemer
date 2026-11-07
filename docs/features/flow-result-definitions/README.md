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

The typed reader uses independent decoded-byte, physical-object and total-value
bounds. Counts are checked against the remaining complete frame before metadata
allocation. Zero object and value bounds admit empty collections and arrays.

```rust,ignore
let definitions = carrier.record_500_ims_definitions(&record, FlowResultDefinitionOptions {
    maximum_decoded_bytes: 2_000_000,
    maximum_objects: 100_000,
    maximum_values: 1_000_000,
})?;
let object = definitions.definition(0)?;
let first = object.array(FlowImsArrayKind::First);
let edit = definitions.replace_identifier(0, -7)?;
```

Views expose every collection interval, physical definition ordinal, signed
identifier and its separate-block interval, raw GUID, normalized IMS version,
body interval, optional allocation byte, shared-array interval and raw values.
The outer version is a byte, independently of the signed IMS definition version.
Absent arrays return `None`; present-empty arrays retain their shared header and
return a zero-length word view. Duplicate identifiers remain separate definitions.

Editors replace existing identifiers, allocation bytes and raw array patterns on
an isolated carrier. They preserve counts, GUIDs, versions and array presence,
then reparse the entire profile and compare all layouts before returning the
reused `FlowSimpleTableEdit` checkpoints and complete physical history. No-op
edits retain original compressed bytes. Fresh reversals restore decoded bytes
without requiring another compressor's original encoding.

Digests and focused results are recorded in `baseline-verification.json`,
`local-verification.json` and `api-verification.json`. Only IMS version two has
native fixture coverage; the other versions are generated reference cases.
Paired record-501 values, native runtime initialization, engineering meaning and
SolidWorks application acceptance remain unverified.

The humanizer skill is unavailable; maintainer prose review remains pending.
