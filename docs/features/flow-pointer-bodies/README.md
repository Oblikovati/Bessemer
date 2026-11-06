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

`FlowCarrier::record_421_bodies` reads these layouts with `FlowPointerBodyOptions`.
`FlowPointerBodyCounts` admits each collection count independently to the native
signed domain and retains their mathematical sum without wrapping. The options
retain an immutable optional code slice, including an explicitly present unused
slice for an empty legacy third collection. Tagged layouts reject that context.
Independent positive limits bound pools, entries, vector descriptors, wide words
and opaque bytes, in addition to the complete decoded-byte bound. Counts are
checked against remaining bytes before creating descriptors or multiplying widths.

Borrowed pools and entries expose raw patterns, vector boundaries, tag bytes,
stored counts, runtime codes and complete decoded intervals. Unknown tags have
no fabricated count, payload or decode flag. Coverage states the exact raw/zlib
mode and actual stored/decoded totals; diagnostics retain the documented exclusions.

The editors replace existing pool/body wide words, vector bytes, opaque tagged
bytes and shape-preserving tags. Counts and vector lengths remain structural.
Every edit builds an isolated carrier, reopens the entire typed record under the
original context, compares every pool/entry/vector descriptor and verifies the
complete patched decoded payload. No-ops retain the original stored bytes and
revision. Results include original/result checkpoints and all physical carrier
history. A raw reverse restores the complete original carrier; a zlib reverse
restores the decoded payload without requiring a canonical compressed encoding.

The implementation decision is exact serialized interoperability for documented
versions 0–5, with no inferred runtime-code substitution or packed-payload decoding.
Later delegated versions return a typed unsupported error. Native runtime state,
orientation postprocessing, tree ordering, engineering roles and native application
acceptance remain unproved.

The focused gate passed 17 Rust tests and eight Python checks. Both fresh-process
read projections matched the complete independent FPB1 packet byte for byte:
2,358,701 bytes across 2,109 views. Owned small fields received exhaustive
existing-value edit checks; larger arrays used first/last samples. Sixteen native
edit samples passed. The separately invoked complete native census covers
107,712 existing values: 30,432 pool words and 77,280 vector bytes. Its acceptance
is recorded separately from the focused gate. All six versions also passed
measured compressed-growth rejection and eight-worker isolation checks.
