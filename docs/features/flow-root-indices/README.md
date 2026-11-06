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
uncompressed fixture bytes.

`FlowDerivedLeafCount::project_root_indices` consumes the already validated
fresh full-grid context. It bounds roots, leaves and total path bytes before
publishing immutable flat metadata and one shared path-byte array. Traversal
uses an explicit stack; it does not recursively parse, walk or own tree nodes.
Root indices use the native nonnegative signed domain, and storage byte offsets
use `u64`. Leaf paths retain their true depth without modulo-256 wrapping.

The result borrows roots and local leaves without allocating per view. It retains
the original grid/tree checkpoints through its source context. Exact coverage
reports every root, leaf and child-path byte, plus the maximum true path depth.
Required limits reject incomplete projections rather than returning partial paths.
Empty grids return completed empty projections. The projection does not modify
input bytes or assign global block ordinals. Native context construction,
parallel scan offsets and native application acceptance remain unproved.

Validation compares every projected field against all 233 frozen cases, checks
exact and one-below limits independently, and retains depth-256 paths through
caller input mutation and eight-worker projections. Two fresh processes match
the complete independent Source packet byte for byte. These checks establish
the declared root/local-path projection; they do not establish block-vector
ordering or application acceptance.
