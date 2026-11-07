# Biography archives

`bessemer::biography` implements the complete standalone `moBiography_c` archive
in `_MO_VERSION_<n>/Biography`, following [clause 11.9](specification-11.9.md).
This is biography behavior version 1. The root schema is separate from each
record's product file version. The documented serializers have no version gates.

Each record retains the product build fields, uninterpreted member `unknown_10`,
processor and reported OS values, CSD text, language ID, and two snapshots.
Snapshots retain the FILETIME tick count, the low 32 bits of the available
commit-memory observation, and six ordered path groups. Each group contains its
raw path, drive-type code, and complete 64-bit available-disk observation. The
library does not query its host, normalize these observations, or reject stored
default and sentinel values. The six groups and two phases have fixed array
sizes, so callers cannot construct an incomplete record shape.

`BiographyArchive` publishes immutable Send/Sync records. Callers edit an owned
copy and submit it through `with_records`. An empty archive is a completed empty
result. Parsing rejects negative or impossible counts, wrong root classes and
map tags, malformed CStrings, truncation, and trailing payload; it publishes no
partial records. Count allocation is bounded by the input's minimum record size.
Callers can limit enclosing stream decompression before invoking the codec.

Raw records preserve their repeated-string spellings. `expanded_records` returns
a separate view: `*` inherits the preceding expanded record's same CSD or path
field. Paths inherit only at the same phase and group index. Numeric fields
never inherit, empty strings stay empty, and a first-record `*` remains literal.
`abbreviated_records` explicitly expands the input and then substitutes `*` for
nonempty strings equal to the preceding expanded field. Neither method mutates
the source. Callers submit the returned records when they want to change stored
abbreviations. Abbreviation can change CString encoding and prefix choices in
inherited text; semantic comparisons use text values rather than their framing.

No-op saves, equivalent edits, and reverts return the original bytes. Actual edits
retain other fields and CString prefix widths. Serialization does not silently
expand or abbreviate strings. Editing an earlier string can therefore change the
meaning of later stored markers; use the expanded view before editing when those
later values should remain independent.

## Usage

```rust
use bessemer::biography::BiographyArchive;

let source = BiographyArchive::from_bytes(&biography_bytes)?;
let mut records = source.records().to_vec();
if let Some(first) = records.first_mut() {
    first.date_code = 2026100;
}
let edited = source.with_records(records)?;
let output = edited.to_bytes()?;
```

`cargo run --example set_biography_date_code -- part.sldprt
_MO_VERSION_19000/Biography 2026100 edited.sldprt` performs a selected-stream
document edit. `rebuild_biographies` accepts `INPUT OUTPUT` pairs and forces
reconstruction from typed records. Its optional `--abbreviate` flag applies the
explicit native-style abbreviation rule before reconstruction.

## Verification

Five owned vectors include native streams from the blank part, assembly, and
drawing, plus independently assembled empty and mixed archives. The mixed vector
uses extreme numeric values, both string encodings, distinct phase/group paths,
empty strings, and three-record abbreviation chains. The Python reference reader
produced [expected rows](../../../tests/fixtures/biography/expected.tsv) for every
raw and expanded field. [The generator](generate_fixtures.py) requires the
reference `solidworks` directory on `PYTHONPATH`; normal Rust tests embed fixtures
and need neither that checkout nor installed SolidWorks files.

Tests compare every field with that oracle, force byte-identical reconstruction,
and exercise construction, edits, reordering, deletion, empty output, equivalent
edits, and reverts. Every truncation of the mixed vector fails, and every single
byte receives a generated mutation that must either fail or reopen as a stable
complete archive. Other cases cover impossible counts, wrong class/map tags,
nonminimal CString widths, empty and first-record marker rules, and exact numeric
preservation during abbreviation. Published results remain unchanged after
source-byte and caller-record mutations. Every public biography type has a
compile-time Send/Sync check; eight workers repeatedly compare independent edits
with serial reconstruction.

On Windows x64, all 1,115 installed biography streams were independently decoded
and reconstructed from typed fields: 82,487 records and 56,893,189 payload bytes.
Every rebuilt stream matched its source byte for byte. The sorted corpus
manifest SHA-256 is
`69a2df03102514185a83bb2e0f48a27e55590874efb1a85c16fd99b04e6e7791`.
The hash concatenates each sample's UTF-8 relative path with `/` separators, a zero
byte, stream name, another zero byte, and raw stream bytes. Samples are sorted by
relative path; streams retain directory order.

The explicit abbreviation pass also ran across all 1,115 streams. The independent
reader expanded both inputs and outputs and found every stored numeric and
expanded text field unchanged across all 82,487 records.

Independent whole-document edits on the three native kinds changed only the
selected date code. Every other stream retained its exact compressed payload.
Separate processes produced identical edited documents and identical rebuilds
of all five vectors. Temporary outputs were discarded.

`cargo xtask pr` passed locally with 88 integration tests, 47 documentation
examples, and three xtask unit tests. The feature PR records all five platform
gates, including Linux ThreadSanitizer. No dependency was added. This establishes
archive acceptance; native SolidWorks application acceptance and performance
benchmarks remain unexecuted. There is no earlier Bessemer release for a
release-to-release comparison.

## Reference identity

The approved reference is `../inventor-interface`, commit
`dd0537b3a142d8dda00ef846891efc736a6aac46`, with uncommitted schema work. The
snapshot was checked against specification edition 0.110 on 2026-10-24. The raw
full-file SHA-256 was
`15edc977c5213610fa5c49960a4676e1dc515bd64d4dae11987b923388ab09bd`.
The saved UTF-8/LF clause fragment has SHA-256
`72603cd536544bd1057fff04f6a697c5844d94eb8aee6eddeeb51d1fcb8fe5e6`.

| Reference reader | SHA-256 |
| --- | --- |
| solidworks/biography.py | c532531a75b6a7db72379fde6b85ec7421c04064038f799a984104126f14bbdd |
| solidworks/mfcobjects.py | eb7ad6c86ed8e4c4bc9e8b5ad6e14c2dbabc4a5ae53bb6cd9686061cef98aa16 |
| solidworks/swarchive.py | 49daa712eacb3d25077835c0177118e94790d435118f281d03f110e73cf5dbf8 |

The humanizer skill is unavailable; its prose pass remains for maintainer review.
