# Earlier line-visibility layouts

Reference baseline: specification edition 0.237, dated 2026-10-25. Reader hashes
are recorded in the fixture manifest.

Complete part and assembly fixtures cover versions 2650, 2687, 2688, 2710 and
2711. The 150 fixtures provide 30175 independently expected fields. They include
all five old light classes, empty shaded parts, null component parts, complete
external faces, and populated or empty 2D and 3D buckets. Every final stream is
parsed to its end by the reference reader at its declared version.

Bucket polygons call `moLineViz_c::Serialize` with flag zero. In that overload,
`visibility` is stored only from `0xA80` onward. Earlier records end after color;
their native in-memory visibility is derived from the style name. The codec
preserves the absence of a stored word. The fixtures use noncanonical signed
minimum values and independent edits to the next signed value after the boundary.

## Component-holder reachability

The `moComponentPropertyHolder` name predicate changes at `0xA96`, inside this
interval. That object has no incoming generic-object route here. The complete
archive decoder's enclosing entry paths have these guards:

| Generic-object entry path | Required enclosing guard |
|---|---|
| Bucket thread-map keys | `v > 0xAD2` |
| Scene floor selection | `v > 0xF5E`, with a stored scene |
| Component or part graphics-reference managers | `v > 0x2B88` or `v > 0x2B9D` |
| Configuration/footer objects and view matrices | Assembly footer `v > 0x2403`; part footer `v > 0x2B5E`, with further inner guards |

The holder's recursive generic pointer cannot create an initial entry. Other
root object slots validate their declared class roles; the regression test rejects
a holder substituted for a typed view bucket. Drawing bucket archives require
model versions from `0x27CC`, outside this interval. No unreachable lower-version
holder decoder was added. Complete fixtures and context rewrites on both sides
of `0xA96` retain identical stored layouts.

## Verification

The generator observes each visibility word in a complete reference parse before
removing or replacing it, then parses the final bytes again. Tests compare
independent fields and exact forced bytes, independently built edits and reverts,
source isolation, incompatible contexts and truncation failures. Empty records
that have no line properties remain unchanged across the visibility boundary.

Two owned unshaded containers check context resolution at version 2650. Four
complete shaded assembly containers cover versions 2650 and 2688 before and after
edits. Both complete referenced faces survive visibility, polygon and text edits,
replacement, reopen and revert. Their cache version stays 19000 independently
of the display version. Process verifiers compare complete streams, containers,
contexts and linked facet bytes in two fresh processes.

This checkpoint expands archive support to version 2650. Earlier bucket edge-visibility
arrays, string-map framing and temporary view triples still need coverage.
Unresolved inline-facet payloads remain explicitly unsupported. Byte reconstruction
does not establish geometric validity or native application acceptance. Complete
schema parity remains tracked by #28.
