# Flow record 129 tagged state

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord129.py`.

`FlowCarrier::record_129_tagged_state` reads complete raw version-nine payloads.
A nonnegative signed row count precedes all tag bytes. Row bodies follow the
entire tag block in physical order. Zero tags have no body and no stored width.
Every nonzero tag introduces a nonnegative signed width and that many raw wide
patterns. A positive row count adds a signed tail, even when every tag is zero.
Empty records omit the tail.

The view retains every tag byte. `FlowTaggedStateRow::tag_offset` locates its
separate tag, while `payload_interval` describes only the body. Zero-tag body
intervals have equal endpoints. `declared_width` and `values` distinguish absent
bodies from nonzero-tag bodies with present zero widths. Wide patterns use the
existing borrowed `FlowTableWideArray` without floating-point conversion.

Raw bytes, row descriptors and total wide patterns have independent bounds.
Counts are checked against available tag/value bytes before descriptor
allocation or multiplication. Every successful read reaches exact EOF. The
immutable result owns its payload and row intervals; it allocates no descriptor
per wide value and takes no lock for reads.

`replace_tag` preserves the zero/nonzero body-presence class, including for
present-empty arrays. `replace_wide` and `replace_tail` edit only physically
stored values. Versions, row count and widths remain structural. Each edit
patches an isolated copy, completely reopens the result with the original
bounds and returns typed checkpoints with full carrier history. Missing values,
invalid indexes and tag presence transitions fail. No-ops retain the original
checkpoint; fresh reversals restore complete carrier bytes and unrelated records.

Coverage reports exact raw framing, row and tag counts, present bodies, actual
wide values, tail presence and every stored scalar. Serialized approximation
error is zero. Diagnostics keep native context validation, runtime state,
engineering column meanings and SolidWorks application acceptance outside this
raw schema check.

The owned FST2 baseline contains 773 cases. Generated cases cover all 256 tag
bytes, absent and present-empty bodies, independent widths, mixed rows,
1,024-row records and raw numeric patterns. The audited native FLD payload
contains 5,532 rows, 60,660 wide patterns, tags 0/1/3 and a zero signed tail in
507,048 bytes. Its full payload, intervals and every value are checked on read.
Source and native hashes are recorded in
`../flow-state-tables/baseline-verification.json`.

Run `cargo test --test flow_state_table_fixtures`. Generated rows and their
existing wide elements are all edited. Native edits select the first row for
each distinct tag/width shape plus first/middle/last physical rows, and
first/middle/last wide elements. The five selected native rows cover all three
native shapes. Every selected edit checks independent frozen offsets, no-ops,
complete history, unchanged row layout, source isolation and fresh reversals.
Independent eight-, thirteen- and thirty-one-byte oracles pin split tags,
absent bodies, present-empty arrays, raw NaN bits and conditional tails.

Seven public types, including the reused wide view, are checked for `Send + Sync`.
Eight workers each perform 32 tag/value/tail edits and compare full carrier bytes
with serial results. Two fresh processes compare every source-checked read and
the same selected edits. ThreadSanitizer and full-platform validation for this
implementation remain pending; the earlier open PR excludes it.
