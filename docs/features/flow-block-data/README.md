# Flow legacy block data

Issue: #241. References: specification sections 14.39 and 14.42,
`../inventor-interface/solidworks/flowcontextcounts.py` and `flowblockdata.py`.

Record 406 versions 1–17 contain a count prefix, a conditional bitmap and
leaf-block entries. The leaf count is explicit external context. Global count
totals do not establish that context or record 433's runtime count. Mathematical
totals and the native signed 32-bit wrapped view are retained separately.

The frozen baseline contains 273 views: one complete native CPT payload and
eight profiles for each legacy version in raw and zlib transport. Profiles
cover empty blocks, bitmap counts on either side of a byte boundary, wrapped
global totals, negative local counts, negative second-array counts where
representable, populated entries and mixed empty/populated blocks. The fixture
retains every raw flag, descriptor, discarded prefix/suffix and array word.
Version-one member 20 aliases the legacy index without consuming another word.

Freezing checks the native file and payload hashes, every corpus block and
entry interval, scalar projection and integer-word hash. Rust independently
rebuilds all decoded bytes, validates version gates and signed views, and checks
the native identifier set and array totals. Regeneration runs in a temporary
directory. The fixed version-14 hex oracle checks entry order independently.

`FlowCarrier::record_406_blocks` reads a revision-bound occurrence under explicit
raw/zlib transport and a validated `FlowLeafCount`. Independent limits bound
decoded bytes, blocks, total entries and total integer words. The published
body owns complete decoded bytes and flat block/entry metadata. Array views
borrow those bytes; there is no allocation per array word or per entry array.
Every stored scalar, raw flag, descriptor and word is selectable by a typed field.

`replace_field` changes an existing field on a copy and reparses the complete
carrier and selected body before returning source/result checkpoints and all
physical history relations. Count edits must retain the existing bitmap,
entry and array sizes. Negative count changes may retain a zero-word branch.
Exact no-ops preserve stored bytes and revision identity. Fresh raw reversals
restore wire bytes; fresh zlib reversals restore decoded bytes without imposing
a canonical compressed representation.

`derive_fresh_flow_leaf_count` requires an explicit `AssumeFresh` argument,
complete active legacy trees, matching default grid bounds and every global
root present. It retains the original grid/tree checkpoints and returns a
validated count. It does not execute native context construction or establish
block-to-tree ordering.

Native runtime flags and allocation are not reproduced. Block wire ordinals
are not assigned to tree nodes, even when a fresh full-grid context establishes
the leaf count. Versions 18 and later use a separate delegated body layout and
are outside this legacy reader. No native application acceptance is claimed.
