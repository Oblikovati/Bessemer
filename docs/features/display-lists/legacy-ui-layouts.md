# Early UI, sketch text, body metadata and facet arrays

Reference baseline: specification edition 0.251, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 442 complete part and assembly roots cover versions 1286, 1287, 1289, 1290,
1300, 1301, 1325, 1326, 1327, 1331, 1332 and 1645, with 146786 independently
expected traced and physical fields. They cover six storage boundaries:

- Body `unknown_20` is absent through 1286 (`0x506`).
- The ghost-transform flag and selected transform are absent through 1289 (`0x509`).
- The secondary user-environment splitter begins at 1301 (`0x515`).
- Sketch-text `unknown_a8` is absent through 1325 (`0x52D`).
- Font `unknown_28` is absent through 1326 (`0x52E`).
- Facet edge-type arrays are absent through 1331 (`0x533`).

Each root adds two explicit sketch-text records, including ANSI and Unicode text,
with and without stored matrices. Both font kinds remain covered. The primary
splitter remains stored throughout the interval. Nested 2D and 3D view buckets
cover zero and nonzero ghost flags and complete selected transforms.

The generator observes complete source records at version 1645. It identifies
the secondary splitter by the exact reference object returned by the environment
reader. Ghost intervals are located after the observed bucket and optional
ordinary transform; the raw ghost flag is checked against the decoded result.
Body words and font doubles are verified before removal. Nested font patches are
applied inside the root item list before sketch-text records are appended. None
of these changes adds or removes MFC class or object slots. Every final root is
parsed to its end at its declared version. Earlier fixture groups stay unchanged.

Ninety-six inline models each contain two complete facets. Forty-eight independent
facet records check every typed member and exact writes at the declared version,
including edge arrays without edge types. Existing node-word preservation,
logical node resets, text tails, component, polygon, material and bucket-list
gates remain independent. Complete cached models, empty/null models and all five
old-light classes remain covered.

Composite sketch text, fonts, nodes, polygons, edges and facet-count edits match
independently built roots and authentically revert. Each boundary rejects a
context requiring its missing fields without changing the source. Truncation
tests reject partial roots.

Eight owned containers cover baseline and edited part and assembly streams at
1286 and 1332. The later pair uses populated 2D models with ghost transforms and
the full secondary splitter, font and sketch-text suffixes. Replacement, reopen
and revert preserve unrelated streams. Process verifiers compare complete roots
and whole containers in two fresh processes.

This checkpoint admits archive contexts from 1286. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
