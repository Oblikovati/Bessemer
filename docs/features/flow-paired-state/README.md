# Flow record 417 paired state

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord417.py`.

`FlowCarrier::record_417_paired_state` reads the complete raw version-one
record. It contains exactly two groups in physical order. Each group stores a
nonnegative signed count followed by that many raw index words. A positive
index count adds a separately counted wide array. A zero index count omits the
wide count and array entirely.

The two array counts remain independent. Positive index counts can introduce
present-empty wide arrays, and wide counts need not be divisible by index
counts. `FlowPairedStateGroup::native_quotient` preserves the documented integer
division, including zero quotients. It returns `None` when the wide array is
absent. No divisibility rule, engineering partition or native allocation
behavior is inferred from this arithmetic.

The view reuses `FlowBlockWordArray` and `FlowTableWideArray` for checked raw
values and exact decoded intervals. It retains signed-index bit patterns,
signed zero, infinities and NaN payloads without normalization. Complete group
intervals include count prefixes; array intervals contain values only.

Payload bytes, total index words and total wide patterns have independent
bounds. Array totals include both groups. Zero bounds admit absent arrays and
present-empty wide arrays. Counts are checked against remaining bytes before
any multiplication or value access. The immutable result stores two group
descriptors and borrows arrays from its owned raw payload, without allocating
objects per element.

`replace_index` and `replace_wide` patch existing values on a copy, completely
reopen the result with the same limits and return both typed checkpoints with
physical carrier history. Versions and counts remain structural. Absent arrays,
invalid groups and out-of-range elements return typed errors. No-ops preserve
the original checkpoint, and fresh reversals restore complete carrier bytes,
including duplicate and unrelated records.

Coverage reports exact raw framing, both groups, actual array totals, present
wide-array count and zero serialized approximation error. Diagnostics keep
engineering interpretation, native runtime population and SolidWorks
application acceptance outside the serialized check.

The owned FST2 baseline contains 237 complete cases. It covers independent
counts, absent and present-empty arrays, nondivisible counts, 1,024-element
arrays and raw numeric single bits, complements and special patterns. The
audited native FLD sample contains two empty groups in twelve bytes. Populated
layouts use generated source-derived fixtures; no populated native acceptance
is claimed. Source and payload hashes are recorded in
`../flow-state-tables/baseline-verification.json`.

Run `cargo test --test flow_state_table_fixtures`. Tests compare every frozen
scope and raw value, edit every existing index and wide value, and check no-ops,
history and fresh reversals. Independent twelve-, twenty- and eighty-byte
oracles pin absence, present-empty arrays, exact intervals and nondivisible
quotients. Malformed-count and exact aggregate-bound checks cover both groups.
Eight workers each perform 32 edits and compare full carrier bytes with serial
results. Two fresh processes compare all source-checked reads and first, middle
and last edits in each existing array. Eight public types, including reused
array views, are checked for `Send + Sync`. ThreadSanitizer and full-platform
validation for this implementation remain pending; the earlier open PR excludes it.
