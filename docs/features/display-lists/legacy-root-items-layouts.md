# Root-owned display items before per-view lists

Reference baseline: specification edition 0.259, dated 2026-10-25, rechecked
against edition 0.261. The intervening model-feature additions are outside this
checkpoint; all recorded display-list reader hashes remained unchanged.

The 282 complete part and assembly roots cover versions 767, 768, 772 and 773,
with 76807 independently expected traced and physical fields. Before 773
(`0x305`), the root stores one display-item list between its bounding box and
environment object. Each view omits its own list. From 773, the root list is
absent and every view stores a list after its bucket pointer.

The decoder reuses the complete typed display-item codec in both positions.
Compact and extended count words remain physical fields. The independent item
trace includes root-owned items alongside later per-view items; its control-word
filter remains separate from exact-byte reconstruction.

The builder observes the complete later root and identifies its view list by
object identity. Before moving that list, it checks that reading it allocated no
MFC class or object slots. Tests compare every relocated typed item, including
its count header, against the later fixture. All remaining typed fields and
graph tags must stay identical. Existing fixture groups remain unchanged.

Forty-eight additional roots exercise ownership independently of view count:
populated global lists with no views, two views sharing one loaded bucket,
extended empty counts, and extended one-point lists. The shared bucket's second
tag must reference the first object's identity. A dedicated point pair differs
only in its point-style word, from -1 to 23. Editing that one typed field must
match the independently built complete stream and revert exactly.

Composite edits cover root-owned text, geometry and lights alongside complete
inline facet arrays. They preserve graph tags and source storage. Thirty-two
inline roots retain two facets each, checked against the independent legacy
curvature records from the preceding checkpoint. Context transitions reject
missing root lists or extra per-view lists. Truncation tests reject partial
roots; concurrent constructors must reconstruct the same bytes.

Eight owned containers at 767 and 773 exercise part and assembly replacement,
reopen and revert while preserving unrelated streams. Root and container
verifiers compare forced writes in two fresh processes.

The Windows verification run also reconstructed all 1086 native display streams
and 20 native drawing containers exactly in two processes. Both accepted corpus
manifests remained unchanged. Reports beside this note record input, output,
context and runtime source hashes, including the item comparison adapter.

This checkpoint admits archive contexts from 767. Earlier line-font omission,
other legacy branches, unresolved representations and the broader documented
model schemas remain open. Byte fidelity does not establish geometric validity
or native application acceptance.
