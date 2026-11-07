# Flow record 121 viewer-query prefix

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord121.py`.

This is the viewer inflate-metadata-query profile. It is distinct from the
FWCPT01 version-six graph layout. The outer signed version must be at least two;
it precedes one complete checksum-valid zlib stream. A version value alone does
not establish which native profile applies.

The decoded prefix stores an ignored signed word, a second signed word and three
strictly positive array lengths. All lengths precede the three raw-wide arrays.
A discarded three-value raw-wide tuple and a final signed word follow the arrays.
The native metadata projection returns the final word as its first output. Its
second output is two when the stored second word is one, otherwise that stored
word is retained unchanged. None of these fields has an assigned engineering role.

Everything after this known prefix remains opaque. The reference reports
`complete_payload_framing = false` even for a synthetic body with no opaque tail.
Extra decoded bytes therefore extend the unknown suffix; they do not validate a
complete body. Extra compressed bytes, concatenated streams and truncated streams
are rejected. Knowing a prefix does not permit a partial compressed stream.

The owned FVP1 packet contains 525 viewer-profile cases and 8,094,299 uncompressed
bytes. Each case retains the stored payload, complete decoded body, all known
fields and intervals, native metadata outputs, opaque suffix and incomplete-body
qualifications. Cases cover every signed scalar bit and complement, every raw-wide
single bit and complement, NaN and infinity patterns, positive lengths through
65,536, outer-version boundaries, compression variants and opaque-tail boundaries.

The native CPT sample has outer version three, array lengths 45, 13 and 13, a
616-byte known decoded prefix and a 141,227-byte unknown suffix. The entire decoded
body has 141,843 bytes; its stored payload has 33,601 bytes. The freeze verifies
the complete source file, physical record extent, stored payload, full original
prefix projection and opaque-tail digest against the dedicated native manifest.
Derived fixtures with shortened native tails remain prefix-only observations.
No proportional compressed-byte semantic credit is assigned.

Run:

```text
python -m unittest discover -s docs/features/flow-viewer-prefix -p test_viewer.py
cargo test --locked --test flow_viewer_prefix_fixtures
```

Python checks independent prefix-length equations, metadata normalization,
opaque-tail behavior, malformed known prefixes, exact compression framing and
native provenance, then regenerates the packet under a temporary root. Rust
reconstructs every decoded byte, checks exact zlib consumption, verifies known and
unknown extents, and checks independent raw-pattern coverage.

The typed reader requires `FlowViewerPrefixProfile::InflateMetadataQuery` and
independent decoded-byte and known-wide-value bounds. The wide bound includes the
three discarded tuple values. For example:

```rust,ignore
let prefix = carrier.record_121_viewer_prefix(&record, FlowViewerPrefixOptions {
    profile: FlowViewerPrefixProfile::InflateMetadataQuery,
    maximum_decoded_bytes: 2_000_000,
    maximum_prefix_wide_values: 1_000_000,
})?;
let edit = prefix.replace_signed(FlowViewerSignedField::StoredSecond, 1)?;
assert_eq!(edit.result().native_metadata().second_i32, 2);
assert_eq!(edit.coverage().body_validation, FlowViewerBodyValidation::Incomplete);
```

Views expose all known scalar fields, array intervals and raw patterns, the
metadata projection and the entire opaque suffix. Editors replace existing signed,
array or discarded tuple values on an isolated carrier. They preserve lengths,
outer version and suffix, then reparse the same explicit profile. No-op edits
retain original compressed bytes; fresh reversals restore decoded bytes without
requiring the original compressor encoding. The dedicated prefix edit result
keeps incomplete full-body validation visible alongside carrier history.

Source and fixture digests are recorded in `baseline-verification.json`; focused
API acceptance is recorded in `api-verification.json`. Full record-121 body
decoding, other native profiles, runtime output mutation and native application
acceptance remain unverified.
