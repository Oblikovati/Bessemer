# Flow parallel, per-leaf and raw tables

Issue: #241. Reference: specification section 14.48 and
`../inventor-interface/solidworks/flowsimpletables.py`.

Record 407 version zero stores both group counts before either group. Each group
then stores a signed presence word; any nonzero value selects four raw binary64
arrays. Declared counts remain visible even when arrays are absent. A present
group with count zero retains four empty arrays. Positive versions store two
zero counts without presence words; populated restored arrays remain unsupported.

Record 437 version zero stores two raw binary64 patterns for each caller-supplied
leaf. Nonzero versions are delegated. The serialized order does not establish
correspondence with tree traversal.

Record 411 versions zero through two store no rows. Their optional external
count remains distinct from an unspecified count. Version three requires the
external member-`0x2d0` count; version four stores its count. Those two versions
store five raw 32-bit words per row. Versions five and later accept a stored
zero count, while populated restored arrays remain unsupported.

Record 413 version zero requires an external member-`0x294` count. Positive
versions store the count. Versions zero and one store that many raw bytes;
versions two and later accept only an inactive zero count.

The independent baseline contains 118 views: four complete native CPT payloads
and owned raw/zlib views. Profiles preserve absent/present-empty arrays, signed
presence extrema, huge inactive external counts without allocating rows,
nonfinite raw patterns, all byte values and empty later layouts. Rust reconstructs
every decoded byte and checks every recorded interval. Python pins fixed hex,
context admission, count origins and complete regeneration. Native totals are
parallel counts 1,944/1,960, 3,984 leaf pairs, 4,056 five-word rows and zero bytes.
Native record-402 members match the two observed table counts; this observation
does not assign field roles or infer caller context.

`FlowCarrier::record_407_arrays`, `record_437_leaf_pairs`, `record_411_rows` and
`record_413_bytes` expose separate typed views and options. Record 437 requires
`FlowLeafCount`; legacy member counts use validated `FlowSimpleLegacyCount`.
Missing required context and unused context in stored-count versions are errors.
Declared count, count origin and actual wire storage remain independently
observable. The readers do not allocate rows for inactive external counts.

Resource limits bound complete decoded bytes and actual serialized wide words,
rows or raw bytes. Huge declared counts with absent arrays or inactive rows do
not consume those value budgets. Zlib transport requires a complete RFC 1950
stream with checksum and exact decoded EOF. Views borrow arrays and rows without
allocating per value, preserve every raw pattern, and report serialized coverage
separately from native and engineering exclusions.

Existing-field writers replace parallel/pair wide words, five-word raw values,
raw bytes and stored parallel presence words. Counts and versions remain
read-only. Presence edits must retain array structure, including the distinction
between absent and present-empty arrays. Writers patch isolated copies, reparse
the complete carrier and selected payload under original options, and publish
`FlowSimpleTableEdit<T>` with typed source/result checkpoints and full physical
history. Exact no-ops retain stored bytes and revision identity. Fresh raw
reversals restore complete carrier bytes; compressed reversals restore decoded
bytes without assuming canonical zlib output.

Engineering roles, native runtime reconstruction, tree ordering and native
application acceptance remain unproved.
