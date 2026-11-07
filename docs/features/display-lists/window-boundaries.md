# Earlier display layouts and the uncounted-window interval

This checkpoint admits display versions 8000–8999 outside `0x1FAE`–`0x1FCE` for
complete part and assembly archives. Native file context remains explicit and
independent. It advances #28; documented-schema parity is still incomplete.

Clause 13, Note 2 states that versions above `0x1FAD` and below `0x1FCF` store one
or two transforms and two words per open window, without storing the number of
windows. The approved reference rejects that span and records no sample for it.
Bessemer now gives a specific typed unsupported error for all 33 versions, before
interpreting payload fields. It does not guess a window count from the view count
or claim that a partial read is a complete stream. Both adjacent versions remain
readable. No opaque remainder fallback was added.

Outside that interval, annotation records add `unknown_i` after `0x1FA8` and their
second string collection after `0x1FD4`. The annotation-view collection also adds
its trailing two words after `0x1FA8`. User environments add `unknown_1c4` from
`0x1FF1`; body tessellation adds its temporary string after `0x2017`. Units are
absent through `0x1FB2`. That threshold lies inside the uncounted-window span;
complete-root fixtures cover the valid versions on either side. Populated 3D-view
planes retain the existing versioned layout, including its `0x2012` boundary.

The annotation physical minimum is 16 bytes before the state word, 20 bytes when
the state word is present, and 24 bytes when the second string count is present.
The later CString minima remain 25 and 27 bytes. Six complete unshaded roots
contain 100 minimal records with zero-length nested arrays. Their remaining bytes
at the count boundary are too short for an assumed 24-byte stride in the earlier
layouts. They independently pin both omission and the correct physical guard.

Those minimal roots charge 152 entries: one root view, three splitter panes,
40 fonts, two 3D views with four section planes, one annotation view, one display
item and 100 annotation records. They load 42 MFC objects. Empty nested arrays
charge zero. The completed unshaded result retains every prefix field and ends
at its declared zero shaded-data flag; it is not a partially decoded shaded root.

Fifty complete shaded boundary/edit roots and six minimal annotation roots
provide 15,801 independent field expectations. Fifty unshaded bucket roots in
both coordinate scopes add 8,992. The total is 24,793 field comparisons across
106 new fixtures. Manifests record wire hashes, reference-reader provenance,
entry budgets and object counts. Existing fixtures remain frozen.

The shared suites now compare 330 general roots, 280 bucket roots and nine
seven-kind geometry roots. New assertions pin omitted fields, exact earlier
annotation strides, both valid sides of the excluded span and typed rejection of
every excluded version for each document kind. Exact resource limits still pass
at their boundary and fail one below. Tests reject every prefix of populated
8000 roots, compare independent ambient edits and reverts, preserve inputs after
source-buffer mutation and eight concurrent edits, and reopen both complete
native container kinds without changing any other stream. Authentic container
reverts remain byte-identical.

The process gate regenerates all 106 new fixtures, checks their frozen hashes and
compares forced bytes and field traces from two Rust processes. The earlier
111-, 120-, 163-, 123-, 134- and 86-fixture gates retain their input/output/trace
hashes. Native CAD application acceptance and geometric validity remain separate.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_window_boundaries.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

`cargo xtask pr` passed on Windows x64 with Rust 1.99.0 MSVC, including all 176
doctests. The native gate passed all 1,086 streams with 1,849,563 independent field
comparisons and identical reconstruction in two processes.
Earlier intervals, unresolved bodies, complete-document context/validation and
remaining schemas are still open. The maintainer-side humanizer skill is
unavailable; its prose pass remains for review.
