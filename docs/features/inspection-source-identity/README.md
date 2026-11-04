# Caller-supplied Inspection source identity

Feature #195 covers the separate comparison described in §14.16 and
`inspectionrecords.compare_document_source`. The caller supplies a revision-bound
document occurrence and source bytes. The operation compares the complete byte
length and lowercase MD5 checksum against the stored strings without opening
`SourcePath`. A mismatch completes the comparison; it does not authenticate the
source or establish native application compatibility.

This directory currently contains independent preparation oracles. No Rust
checksum or source-comparison API has been implemented here. Bessemer and Corliss
currently have no MD5 implementation or dependency. The required compatibility
operation will remain separate from a general cryptography API.

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
with the expected separate size/checksum flags. These are preparation oracles,
not acceptance of a Rust source-comparison implementation.

Stored size and checksum text will remain uninterpreted: missing and empty
metadata stay distinct, leading zeros in a stored size are not normalized, and
uppercase stored checksum text is not silently lowercased. Required gates include
wrong-role, stale and foreign identities, caller-input isolation, the installed
tutorial PDF and modified/truncated bytes, original Source agreement, independent
hash expectations, fresh-process equality and concurrent comparisons. Complete
local and exact-head remote checks remain required before integration.
