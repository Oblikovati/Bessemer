# Flow legacy pointer and word tables

Issue: #241. Reference: specification section 14.45 and
`../inventor-interface/solidworks/flowlegacytables.py`.

Record 401 versions 0–3 contain exactly two groups. A populated group stores
member triplets, three raw binary64 arrays and signed pointer-index pairs.
Version one retains discarded count words even for empty groups, extra triplets
and a pre-pair word. An optional global pointer count checks each pair's range;
absence preserves unbounded signed values without claiming valid pointers.
Repeated pairs remain repeated. Range checking does not populate native pointers.

Record 405 version zero requires an explicit nonnegative G.Fluid/G.Solid pair
whose mathematical sum fits the signed native count domain. Version one stores
its own count and rejects unused legacy context. Words remain raw unsigned
32-bit patterns; their engineering roles are unresolved.

The baseline has 173 views: the two native CPT payloads, with both context modes
for native record 401, plus owned legacy versions in raw and zlib transport.
Profiles include empty/one-sided/populated groups, present empty arrays, repeated
pairs, unbounded negative pairs, signed scalar extrema and nonfinite binary64
patterns. Word profiles preserve high bits and empty legacy/stored counts.
Source file and native payload hashes are checked before freezing.

Rust independently reconstructs every decoded byte, group/array interval,
discarded field, pair and word. Native totals are 15,880 pairs, 7,930 wide words
and 5,952 record-405 words. Python pins fixed hex bytes and distinguishes context
presence, pointer range checking and native population. Regeneration uses a
temporary directory and compares complete uncompressed fixture bytes. This
baseline pins the wire contract independently of the typed reader/editor.

`FlowCarrier::record_401_pointer_table` accepts explicit raw/zlib transport,
optional validated pointer-count context, and independent decoded-byte, pair and
wide-word limits. Both groups retain their physical headers and optional bodies.
Populated bodies borrow member/discard triplets, three arrays and ordered pairs.
The array and pair views do not allocate per value or collapse duplicates.

`FlowCarrier::record_405_words` requires a validated legacy count pair for
version zero and rejects unused context for version one. Stored and external
counts remain distinguishable. Raw unsigned words retain all bits.

`replace_field` on pointer tables and `replace_word` on word tables edit existing
fixed-width fields on isolated copies. They reparse the complete carrier and
selected table before publication, retaining source/result checkpoints and every
physical history relation. Supplied pair-index bounds also apply to edits.
Structural counts, lengths and versions are not changed by these field editors.
Exact no-ops retain stored bytes and revision identity. Fresh raw reversals
restore complete wire bytes; fresh zlib reversals restore decoded bytes, without
requiring a canonical compressed representation.

Coverage reports all serialized bytes and values separately from the external
range/count context. Native pointer population, engineering word roles and
native application acceptance remain unproved.
