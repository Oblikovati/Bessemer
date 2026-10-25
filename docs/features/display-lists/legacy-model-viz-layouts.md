# Early model visuals, pane quality and light layouts

Reference baseline: specification edition 0.255, dated 2026-10-25, rechecked
against edition 0.257. Editions 0.256 and 0.257 add model-feature layouts outside
this display-list checkpoint. Reader hashes are recorded in the fixture manifests.

The 962 complete part and assembly roots cover seventeen versions from 989 to
1180, with 284873 independently expected traced and physical fields. They
exercise both sides of the model-visual pointer, facet edge-array, pane quality
and old-light layout boundaries. Earlier fixture groups remain unchanged.

Part models omit their model-visual pointer before 998 (`0x3E6`). The generator
observes that pointer at the end of the complete part payload and proves that
removing it changes no MFC class or object slots: its value is null or an alias.
Facet edge arrays begin at 990 (`0x3DE`). The fixtures include 136 inline roots,
each with two facets, and 68 independently built complete facet records.

Panes omit the hidden-line display quality word through 1116 (`0x45C`). Their
perspective word and direct view transform remain independent. The generator
locates the quality word from the second pane transform and checks the reference
reader's normalization of stored `0xFFFF` to logical zero. Rust preserves the
physical word where it exists.

All five old-light classes have dedicated baseline and edited roots at every
checkpoint, totaling 340 roots. Shaded profiles contain an ambient definition,
a second definition and shared references; unshaded profiles contain none.
Through 1063 (`0x427`), the base stores six doubles and three vectors before
color and suppression. Later bases use the shorter layout. Cartesian-display
selection begins at 1106 (`0x452`). User names are absent from 1031 through 1107
inclusive; identity names remain stored throughout these checkpoints.

Directional and point geometry begins at 1064. Spot geometry first stores
position, target and cone angle; exponent and falloff fields begin at 1087
(`0x43F`). Independent builders preserve property bytes and graph slots while
constructing each base and derived payload. Complete-root parsing must reach
the end at its declared version.

Tests compare all independent fields and forced writes, edit each light class
and the populated geometry profiles, revert exactly, and check source isolation.
Context transitions reject missing or extra fields while accepting unchanged
light subtypes. Truncation tests reject partial roots. Eight owned containers
at 989 and 998 cover inline facets and model pointers; replacement, reopen and
revert preserve unrelated streams. Process verifiers compare roots and whole
containers in two fresh processes.

The Windows verification run reconstructed all 1086 native display-list streams
and 20 native drawing containers exactly in two processes. Their accepted corpus
manifests remained unchanged. The root and document verification reports beside
this note record input, output, context and source hashes for replay.

This checkpoint admits archive contexts from 989. Earlier documented layouts
and unresolved representations remain tracked by #28. Byte fidelity does not
establish geometric validity or native application acceptance.
