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

This is the fixture baseline for the typed readers/editors. Engineering roles,
native runtime reconstruction, tree ordering and native application acceptance
remain unproved.
