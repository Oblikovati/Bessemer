# Header archives

`bessemer::header` implements the standalone `moHeader_c` archives in `Header2`
and `Contents/Config-<n>-ModelHeader`, following [clause 11.8](specification-11.8.md).
This is header behavior version 1. Callers supply the outer archive version;
`ArchiveVersion::from_stream_name` can extract it from a `_MO_VERSION_<n>/...`
stream name. Signed 32-bit versions are normalized to unsigned values.

The typed graph contains headers, string arrays, object lists, external-object
lists, revision logs, stamps, string handles, and modern external references.
Every documented field has a typed representation, including fields whose
meaning remains unknown. Null pointers differ from version-absent fields.
Bounding boxes retain all ten IEEE-754 bit patterns, including signed zero and
NaN payloads, without making claims about geometric validity.

Users can carry a stored version-marker string. That version controls the header
body through `unknown_60`; the suffix returns to the outer version. The marker
remains in the stored user array. Class schemas are retained separately from
archive versions. External references below version 0xF18 are explicitly
unsupported because their documented legacy layout remains incomplete.

`DocumentHeader` publishes immutable, Send/Sync contents. Callers clone the graph,
edit owned values, and submit them through `with_contents`. Validation rejects
missing and orphan objects, incorrect field presence, wrong reference classes,
inconsistent shared-name variants, and contradictory box flags. Failed edits
publish no partial result and leave the source unchanged. Serialization assigns
wire indices in traversal order, preserving shared identities and keeping
distinct equal objects distinct.

Both decoding and encoding use explicit work stacks. Cyclic generic lists are
supported, and deep nesting does not consume the thread's call stack. Decoding
requires an explicit maximum object count and checks collection lengths against
remaining input before allocation. The enclosing document's stream-size limit
can bound decompression before header parsing.

No-op saves, equivalent edits, and reverts preserve original bytes. Changed
archives retain field values and CString framing; object tags and collection
counts may use a shorter valid representation when rebuilt. Header validation
checks archive structure, not CAD topology, geometry, or native application
acceptance.

## Usage

```rust
use bessemer::header::{ArchiveVersion, DocumentHeader, HeaderObject};

let source = DocumentHeader::from_bytes(&header_bytes, ArchiveVersion::new(19000), 100_000)?;
let mut contents = source.contents().clone();
if let Some(node) = contents.objects.get_mut(&contents.root) {
    if let HeaderObject::Header(record) = &mut node.object {
        record.next_id = 123;
    }
}
let edited = source.with_contents(contents)?;
let output = edited.to_bytes()?;
```

`cargo run --example set_header_next_id -- part.sldprt 19000 Header2 123 edited.sldprt`
performs the complete selected-stream edit. It does not synchronize redundant
headers or interpret the allocation counter's effect on the native application.
`rebuild_headers` accepts `VERSION INPUT OUTPUT` triples and reconstructs each
raw archive from typed contents, bypassing the original-byte preservation path.

## Verification

The owned fixtures contain the three native blank-document headers and 77
independently assembled vectors around every documented version threshold,
including all external-reference gates and three embedded-version overrides.
[The fixture generator](generate_fixtures.py) verifies its output with the Python
reference reader. Regeneration requires that reference's `solidworks` directory
on `PYTHONPATH`, followed by `cargo fmt`. Normal Rust tests use embedded fixtures
and require neither the reference checkout nor an installed SolidWorks corpus.

Tests rebuild every vector byte for byte, check native values, truncate every
prefix of a modern synthetic header, and reject impossible collection counts,
bad references, unsupported legacy bodies, invalid graphs, and mismatched field
gates. They check exact object-budget boundaries, nonminimal count preservation,
shared external handles, distinct equal objects, cyclic lists, 65,536-element
collections, and a 30,000-level graph on a 128 KiB thread stack. Every public
header type has compile-time Send/Sync checks. Eight concurrent workers repeatedly
read and edit isolated graphs and compare their output with serial reconstruction.

On Windows x64, the independent Python reader checked all 2,201 installed header
streams, covering 14,218,178 payload bytes. The Rust example reconstructed every
stream from typed fields and reproduced every source byte. The corpus manifest
SHA-256 is `2005a3fa59a2e1453380dca647f62706d15622145a24270019a2db5decbfa19d`.
Its input concatenates each sample's UTF-8 relative path
with `/` separators, a zero byte, stream name, another zero byte, and raw stream
bytes. Samples are sorted by their relative path; streams retain directory order.

Whole-document edits on the three owned native kinds were checked independently:
only `next_id` changed, and every other stream retained its compressed payload.
Two separate processes produced identical edited documents. Separate-process
reconstruction also matched for all 80 owned header vectors. Temporary outputs
were discarded.

`cargo xtask pr` passed locally with 76 integration tests, 39 documentation
examples, and three xtask unit tests. It checks formatting, clippy, build, tests,
and documentation examples.
The feature PR records the five platform gates, including the Linux ThreadSanitizer
run. No dependency was added. Native SolidWorks application acceptance and
performance benchmarks remain unexecuted; there is no earlier Bessemer release
for a release-to-release comparison.

## Reference identity

The approved reference is `../inventor-interface` at commit
`dd0537b3a142d8dda00ef846891efc736a6aac46`, with uncommitted schema work. The saved
clause was checked against specification edition 0.109 on 2026-10-24. Its raw
full-file SHA-256 was
`48bbcc6b1155d4cf756c3f58906f0795eabc1dfd6d9b42f4dc1ea95f56c9ea91`.
The saved UTF-8/LF clause fragment has SHA-256
`43444e44bad425bc479c33ea7a1c04e5e0048887cb290478aef453f40a4383fd`.

| Reference reader | SHA-256 |
| --- | --- |
| solidworks/modelheader.py | b925126581478aa1a09539ddd7697767720864585ca970ff17e8d48b9f9ea016 |
| solidworks/mfcobjects.py | eb7ad6c86ed8e4c4bc9e8b5ad6e14c2dbabc4a5ae53bb6cd9686061cef98aa16 |
| solidworks/swarchive.py | 49daa712eacb3d25077835c0177118e94790d435118f281d03f110e73cf5dbf8 |
| solidworks/modelhistory.py | 86821847525031f37eab8479afe4acda16bb335748f5c49ce132bf51a3d5d5cd |

The humanizer skill is unavailable; its prose pass remains for maintainer review.
