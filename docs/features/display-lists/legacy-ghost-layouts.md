# Ghost items, hidden counts, part timestamps and older facets

Reference baseline: specification edition 0.253, dated 2026-10-25. Reader hashes
are recorded in the fixture manifests.

The 406 complete part and assembly roots cover versions 1215, 1216, 1235, 1236,
1253, 1254, 1270, 1271, 1275, 1276 and 1286, with 133273 independently expected
traced and physical fields. They cover five storage boundaries:

- Ghost display-item lists, including their counts, are absent through 1215 (`0x4BF`).
- Bucket `show_hidden_count` words begin at 1236 (`0x4D4`).
- Facet legacy triangle arrays and planar words begin after 1253 (`0x4E5`).
- Part `ddata_time_stamp` words begin at 1271 (`0x4F7`).
- The discarded facet dynamic array begins after 1275 (`0x4FB`).

Above the ghost-list boundary, populated buckets contain a complete point display
item; empty buckets retain zero item counts. Edited fixtures change a stored item
word. Both 2D and 3D nested buckets are covered; display-item points retain three
coordinates independently of the enclosing polygon coordinate scope.

The generator identifies the exact ghost-item list returned by each complete
bucket parse before replacing or removing it. Hidden-count and timestamp words
are verified before removal. The reference represents the all-one timestamp as
a signed sentinel; the physical comparison retains its original 32 bits. Fixture
scalar extraction only includes a timestamp when the source version stores it.
These primitive changes preserve all MFC class and object slots. Every final
root is parsed to its end at its declared version. Earlier groups stay unchanged.

Eighty-eight inline models each contain two complete facets. Forty-four separate
facet records are independently built at the declared versions. Tests compare
all typed members and exact bytes, including absent legacy triangles, planar
words and discarded arrays. Existing UI, sketch-text, font, node, polygon,
component and material boundaries remain independent. Complete cached models,
empty/null models and all five old-light classes remain covered.

Composite ghost-item, sketch-text, font, node, polygon, edge and facet-count
edits match independently built roots and authentically revert. Each boundary
rejects a context requiring its missing fields without changing source inputs.
Truncation tests reject partial roots. Physical object-array headers anchor the
part timestamp and body-field assertions; Bug #40 adds a regression proving the
body assertion visits a real populated body on each side of its boundary.

Eight owned containers cover baseline and edited part and assembly streams at
1215 and 1276. The later pair uses populated 2D models with ghost display items,
hidden counts, timestamps and complete legacy facet arrays. Replacement, reopen
and revert preserve unrelated streams. Process verifiers compare complete roots
and whole containers in two fresh processes.

This checkpoint admits archive contexts from 1215. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
