# Flow record 138 context state

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord138.py`.

`FlowCarrier::record_138_context_state` reads complete raw versions two through
five. The stored object count is a nonnegative signed word. It does not
determine index width. A positive object count in versions two, three or four
requires `FlowContextStateOptions::index_count`, using the existing validated
`FlowSimpleLegacyCount` type. The caller supplies this native member `+0x380`
assumption explicitly. Empty records and version five ignore supplied context,
including an otherwise unused maximum signed count.

The view preserves all raw index patterns through the existing borrowed
`FlowBlockWordArray` view. It does not validate index values against object
count or infer engineering meaning from their bits. Versions two and three
also store a signed tail. Every successful read consumes the complete payload;
there is no successful prefix result with an opaque extension.

Payload bytes and actual index words have independent limits. A zero word limit
allows inactive records and applied zero-count contexts. Even the maximum
stored object count can coexist with zero index words. No storage is allocated
per object or per index; the immutable view borrows the validated raw array.

`replace_index` and `replace_tail` patch only existing values on an isolated
copy. They completely reopen the result with the same context and bounds and
return typed checkpoints with full carrier history. Object count and version
remain structural. Missing tails and out-of-range index edits fail. No-ops
retain the original checkpoint, while fresh reversals restore complete carrier
bytes, including duplicate and unrelated records.

Coverage separates stored object count, actual index words and the applied
count assumption. `Some(0)` remains distinct from ignored context. Diagnostics
report the count assumption only when applied, and retain native runtime,
engineering and SolidWorks application exclusions. Exact serialized framing
does not establish native context provenance or application acceptance.

The owned FST2 packet contains 86 complete cases. It covers every supported
version, inactive and positive object counts, independent index counts through
1,024, and maximum signed object counts. The audited native CFLD payload has
one object and 3,369 index words under an extent-derived context assumption.
That count has not been verified against a native owner. Source and payload
hashes are recorded in `../flow-state-tables/baseline-verification.json`.

Run `cargo test --test flow_state_table_fixtures`. The tests compare every
source-frozen field and raw value, check independent signed/unsigned oracles,
exercise exact bounds and missing/ignored context, and verify existing-value
edits, history, no-ops and fresh reversals. Native runtime population remains
outside this API.

Seven public types, including the reused count and array view, are checked for
`Send + Sync`. Eight workers each perform 32 edits and compare full carrier
bytes with serial results. Two fresh processes replay every source-checked
state and existing-value edit byte for byte. ThreadSanitizer and the full
platform matrix for this implementation remain pending; the earlier open
PR does not include it.
