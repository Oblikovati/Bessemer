# Complete archives with legacy facet-curvature arrays

Reference baseline: specification edition 0.258, dated 2026-10-25. Reader hashes
are recorded in the fixture manifest. This edition's model-plane constraint
additions are outside this display-list checkpoint.

The 290 complete part and assembly roots cover versions 773, 774, 816, 817 and
903, with 86015 independently expected traced and physical fields. They retain
cached, empty, external-face and inline-facet models, nested buckets, text and
all five old-light classes. Earlier fixture groups remain unchanged.

Through 816 (`0x330`), a facet container stores a complete dynamic array in the
curvature position. Later versions store a raw flag and optional f32 colour
array. The existing standalone facet codec already distinguishes these two
representations. This checkpoint proves their use inside complete display-list
archives, including whole-file replacement and exact reverts.

The legacy array's semantic element domain remains unknown. Fixtures explicitly
store width 24, initial capacity 17, growth step 23, and one or two elements of
known bytes. Edited fixtures increase capacity, growth and count by one and
replace the final payload byte with 255. Tests construct the expected typed
array independently, checking every metadata word and payload byte. Later
fixtures check the raw -7 presence word, NaN payload bits and negative zero in
the calculated colour representation.

Forty inline roots each contain two complete facets. Twenty independently built
facet records check full typed contents and exact writes. The root generator
replaces only complete facet spans, preserving all class and object slots.
Context changes reject legacy arrays in calculated contexts and calculated
fields in legacy contexts. Empty and cached models remain compatible because
they do not contain an inline facet body.

Composite edits change legacy array metadata and payload alongside triangle
counts, text, nodes and lights, matching independently built complete roots.
Reverts restore source bytes exactly. Additional checks reject truncated roots
and reconstruct shared immutable archives concurrently. Eight owned containers
at 773 and 817 cover part and assembly edits, reopen and revert while preserving
unrelated streams. Root and container verifiers compare forced writes in two
fresh processes.

The Windows verification run also reconstructed all 1086 native display streams
and 20 native drawing containers exactly in two processes. Both accepted corpus
manifests remained unchanged. Verification reports beside this note record input,
output, context and runtime source hashes, including the reused facet codecs.

This checkpoint admits archive contexts from 773. Earlier root-level item and
line-font layouts, unresolved representations and the broader documented model
schemas remain open. Byte fidelity does not establish geometric validity or
native application acceptance.
