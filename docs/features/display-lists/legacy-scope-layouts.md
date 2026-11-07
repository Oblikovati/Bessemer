# Three-dimensional buckets before stored coordinate scopes

Reference baseline: specification edition 0.254, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 154 complete part and assembly roots cover versions 1180, 1181, 1182 and
1215, with 49017 independently expected traced and physical fields. Through
1180 (`0x49C`), buckets store neither `has_2d_info` nor its selected transform.
Their polygons use three coordinates per point. From 1181, the stored flag selects
two or three coordinates, with a transform when the flag is nonzero. The direct
view transform remains independent of this bucket coordinate scope.

The generator observes complete buckets at version 1215. It verifies the 2D flag
at the end of the fixed bucket header and locates the selected transform by its
physical start. Before omitting that interval, it expands every populated 2D
polygon to a complete 3D record. Existing X/Y bytes remain exact; each point gains
an independently built Z value. Empty and nonpositive polygons remain unchanged.
The same rule applies inside edge-visibility records. These primitive changes
preserve all MFC class and object slots. Each final root is parsed to its end at
its declared version. Earlier fixture groups remain unchanged.

Tests check the coordinate arity of every populated polygon using its owning
bucket's actual scope. Converted old profiles explicitly store Z=1 and edit it
to Z=2, without changing their X/Y words. Composite edits also change text, fonts,
nodes, edges and facet counts, matching independently built roots and
authentically reverting without source mutation.

Thirty-two inline models each contain two complete facets. Sixteen independent
facet records compare full typed contents and exact writes. The discarded legacy
facet word is absent through 1181 (`0x49D`) and present from 1182. All earlier
body, UI, text, component, material and bucket-list gates remain independent.
Checks for ghost lists and hidden-count words anchor to the always-stored bucket
tessellation number, so omitted 2D headers cannot skip those assertions.

Complete cached models, empty/null models, all five old-light classes and nested
buckets remain covered. Context changes reject missing coordinate-scope fields
and missing facet words. Truncation tests reject partial roots.

Eight owned containers cover baseline and edited part and assembly streams at
1180 and 1182. Their source profiles use 2D buckets: the older containers carry
fully expanded 3D polygons, while later containers retain scoped 2D coordinates.
Replacement, reopen and revert preserve unrelated streams. Process verifiers
compare roots and whole containers in two fresh processes.

This checkpoint admits archive contexts from 1180. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
