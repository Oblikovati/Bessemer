# Flow legacy pointer pools and bodies

Issue: #241. Reference: specification section 14.49 and
`../inventor-interface/solidworks/flowpointerbodies.py`.

Record 421 versions 0–5 require an explicit leaf count and three collection
counts. Each leaf pool starts with a byte count and stores three raw binary64
patterns per item. The three body collections follow in physical order. Their
counts are external and are not serialized in this payload.

Before version three, the first two collections contain byte-counted single
vectors. The third requires current runtime entry-code context. The documented
code predicate selects either byte-counted wide triplets or a byte-counted list
of byte-counted vectors. Identical empty wire bytes can select different modes
under different contexts. Stored record-402 code bytes must not silently replace
the caller's current runtime context.

Versions three through five use a raw tag byte. Tags one and two store a signed
nonnegative count. Version three tag two stores `3 * count + 1` wide patterns,
including one word when count is zero. Other recognized payloads store exactly
`count` opaque bytes. Unknown tags consume only their tag and have no stored
count or packed-decode flag. Raw payload bytes remain opaque even if they resemble
a compressed stream; packed payload decoding is not reproduced.

The independent baseline has 2,109 views: one complete native CPT payload, owned
profiles for all six versions, every current-code byte and every tagged byte in
raw/zlib transport. It covers empty and unused context presence, maximum byte
counts, nested empty vectors, nonfinite patterns and tagged zero counts. Rust
reconstructs every decoded byte and verifies all pool, group and entry intervals.
Python pins fixed hex, independent code masks, context ambiguity and complete
regeneration. Native totals are 3,984 pools, 10,144 pool triplets and collection
counts 9,338/6,542/5,532. The native fixture explicitly supplies codes and counts
from the documented prior fields; this does not reproduce mutable runtime state.

This is the fixture baseline for a typed reader/editor. Native runtime state,
orientation postprocessing, tree ordering, engineering roles and native
application acceptance remain unproved.
