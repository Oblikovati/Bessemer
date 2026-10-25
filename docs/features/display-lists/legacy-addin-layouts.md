# Earlier add-in and body-property layouts

Reference baseline: specification edition 0.228, dated 2026-10-25. The fixture
manifest records the independent reader hashes used for this checkpoint.

Complete part and assembly fixtures cover versions 2796, 2859, 2860, 2890 and
2891. The 50 fixtures provide 5192 independently expected fields and include all
five old light classes, empty shaded parts, unshaded roots, null component parts
and non-null cache-backed parts. The reference reader parses every fixture to
the end of the stream at its declared version.

Appearance records store `addin_identifier` and `valid_only_for_addin` only
after `0xB2B`. Body records store their single appearance pointer from `0xB4B`
onward. Before that boundary there is no body-property pointer, count or key
in the stream. The reader and writer preserve that absence instead of inserting
a null pointer. Add-in fixtures use Unicode names and noncanonical signed flags.

The generator observes field spans in a complete reference parse, then removes
the absent fields. It removes body pointers only when they are null or references,
so every later MFC slot keeps its original index. A second reference parse checks
the resulting complete layout. Field traces, exact forced writes, independent
edits and reverts are checked by integration tests. Incompatible version contexts
and truncated input fail without publishing partial results. Empty bodies remain
valid across the body-pointer boundary because no changed payload is present.

Two owned containers check document context resolution at version 2796. Four
additional cache-backed assembly containers cover versions 2796 and 2890, before
and after edits. Both complete referenced faces survive edits, replacement,
reopen and revert. Their cache version remains 19000 independently of the display
version. Process verifiers compare full streams, full containers, contexts and
complete linked facet bytes in two fresh processes.

This checkpoint expands archive support to version 2796. Older simple-text width
counts still need their implicit-count layout. Unresolved inline-facet payloads
remain explicitly unsupported. Byte reconstruction does not establish geometric
validity or native application acceptance. Complete schema parity remains tracked
by issue #28.
