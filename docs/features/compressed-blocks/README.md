# Compressed-block carriers

`bessemer::block_stream` implements clause 9.3 of the SolidWorks specification.
`BlockStream::from_bytes` requires the caller to choose `LengthPrefixed` for body
partition carriers or `GuidOnly` for `__ZLB` carriers, and to set the maximum total
expanded size. Both modes accept multiple chunks and multiple blocks per chunk.

Parsing checks the chunk GUID, each prefixed chunk boundary, the paired zero
terminator, and the specification's 1,048,576-byte block limit. Each zlib stream
must finish, consume exactly its declared compressed bytes, produce exactly its
declared expanded bytes, and pass its checksum. Empty carriers and chunks with no
nonempty blocks are rejected. A budget failure is distinct from corrupt zlib data.
No partially decoded result escapes after a failure.

`replace_chunk` accepts nonempty replacements within the original total expansion
budget. It changes no other chunk. Identical content retains the original encoding;
restoring the original content removes the pending replacement. Serialization
splits changed content into blocks of at most 1 MiB and writes zlib streams at
compression level 1. It preserves every unchanged chunk byte for byte.

These are carrier operations. An inflated Parasolid transmit file or add-in archive
is returned as bytes; its geometry, topology, record schema, and consistency with
other document streams are not certified. Chunk insertion and removal are not
implemented. The replacement example operates on an extracted carrier, leaving
document naming and model consistency to the caller.

## Evidence and provenance

This is carrier behavior version 1, introduced by issue #4. The reference checkout
is `../inventor-interface`, at commit `dd0537b3a142d8dda00ef846891efc736a6aac46`
with uncommitted specification work. The files actually consulted have these
SHA-256 identities:

| Reference file | SHA-256 |
| --- | --- |
| docs/solidworks-file-format/specification.md, edition 0.89 | f4b4f472a751fbe1ecafc036a240c5e8dfb026ec029d5be6e05d6fa4f4bd514d |
| solidworks/swcover.py | f11d8e93420921c7ba4aaa42406d95365a856472d654e207cc6fb178ae3225b2 |
| solidworks/swcontainer.py | fa3eaf9159ae5deff97559ae7c4feb2a0d2c4425a86a62b4a99944c323ccec19 |

The native carrier fixture is the unmodified `Contents/Config-0-Partition` stream
extracted by `swcontainer.Document` from the owned `blank-part.SLDPRT` fixture.
Its two chunk payloads were independently expanded with `swcover.zlb_blocks` and
Python zlib. Their owned expected bytes have these identities:

| Expected payload | Bytes | SHA-256 |
| --- | --- | --- |
| tests/fixtures/blocks/blank-part-chunk-0.bin | 307 | c0b8b08d8fa2906d6deec5a3b5ced130906fd61fa523ecea3d371cea4d63bb2f |
| tests/fixtures/blocks/blank-part-chunk-1.bin | 411 | f2a380d2f76fd50d20634d35230680382a38f9ca1692843ea3918bb2c066230a |

Integration tests compare native expanded bytes and exact re-encoding, exercise
both framing modes with independent zlib vectors, and check multi-block writes,
malformed lengths, every truncation of a small carrier, invalid terminators,
checksums, trailing compressed bytes, budget failures, equivalent edits, reverts,
and isolation after success and failure. Public types have compile-time Send/Sync
assertions. Eight concurrent workers produce the same bytes as each other while
leaving the source unchanged.

On Windows x64, the Python reference independently verified a 1,048,577-byte
replacement written by the Rust example. It checked both resulting blocks, their
complete concatenated payload, the chunk boundary, and the exact unchanged second
chunk. Two separate Rust processes produced identical bytes. Temporary outputs
were discarded. The local PR gate passed. No new dependency was added.

Native SolidWorks application acceptance, full installed-corpus coverage,
and performance benchmarks remain unexecuted for this layer. The shared full gate
now runs its tests under ThreadSanitizer, as recorded by the
[version-history feature](../version-history/README.md).
Cross-platform verification is recorded by the feature PR's five CI checks.
The humanizer skill is unavailable; its prose pass remains for maintainer review.
