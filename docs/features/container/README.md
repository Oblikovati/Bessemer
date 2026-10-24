# SolidWorks container foundation

Bessemer follows Corliss's separation between containers, stream codecs, typed
records, document models, and editing operations. Only the container layer is
implemented here. Its format authority is clause 7 and the editor requirements
in clause 15 of the SolidWorks specification in `../inventor-interface`.

The implementation reads version-4 headers, recovers the three archive signatures
from the file, lists streams, and extracts stored or raw-deflate streams. Extraction
checks expanded length and CRC-32. `read_stream_limited` lets the caller bound the
expanded allocation. Parsing checks structure; it does not inflate every stream or
certify model validity. Unsupported container versions, compression, encryption,
and data descriptors return errors.

`replace_stream` edits an existing stream. Serialization with no edits returns the
original file byte for byte. With edits, it keeps the key, record order, names,
date-time fields, and every untouched stream's compressed bytes. It rebuilds the
directory and omits filler, as permitted by clause 15 [W-06]. It does not create
documents, add or delete streams, or validate the meaning of a replacement payload.
The caller must supply consistent stream contents.

An identical replacement retains the stream's original compression. Restoring the
original payload after an intermediate edit removes that pending edit; if no other
edits remain, serialization returns the original whole file. This fixes issue #5
in container behavior version 2. Equality checks are bounded by the supplied
replacement length, and a corrupt original stream can still be explicitly replaced.
Migration from behavior version 1 to 2 changes only equivalent-edit handling; no
on-disk migration is needed. The former recompression behavior is retired by this
recorded migration, and the owned fixture bytes remain unchanged.

## Evidence

Reference revision: `inventor-interface` commit
`dd0537b3a142d8dda00ef846891efc736a6aac46`, specification edition 0.89.
The owned fixtures are native blank part, assembly, and drawing documents saved
with SolidWorks 2026 SP2.1 on 2026-10-24. Their provenance and hashes are in
`tests/fixtures/README.md`.

`streams.tsv` records all 113 stream names, expanded sizes, and CRC-32 values
obtained with the Python reference reader. Tests compare extraction against that
baseline, check exact no-op output, reopen edited containers, and verify input
isolation. Independent synthetic records cover CRC errors, malformed lengths,
unsupported flags and methods, truncation, and empty replacements. A deliberately
different deflate representation checks preservation of untouched compressed bytes.
Eight concurrent workers exercise reads and isolated edits against serial results.

On Windows x64, `cargo xtask pr`, `cargo deny --all-features check`, and `cargo audit`
passed. The Rust replacement example was also used to rewrite each native fixture's
`PreviewPNG` stream with its existing contents. The Python reference independently
reopened all three outputs, checked every extracted stream, and compared every
untouched compressed payload byte for byte. Temporary outputs were discarded.
The required humanizer skill is unavailable in this checkout; its prose pass is
left to maintainer review, as permitted by the contributor guide.

These checks establish container behavior on those fixtures. They do not establish
SolidWorks application acceptance of Bessemer's output or coverage of the installed
1,115-file corpus. The upstream application acceptance described by Annex F.4
supports the filler-free layout; it is not a live test of this implementation.

## Remaining interoperability layers

1. Decode previews, compressed-block carriers, and XML metadata using the reference
   reader and owned expected outputs.
2. Decode archive primitives and typed model records for parts, assemblies, and
   drawings. Keep unknown payload bytes available for lossless edits.
3. Decode Parasolid partitions, bodies, topology, geometry, and rollback deltas.
4. Add typed edit operations with history and validation contracts. Confirm each
   operation against independent expected results and native application opens.

The schema remains under development. Each layer needs its own feature issue,
fixtures, and acceptance gate before its public API is added. No placeholder model
or geometry API is shipped. Legacy compound files and unobserved container versions
remain outside this first slice. ThreadSanitizer, application opens, cross-platform
CI, previous-release benchmarks, and full-corpus checks must be recorded separately
when executed. There is no previous Bessemer release to benchmark.
