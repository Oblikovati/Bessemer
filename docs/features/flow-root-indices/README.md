# Flow root storage indices

Issue: #241. Reference: specification section 14.43 and
`../inventor-interface/solidworks/flowrootindices.py`.

The documented projection uses an explicit fresh full-grid assumption. A root's
storage index is `(third*n1 + second)*n0 + first`; its byte offset is that index
times 56. Leaves are enumerated locally within each root by depth-first child
order zero through seven. A leaf retains its full child path and serialized
node interval. Native runtime construction, parallel scan offsets and global
block-vector ordinals are separate work.

The frozen baseline contains 233 cases: the native CPT grid/tree pair, all
27 triples drawn from dimensions zero, one and two across four legacy versions
and two tree transports, and depth-255/256 unit trees across those same versions
and transports. The grid boundary words retain nonfinite values and signed
zero without introducing a geometry interpretation. Legacy versions zero and
one retain explicit external bounds. Native source and payload hashes are
checked before freezing.

The Python checks pin the 56-byte stride, first-coordinate-fast storage indices,
local leaf ordinals, full depth-256 paths and the native totals of 792 roots and
3,984 leaves. Regeneration uses a temporary directory and compares complete
uncompressed fixture bytes. This baseline precedes the typed projection API;
it does not assign global block ordinals or claim native application acceptance.
