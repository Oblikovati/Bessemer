# Caller-supplied Inspection source identity

Feature #195 covers the separate comparison described in §14.16 and
`inspectionrecords.compare_document_source`. The caller supplies a revision-bound
document occurrence and source bytes. The operation compares the complete byte
length and lowercase MD5 checksum against the stored strings without opening
`SourcePath`. A mismatch completes the comparison; it does not authenticate the
source or establish native application compatibility.

`InspectionRecords::compare_document_source` accepts a validated Document ID and
a byte slice. It returns stored and actual identity fields under `values`,
completion, exact check coverage, semantic exclusions and a copied-document
history relation anchored to the retained immutable project. Foreign, stale and
wrong-role IDs fail before checksum computation. Mismatched or absent metadata
produces a completed comparison with separate size/checksum flags.

The checksum implementation is private, uses operation-local state and fixed
integer parameters, and adds no dependency. It hashes arbitrary supplied bytes,
including empty input; no text decoding or file-format recognition is performed.

The owned corpus contains 172 cases: the seven literal test vectors in
[RFC 1321 A.5](https://www.rfc-editor.org/rfc/rfc1321.html), every
input length from zero through 129, larger block boundaries, homogeneous zero/one
patterns, byte cycles and one million `a` bytes. `freeze_identity.py` records
Python hashlib expectations. `verify_dotnet.ps1` checks every digest with the
installed .NET provider and requires exact packet EOF.

The owned metadata packet adds 22 absent, empty, whitespace, Unicode,
out-of-range-size and checksum-spelling cases. The unchanged Source reader and
comparison function accepted their exact expected results and all 172 binary
inputs from an immutable single-reader snapshot. Its imports are standard-library
modules; no other Source modules were copied or allowed as live imports.

The installed tutorial PDF is pinned at 164,616 bytes and checked against its
stored checksum. Python and .NET also agree on a same-length last-byte mutation
and a truncated copy. The Source operation completed both mismatching comparisons
with the expected separate size/checksum flags. The focused Rust suite exercises
all 172 checksum vectors, 22 metadata cases and three native PDF variants against
these frozen independent expectations. It also checks duplicate Uid occurrences,
input mutation, source isolation, complete provenance and every published type's
Send/Sync bounds. Eight workers each repeat the full input matrix eight times
against the complete serial results in a fixed order.

Stored size and checksum text will remain uninterpreted: missing and empty
metadata stay distinct, leading zeros in a stored size are not normalized, and
uppercase stored checksum text is not silently lowercased. Two fresh processes
each passed nine tests and exported 197 complete comparisons. The packets were
byte-identical. The unchanged Source reader and comparison function matched every
stored/actual value and flag; the independent result decoder also checked exact
caller XML, coverage and provenance invariants through packet EOF. The immutable
single-reader snapshot and import/content guard passed.

Complete local and exact-head remote checks remain required before
integration. Application acceptance remains separate from this lexical checksum
comparison.
