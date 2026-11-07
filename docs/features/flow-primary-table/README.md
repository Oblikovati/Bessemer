# Flow record 439 primary-class tables

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord439.py`.

Bessemer reads and edits record 439's complete documented primary-class grammar
for versions 0 through 3 with `FlowCarrier::record_439_primary_table`.
The reference binds that profile to native vtable `0x1809ec4b0`; the serialized
bytes do not establish native object allocation or engineering meanings.

Each item stores a kind byte and element-kind byte, followed by exactly two
groups. Version three adds a stored mode byte before each group's two signed
members and presence byte. Any nonzero presence byte introduces a body with two
independently counted raw 32-bit vectors, three signed members and fixed arrays
of six and three raw wide patterns. Version zero also stores nine legacy wide
patterns. Versions two and three store a flag whose native projection is true
only for the exact byte value one. Other nonzero flag bytes remain false in that
projection while retaining their stored bits.

Kinds two and three add a typed-element list after each group, including groups
whose optional body is absent. A zero list accepts every unused element-kind
byte. A populated list requires one of four layouts: a raw 32-bit word, two raw
32-bit words, a word plus a raw 64-bit pattern, or word/wide/word. Their serialized
strides are 4, 8, 12 and 16 bytes. Each element contains independently counted
ragged rows and independently counted blocks of sixteen wide patterns. Empty
rows, empty elements, absent bodies and absent typed lists remain distinct.

The F439 packet freezes 5,685 complete payloads. It covers all versions, every
kind, unused element-kind, mode, presence and stored-flag byte, all four typed
layouts, consecutive and reversed items, independent vector counts, empty rows,
1,024-entry rows, 128 empty rows/elements and 128 wide blocks. Raw numeric cases
cover each single bit and complement, signed zero, infinities, NaNs and
subnormals. Expected data retains every field and nested item/group/body/element/
row/block interval, and Rust reconstructs the complete payload from those fields.

The audited native CPT sample has an eight-byte version-two payload with zero
items. Its whole-file and payload hashes are recorded in
`baseline-verification.json`. This proves the empty native framing only;
populated primary-class layouts use generated fixtures. No populated installed
fixture, native runtime behavior, engineering role or application acceptance is
claimed.

Python independently rebuilds every record, regenerates the packet in a temporary
root, checks all proper prefixes of compact layouts, mutates every reached
nested count to negative/impossible values and rejects trailing bytes. It also
pins the distinction between an unused factory tag and a populated unsupported
tag. Four Rust tests check complete reconstruction, native empty framing, all
byte-valued roles, every typed stride and raw numeric pattern coverage.

The typed view requires an explicit primary-class selection and raw transport.
Independent aggregate limits bound payload bytes, items, elements, rows, wide
blocks, scopes, field descriptors and stored values. Zero collection limits
admit empty collections. Immutable scopes retain every physical ancestor and
complete interval; field views retain exact encoding, count and stride without
allocating one object per numeric value. Counts and version are read-only.
Existing-field edits use revision-bound IDs, reparse the complete payload and
require unchanged scope intervals and field layouts. No-ops keep the original
checkpoint. Each completed edit returns both typed views and complete carrier
history through `FlowPrimaryTableEdit::carrier_edit`. Native allocation,
engineering meanings and application acceptance remain excluded.

The separate F4EP packet supplies 1,402 expected fixed-shape edits for that API.
The oracle changes a field in the reference reader's parsed tree, serializes the
whole changed tree and reparses it before comparing the changed byte interval.
This keeps the expected output independent of Bessemer's patching code.
It covers versions zero through three, both groups, all four row layouts,
optional-body values, wide blocks, unused factory tags and flag transitions to
and from the exact-one native projection. Structural counts and version bytes
are excluded. Python checks field locations against all 5,685 original payloads
and regenerates F4EP in a temporary directory. Rust verifies each complete
expected patch and its byte-for-byte reversal. These are oracle checks; they do
not establish native application acceptance.

The typed API tests compare all 5,685 complete payloads with their frozen Source
values, field roles, ancestry and scope intervals. Every byte belongs to exactly
one stored field element; independently counted descriptors retain present
empty columns. All 1,402 expected edits run through the production editor, check
complete carrier bytes and history, preserve no-op checkpoints and reverse from
freshly opened results. Malformed checks span every supported version and
factory, all reached count fields, proper prefixes, trailing bytes and unknown
populated factories. Resource limits are checked at their exact bounds and one
below. Fourteen public types have compile-time Send/Sync assertions; eight
workers compare serial reads and isolated edits. Two fresh processes also
compare complete canonical field transcripts, each checked against the Source
tree. The subprocess worker is skipped as a standalone test and executed twice
by its regular parent test. Full-gate and native-application acceptance are
reported separately from these focused checks.
