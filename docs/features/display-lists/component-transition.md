# Display versions 10000–10999

This checkpoint fills the 10000–10999 interval of the complete part and assembly
display-list codec, using explicit file context 19000. It advances #28. The new
vectors prove wire interoperability for these layouts; native CAD application
acceptance and geometric validity remain separate.

Viewer components add configuration-description text after `0x279C`, fixed state
and a 16-bit solve status after `0x279D`, temporary-fixed state after `0x27B1`,
and flexible state after `0x27C1`. Versions above `0x27B6` and below `0x27E3` also
contain an object pointer that the reference reader labels `unknown_obj`. The
codec retains its object tag, typed viewer body and aliases. Reading an alias
does not recursively restore its body; a viewer can therefore refer to itself
without loading an extra object or losing identity.

The independent fixture builder inserts a new viewer in that pointer slot and
relocates subsequent MFC class/object references by the two newly occupied map
slots. The nested viewer stores its own map identity in the transient pointer.
Four complete roots cover both ends of the populated flexible-state interval and
an independent change from 22 to 23. Other roots cover null transient pointers
and omission before/after the interval. The field oracle walks the nested viewer
once and omits its recursive pointer; forced reconstruction checks the graph tags.

Other boundaries reuse existing codecs. The body trailing word and facet display
colors appear after `0x2791`; graphic-mesh appearance state appears after
`0x2794`. Facets add a face node after `0x279D`, a face normal after `0x279F`,
face-type/edge-ID records after `0x27AD`, another raw flag after `0x27B5`, and a
body atom ID after `0x27E0`. Bucket hatch attributes and hatch-set records appear
after `0x2789`. All earlier float coordinates and raw boolean words remain exact.

The 58 complete shaded boundary/edit roots provide 17,590 independent field
expectations. Four populated transient-viewer roots add 1,344. The 58 unshaded
bucket roots cover both coordinate scopes and supply 12,052: 30,986 new field
comparisons across 120 fixtures. Each manifest records input hashes, reader
provenance, entry budgets and object counts. Existing fixture bytes remain frozen.

The two populated facet display-color arrays add four entries, and the two
facet edge-ID lists add another four, only when their version gates are present.
The two bucket hatch-set records add two entries. The nested viewer adds five
entries through its gobbled-ID and dependency collections, plus one loaded
object; its class descriptor occupies a map slot but does not consume another
loaded-object allowance. Exact limits and failing lower bounds pin these costs.

The shared suites now compare 217 shaded roots and 180 bucket roots, plus the
nine seven-kind geometry roots. Additional assertions check every component-state
boundary, self-alias identity and declared pointer roles. They reject every
prefix of a populated transient-viewer root, compare eight concurrent edits after
mutating the input buffer, reopen an independent edit inside a complete assembly
container, and revert to authentic input bytes.

The process verifier regenerates all 120 new fixtures, checks frozen hashes and
compares forced bytes and field traces from two Rust processes. The earlier
163-, 123- and 134-fixture process gates retain their previous input/output/trace
hashes. Reports record runtime source hashes and the local platform.

```powershell
cargo build --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/verify_component_transition.py target/pinned-1.99.0/debug/examples/rebuild_display_archives.exe
cargo xtask pr
```

The native gate passed all 1,086 streams with 1,849,563 independent field
comparisons and identical results from two processes. `cargo xtask pr` passed on
Windows x64 with Rust 1.99.0 MSVC, including all 176 doctests. Earlier
intervals, other file-version contexts, unresolved class bodies, whole-document
context and validation, and remaining schemas are still open. The maintainer-side
humanizer skill is unavailable; its prose pass remains for review.
