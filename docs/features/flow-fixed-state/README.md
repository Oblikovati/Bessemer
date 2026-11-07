# Flow record 139 fixed state

Issue: #241. Reference: `../inventor-interface/solidworks/flowrecord139.py` and
the record-139/301/417 native findings in specification section 14.74.

`FlowCarrier::record_139_fixed_state` reads complete raw versions zero through
four. Version zero ends after its version word and has no stored count.
Versions one through four add a nonnegative signed row count. Every row stores
two raw wide patterns. Versions two through four also store 320 wide settings
patterns and 100 word settings patterns. Versions three and four add a final
signed member. Row widths are exactly 16, 2,976 and 2,980 bytes respectively.

The view preserves count absence separately from a stored zero. It stores the
normalized layout once and computes borrowed row descriptors on demand, without
allocating one object per row. Both actual rows and complete payload bytes have
explicit limits; zero rows admits version-zero and stored-empty layouts.
Impossible counts fail against available bytes before multiplication or access.
Successful reads consume the complete payload, including exact EOF.

`FlowFixedStateWideMember` names the leading member at native offset `+0x600`
and the separately stored discarded pattern. Optional arrays reuse the existing
borrowed word and wide views. `replace_setting` reuses `FlowFixedSettingsKind`
and `FlowFixedSlot` only to select storage width, count and exact bit encoding.
This does not import global settings-slot meanings from records 7/102. Version
one's missing serialized arrays remain absent; project settings are not read
as an implicit fallback.

`replace_wide_member`, `replace_setting` and `replace_final_member` patch only
existing values on isolated copies. They completely reopen each result with
the original bounds and return typed source/result checkpoints and physical
carrier history. Counts and versions remain structural. Missing rows or fields,
invalid indexes and mismatched slot kinds return typed errors. No-ops retain
the original checkpoint, and fresh reversals restore complete carrier bytes,
including duplicate and unrelated records.

Coverage reports complete raw framing, count presence, row width, actual field
totals and every stored scalar, including header words. Serialized approximation
error is zero. Native settings application, runtime destination capacity,
engineering roles and SolidWorks application acceptance remain outside this
check. No settings normalization or floating-point conversion is applied.

The owned FST2 baseline contains 562 cases. It covers all supported versions,
absent and zero counts, multiple rows, raw single bits and complements, signed
zero, infinities, NaN payloads and subnormals. The audited native FLD sample is
version four with one row and exactly 2,988 bytes. Both leading patterns are
`0x7FEFFFFFFFFFFFFF` and the final member is zero. Source and payload hashes
are recorded in `../flow-state-tables/baseline-verification.json`.
The dedicated native manifest's full projections and physical boundaries are
also checked in `native-manifest-verification.json`.

Run `cargo test --test flow_state_table_fixtures`. Tests compare every frozen
row interval, field and raw value, edit every existing stored value, and verify
no-ops, copied/derived history, source isolation and fresh reversals. Independent
four-, eight- and twenty-four-byte oracles pin count absence, stored zero counts,
member ordering and optional field absence. Prefix, extension, malformed-count,
type and exact-bound checks reject incomplete layouts.

Eleven public types, including reused array and slot types, are checked for
`Send + Sync`. Eight workers each perform 32 edits and compare complete carrier
bytes with serial results. Two fresh processes replay all source-checked reads,
both leading members, present finals and first/middle/last settings slots.
ThreadSanitizer and the full-platform gates for this implementation remain
pending; the earlier open PR excludes it.
