# Earlier polygon-region layouts

Reference baseline: specification edition 0.233, dated 2026-10-25. Reader hashes
are recorded in the fixture manifest.

Complete part and assembly fixtures cover versions 2725, 2748 and 2749. The 94
fixtures provide 18896 independently expected fields. They include all five old
light classes, empty shaded parts, null component parts, complete external faces,
and populated or empty 2D and 3D buckets. Every final stream is parsed to its end
by the reference reader at its declared version.

Positive-count polygons store `new_region` only after `0xABC`. Earlier layouts
contain no region word. Nonpositive polygon counts end the record before any
region attributes, including at later versions. Empty buckets and null bucket
roots therefore keep the same layout across this boundary.

Introduced region words use a noncanonical signed minimum value in base fixtures
and zero after edits. The independent reader normally folds the stored word into
flag bit `0x8`. The oracle captures the actual argument to that flag operation
before it is normalized, then records a separate physical scalar expectation.
The regular reference trace still checks the resulting logical flags. This
verifies both raw words and derived behavior without inferring one from the other.

The fixtures reuse complete bucket builders and relocate later MFC references
when inserting buckets before shaded models. Existing thread-pattern fixtures
remain unchanged. Region edits also exercise polygon smoothing, text and width
edits. Tests compare independently built bytes, authenticate exact reverts, and
check source isolation, incompatible contexts, truncated input and concurrent
shaded-assembly construction.

Two owned unshaded containers check context resolution at version 2725. Four
complete shaded assembly containers combine populated buckets and non-null
models at versions 2725 and 2749, before and after edits. Both referenced faces
remain complete through raw region-flag edits, replacement, reopen and revert.
Their cache version stays 19000 independently of the display version. Process
verifiers compare complete streams, containers, contexts and linked facet bytes
in two fresh processes.

This checkpoint expands archive support to version 2725. Earlier component
display-mode layouts still need their own coverage. Unresolved inline-facet
payloads remain explicitly unsupported. Byte reconstruction does not establish
geometric validity or native application acceptance. Complete schema parity
remains tracked by #28.
