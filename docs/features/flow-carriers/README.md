# Flow ordinary physical carriers (#241)

`bessemer::flow::FlowCarrier` reads the ordinary physical carrier in specification
14.29. It admits the observed little-endian version words 7 and 8, followed by
records with a four-byte identifier, eight-byte payload length and exact payload.
The parser requires complete physical EOF. Unknown, duplicate, reserved and
empty records remain in their original order, with revision-bound occurrence IDs.
`latest` selects the final ordinary occurrence and explicitly rejects the two
reserved native index selectors, -99 and -101.

```rust,ignore
let carrier = FlowCarrier::from_bytes(&raw, limits)?;
let record = carrier.latest(123)?.ok_or("missing header")?;
let edit = carrier.replace_payload(&record.id, replacement)?;
let output = edit.result().as_bytes();
```

Payload replacement is a physical operation on an isolated copy. It changes the
selected length and payload, preserves every other byte, and reparses the entire
output before publication. The result retains the original checkpoint and a
source/result parent for every record. A no-op retains the original revision.
Foreign or stale IDs fail without modifying either input. Fresh replacement of
the original payload restores the original carrier bytes, including original
record order. Positive bounds limit complete wire bytes, record count and each
payload before allocation.

Coverage is explicitly `ExactOrdinaryPhysicalCarrier`: complete physical framing,
opaque payload-byte count and zero achieved byte error. Successful empty carriers
retain a valid version word and report zero records. Diagnostics exclude payload
schemas, alternate compressed terminal-index interpretation and native application
acceptance. Physical replacement does not update opaque index offsets or infer
payload cross-references. The [GUID-header view](../flow-headers/README.md)
separately reads and edits established prefixes. The [record-433 view](../flow-arrays/README.md)
adds compressed signed arrays and existing-cell edits. Grouped record 404 and
the later documented Flow schemas remain separate required work.

The frozen packet includes all three installed carriers: 66 records across
2,936,303 original bytes, plus owned empty, duplicate-ID and reserved-ID cases.
The Python reference parser and source corpus independently establish every
identifier, offset, size and payload hash. Tests cover every record's no-op,
empty and changed replacement, all untouched bytes, fresh reversals, every u16
version, small-carrier truncations, u64 overflow, complete appended records,
exact/one-below bounds, input isolation and eight concurrent editors.
`native-byte-verification.json` authenticates 16 retained Source native ranges
against the original installed DLL; it adds no native execution credit or claim
that every retained range covers a complete logical function.

The humanizer skill is unavailable; its prose pass remains deferred to review.
