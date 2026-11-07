# Exploded lists, component zones and textured planes

Reference baseline: specification edition 0.247, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 262 complete part and assembly roots cover versions 1875, 1876, 1896, 1897,
1899, 1900 and 2011, with 79234 independently expected traced fields. Exploded
polygon lists are absent through 1875 (`0x753`); component `is_a_zone` words are
absent through 1896 (`0x768`). The entire textured-plane list, including its count,
is absent through 1899 (`0x76B`). Existing font, solid-fill, body, polygon, facet,
component-atom, environment and material/text gates remain independent.

The generator observes complete source records at version 2011. It identifies
both exploded-list objects returned by each bucket before removing their exact
physical intervals, and checks the final component zone word before removal.
Textured-plane pointers are null; their transforms are complete. These changes
introduce no new MFC class or object slots. Every final root is parsed to its end
at the version declared by its fixture. Earlier fixture groups stay unchanged.

Above the plane boundary, shaded roots contain two independently built planes.
One stores a noncanonical matrix-presence byte and matrix; the other omits the
matrix. Both retain their translation, scale, flags and selected perspective
payloads. Separate reference traces check 2112 transform fields, including raw
floating-point bits. An edited fixture changes the first plane's translation
alongside font, text, polygon, edge and facet-count edits. Exact writes and
authentic reverts preserve the original source.

Fifty-six inline part and assembly roots retain two complete facets each. Their
layout matches the independently built version-2011 records, checked as complete
typed contents and forced bytes at each actual archive version. Complete cached
models, empty/null models, all five old-light classes and nested 2D/3D buckets
remain covered. Truncation tests reject partial roots. Incompatible contexts
require newly stored fields, while parts remain compatible across the
assembly-only zone-word boundary.

Eight owned containers cover baseline and edited part and assembly streams at
1875 and 1900, proving both omitted and populated textured-plane layouts through
replacement, reopen and revert. Unrelated streams remain unchanged. Process
verifiers compare complete roots and whole containers in two fresh processes.

This checkpoint admits archive contexts from 1875. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
