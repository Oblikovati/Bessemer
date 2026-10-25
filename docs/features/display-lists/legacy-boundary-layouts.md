# Polygon, body and facet boundaries

Reference baseline: specification edition 0.245, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 370 complete part and assembly roots cover versions 2078, 2079, 2089, 2090,
2091, 2101, 2102, 2106, 2107 and 2130, with 99441 independently expected traced
fields. Populated polygon boundary words begin after 2078 (`0x81E`); polygon atom
words begin after 2101 (`0x835`). Empty polygons store neither. Body `unknown_24`
begins at 2091 (`0x82B`). Earlier body/part arrays, component atom lists,
environment tails and material/text/view gates remain independent.

The generator observes complete polygons and bodies at version 2130. Before
removing either polygon word, it checks the stored atom against the reference
result and the raw boundary word against the computed flag. Body word values
are checked before removal. Only primitive intervals change, preserving every
MFC class and object identity. Each final root is parsed to its end at the
version declared by its fixture.

Eighty inline part and assembly roots each contain two complete facets, using
sparse and populated arrays. Facet stamps begin at 2090 (`0x82A`); the entire
cone-culling record begins at 2107 (`0x83B`). Forty independent facet records
check full typed contents and exact writes. Edited records change the stored
facet count, and the stamp where present, without introducing a missing field
into older layouts. These are stored archive values, not validated mesh counts.

Composite edits also change ordinary and simple text, polygons and edges. Tests
compare independently built edited roots, authentic reverts and unchanged
sources. Each boundary rejects republication under a context requiring a missing
polygon, body or facet field. Truncation tests reject partial archives.
Complete externally cached assembly models, empty/null models, all five old-light
classes, and populated/empty 2D and 3D buckets remain covered.

Four owned containers cover baseline and edited part and assembly streams at
2078. Replacement, reopen and revert preserve unrelated streams. Process
verifiers compare complete roots and whole containers in two fresh processes.
Earlier fixture groups remain unchanged.

This checkpoint admits archive contexts from 2078. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
