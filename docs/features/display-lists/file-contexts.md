# Native file-version context

This checkpoint removes the artificial file-context-19000 restriction from the
part and assembly display-list archive codec. It advances #28. The caller still
supplies the unsigned current-file version explicitly; the stream does not store
it. Bessemer keeps that value on the immutable archive instead of consulting a
process-wide setting or inferring it from the display-list version.

In the current approved readers, `dl_ui._file_version` affects the two tangent
bucket lists. They are absent below `0x224` and present from that value. The
reference's default is 19000, but its explicit archive override selects the other
layout. Bessemer now applies the same condition in the common bucket reader,
including nested and detailing buckets. Other display-version gates are unchanged.
An absent list is distinct from a stored empty list; no count field or placeholder
is synthesized for the absent case.

Owned complete-root fixtures use file contexts 0, `0x223`, `0x224`, 18000, 18999,
19000 and `u32::MAX`. Seventy-two unshaded roots cover display versions 9000,
16000 and 19000, both coordinate scopes, part/assembly context, populated and
empty tangent lists, and independent display-mode edits. Fourteen complete shaded
roots replay both owned native blank documents under each file context. These
have no view bucket, so their identical bytes and decoded contents must still
retain the caller's distinct context value.

The tangent lists contain one polygon each when present. Their coordinates,
optional type data and raw flags use the existing versioned polygon codecs. The
new lists add ten aggregate entries in 3D or eight in 2D; they add no loaded MFC
objects. Children and the original detailing bucket remain empty, while their
stored negative empty counts are retained when the context includes the lists.
Exact resource limits and failing lower bounds pin both layouts.

Eighty-six fixtures supply 23,795 independent field expectations. They retain
explicit file-version headers in the fixture format and in the reconstruction
example's context frame. The process verifier regenerates every fixture, checks
frozen wire hashes and compares forced reconstruction and field traces from two
isolated Rust processes. Shared process framing now accepts the supplied file
version while preserving 19000 for the existing corpus callers.

Seven focused tests check complete fields and forced bytes, context retention,
list omission through child buckets, coordinate scope and exact limits. They
reject construction under the opposite tangent-layout context, every prefix of
two populated roots, impossible polygon counts and trailing bytes. Independent
mode edits match owned golden bytes, including eight concurrent edits after input
buffer mutation. Complete-document checks reopen the edited stream with its
explicit context, compare every other stream, and restore authentic container
bytes. Existing tests now accept alternate file versions for roots whose layout
is unaffected, while retaining their unsupported display-version checks.

The earlier 111-, 120-, 163-, 123- and 134-fixture process gates retain their
input, output and trace hashes. No existing fixture bytes were changed. The
native corpus still uses its recorded file context 19000; these synthetic older
contexts prove wire branches, not native application acceptance for older files.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_file_contexts.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

`cargo xtask pr` passed on Windows x64 with Rust 1.99.0 MSVC, including all 176
doctests. The native gate passed all 1,086 streams with 1,849,563 independent field
comparisons and identical reconstruction in two processes.
Earlier display-version intervals, unresolved class bodies, complete-document
context resolution/validation and remaining schemas are still open. The
maintainer-side humanizer skill is unavailable; its prose pass remains for review.
