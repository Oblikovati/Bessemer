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

`FlowCarrier::record_408_extras` requires a validated `FlowLeafCount` and an
explicit `FlowBlockExtrasEnabled::AssumeEnabled` selection. Raw/zlib transport is
supplied separately. Independent limits bound decoded bytes, blocks, scope
descriptors, field descriptors and total stored scalar/array elements.

The immutable view enumerates typed scopes and typed fields in deterministic
parse order. Scope keys retain all physical parent ordinals. Nested indices
report whether they were stored or derived. Fields expose their exact encoding,
count, first-byte offset and stride, plus checked typed values and raw element
bytes. Version-eight mixed arrays have twelve-byte strides; they do not pretend
to be contiguous arrays. Empty fields retain their physical presence and offset.

Each edit uses a revision-bound field ID and an existing element index. IDs also
retain the leaf-count interpretation. Counts and version are read-only. The
editor patches an isolated decoded copy, reopens the complete carrier, reparses
the entire payload under the original options and requires identical scope and
field layouts before publishing. Source/result checkpoints and full physical
history accompany the result. No-ops preserve original stored bytes and revision
identity. Fresh raw reversals restore whole carrier bytes; compressed reversals
restore decoded bytes without assuming canonical zlib output.

The supplementary field oracle reconstructs the Source parser's complete
structured values independently, checks every declared interval and proves that
field elements cover every decoded byte exactly once. Its separate `FEO1`
packet leaves the original `FBE1` baseline unchanged. Read tests compare every
scope, index source, role, encoding, stride and raw element against that oracle.

Tree-order assignment, native object-reference resolution, runtime reconstruction
and native application acceptance remain unproved.
