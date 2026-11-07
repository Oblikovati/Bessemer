# Temporary bucket edges and view triples

Reference baseline: specification edition 0.238, dated 2026-10-25. Reader hashes
are recorded in the fixture manifest.

Complete part and assembly fixtures cover versions 2606, 2626, 2627, 2634, 2635,
2649 and 2650. The 206 fixtures provide 43739 independently expected fields.
They include all five old light classes, empty shaded parts, null component parts,
complete external faces, and populated or empty 2D and 3D buckets. Every final
stream is parsed to its end by the reference reader at its declared version.

Before `0xA5A`, buckets store an edge-visibility array after their main polygon
lists. Each record contains four named i32 words and a polygon list. Before
`0xA4B`, an additional i32 is stored after the first two words. The native reader
discards that local; the oracle captures the actual read value and the codec
retains it as `discarded_old_8`. Edge polygons inherit the enclosing bucket's
coordinate dimension. Nonpositive array and polygon counts remain exact empty
markers.

Component string maps are absent through `0xA59`; the map header and entries
begin at `0xA5A`, when the edge array disappears. View records additionally store
temporary i32 triples only for `0xA42 < v < 0xA5A`, after their sub-buckets.
Fixtures contain populated root triples and negative empty child counts. Aliases
carry no second copy of the referenced object's payload.

The generator observes the later array and map positions in complete reference
parses, inserts the temporary records and removes absent maps. Nested child and
parent insertions can share an original byte position; their physical return
order is preserved. These records contain no MFC objects, so class and object
slot identities remain unchanged. Existing bucket builders still provide the
complete shaded models and explicit external face references.

Tests compare independent fields and exact forced bytes, independently built
edits and reverts, source isolation, inherited coordinate scopes, nested triples,
incompatible contexts and truncated input. Oversized array and triple counts
return typed failures before allocation or publication. Empty records that do
not contain the changed payload remain compatible across the corresponding
version boundary.

Two owned unshaded containers check context resolution at version 2606. Ten
complete shaded assembly containers cover versions 2606, 2634, 2635, 2649 and
2650 before and after edits. All twenty face references resolve to complete
facets through edge, triple, polygon and text edits, replacement, reopen and
revert. Their cache version stays 19000 independently of the display version.
Process verifiers compare complete streams, containers, contexts and linked
facet bytes in two fresh processes.

This checkpoint expands archive support to version 2606. Earlier component-holder
and legacy edge-record branches still need reachability or format coverage.
Unresolved inline-facet payloads remain explicitly unsupported. Byte reconstruction
does not establish geometric validity or native application acceptance. Complete
schema parity remains tracked by #28.
