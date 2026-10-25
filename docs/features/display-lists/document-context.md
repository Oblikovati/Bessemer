# Display context resolved from documents

Feature #28 now provides `DisplayArchiveContext::from_document`. It resolves
exact `_DL_VERSION_<integer>/DLUpdateStamp` names and parses the optional
`FaceTessellations/Directory` declaration stream. The caller supplies the document
kind, native file version, tessellation archive context, tessellation limits and
maximum expanded directory bytes. The resolver does not guess these values from
the container's version 4 header or substitute a default display version.

Signed and unsigned display-marker aliases normalize once; missing, malformed
and conflicting markers fail. Absent directories produce an empty component set.
A present empty directory must still parse completely. Malformed, corrupt or
over-budget directories fail rather than silently becoming absent. Current
pending document replacements are read through the existing bounded extraction
API, including its CRC checks.

External component IDs are deduplicated into an owned, immutable ordered set.
Case, Unicode and literal spelling remain significant. The original directory
bytes retain their own string encodings and duplicate records. The directory's
archive version is independent of the display version: an owned vector combines
a display version of 19000 with directory version `0x3B1C` and legacy tessellation
number pairs. Reading that directory under the wrong version fails.

This operation resolves declarations. It does not read external face blocks,
validate index/hash or face-count links, or certify geometry. A directory may name
cached components or blocks that are irrelevant to parsing the display stream.
The separate [external face-link check](external-face-links.md) validates named
blocks and requested cache positions; a resolved context alone is not evidence
that those checks passed. Geometry validation remains unfinished.

The owned corpus has five complete positive documents, five marker failures or
alias cases, one directory CRC failure, and independently patched display-stream
edit results. Python's ZIP primitives construct filler-free version-4 records;
the approved container, directory and display readers validate the positive
payloads with explicit versions. Seven integration tests cover exact context,
case-sensitive IDs, duplicates, missing directories, malformed declarations,
limits, corruption, source ownership, concurrency, pending replacements,
independent edits, complete reopening, unrelated streams and authentic reverts.

`rebuild_display_documents` resolves context from each supplied container, forces
`DisplayArchive::new` reconstruction, checks display bytes, replaces that stream
and serializes the complete document. The verifier compares both complete output
files and context traces byte for byte against independent expectations in two
isolated Rust processes. It batches native documents to bound aggregate memory.
The accepted-path manifest prevents an accidental reduction of the 1,086-document
native corpus.

On Windows x64 with Rust 1.99.0, all five owned documents and 1,086 native documents
passed. The native contexts contained 1,258 component IDs after per-document
deduplication. The hash of sorted corpus-relative path, NUL and complete source
file bytes was `4b9f6b0846d0ea1aaf0a858291d5dc6972ac6b1182fa6d3268c3fca38a809511`.
This whole-file hash is separate from the existing display-stream corpus hash.

```text
cargo test --test display_document_context
cargo build --example rebuild_display_documents
python -B -X utf8 docs/features/display-lists/verify_document_contexts.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe
python -B -X utf8 docs/features/display-lists/verify_document_contexts.py target/pinned-1.99.0/debug/examples/rebuild_display_documents.exe --native
```

Full documented-schema parity and native application acceptance remain
unfinished. The humanizer skill was unavailable; prose remains subject to
maintainer review.

The seven focused tests and `cargo xtask pr` passed, including 180 documentation
tests. All earlier process checks passed. The independent display-stream audit
also retained 1,086 accepted streams and 1,849,563 field comparisons with unchanged
hash `94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
Cross-platform CI remains pending the complete feature PR.
