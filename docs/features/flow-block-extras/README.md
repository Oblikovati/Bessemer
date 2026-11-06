# Flow legacy block extras

Issue: #241. Reference: specification section 14.47 and
`../inventor-interface/solidworks/flowblockextras.py`.

Record 408 versions 0–8 require an explicit leaf count and enabled runtime
context. Each leaf stores a signed token and, from version three, a raw flag.
Token `-1` or a nonzero flag selects a body. The serialized stream does not
establish its correspondence with tree traversal order.

Bodies retain four lists, mixed signed/raw-wide tables, byte arrays, nested
arrays and versioned tails. Mixed tables use parallel arrays before version
eight and interleaved twelve-byte elements in version eight. First-list
references widen from signed 32-bit words to raw 64-bit words in version five.
Additional-array presence exists only from version seven and only after a
nonempty byte array. Any nonzero additional or extra flag selects its fields;
outer, nested and final presence require exactly `1`.

Nested counts widen from unsigned bytes to signed nonnegative 32-bit counts in
version five. Nested indices are stored from version six. Earlier versions
derive the full ordinal without truncating it to a byte. The last raw byte
appears from version four. Every flag and otherwise unresolved value remains
part of the frozen wire representation.

The baseline contains 289 views: one complete native CPT payload and 288 owned
views spanning every legacy version in raw/zlib transport. Profiles include
zero leaves, absent/minimal/rich bodies, noncanonical presence values, empty and
maximum byte arrays and lists, 255/300-entry nested rows, both mixed layouts, signed extrema
and nonfinite wide patterns. Rust reconstructs every wire byte and verifies all
recorded intervals. Python pins fixed hex, version gates, context admission and
complete regeneration. Native totals are 3,984 leaves, 1,936 bodies, list counts
of 4,636/19,880/3,904 and 25,632 fourth-list elements.

This is the independent fixture baseline for a typed reader/editor. Tree-order
assignment, native object-reference resolution, runtime reconstruction and
native application acceptance remain unproved.
