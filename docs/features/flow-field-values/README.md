# Flow version-prefixed field values and runtime widths

Issue: #241. Reference: specification section 14.51 and
`../inventor-interface/solidworks/flowfieldvalues.py`.

Record 108 stores an i32 version before its raw or zlib body. Body intervals begin
after that version. Versions five and six use an externally counted width table
and object widths in explicit global-object order. Zero-width objects retain
their positions while consuming no words. Optional expected-table entries can be
wildcards; unchecked stored table words remain signed and may be negative.

Versions seven and above store a region count and each region width. External
context supplies row counts, expected widths and active-storage flags. Inactive
regions and zero widths consume no value patterns. Large inactive dimensions
must remain representable without allocating their mathematical products. A
positive width with zero rows also consumes no patterns, while retaining its
distinct width and storage context. Compatibility restriction is explicit and
rejects versions above seven before body decompression.

An empty legacy body is valid, including under a zero decoded-body bound. A
compressed body still needs one complete checksum-valid zlib member. The four
raw version bytes contribute to logical decoded payload size but do not belong
to body offsets or its decoded-byte bound.

The FFV1 baseline contains 203 views. One complete installed FLD payload is read
under three supplied expected-width contexts; the other 200 cases cover both
body transports, compatibility branches, empty and zero-width layouts, wildcard
checks, negative stored table values, inactive maximum dimensions and mixed
region storage. It retains every context value, optional-check presence, raw
pattern, object/region interval and byte-accounting field. The packet is
7,595,839 bytes with SHA-256
`b347f3157f185b895fc8d8a60b4d1a8367da2b9d4350afb87ddfd858018599cc`.

The native version-five payload has 301,710 stored bytes: four version bytes and
301,706 compressed body bytes. Its supplied layout has 3,368 objects of width 14
and 2,584 zero-width objects. The body contains table `[14]` and 47,152 raw wide
patterns, consuming 377,220 decoded body bytes and 377,224 logical payload bytes.
These are explicit conditional contexts, not observed runtime execution or an
automatic binding to the selected region's numeric storage.

The separate FWP1 baseline contains 315 isolated width requests: 289 accepted
widths and 26 rejected strict-profile inputs. It covers every signed type byte,
cached count bounds, missing and ignored context, out-of-range region indices,
selected versus unselected invalid values, signed integers and binary64 edges.
Rust checks the frozen outcomes with integer bit decomposition rather than a
floating-point cast. This pins truncation before count validation: values between
negative one and zero produce zero, while negative integers, nonfinite values
and results above the native signed maximum are rejected. The packet is 10,938
bytes with SHA-256
`a6ddced69e5421fe601977b58652bf683b385ed2c2271f309ca7fba8f1e58d14`.

`FlowCarrier::record_108_values` reads these layouts with `FlowFieldValueOptions`.
`FlowFieldCount` admits native count/width values once. Immutable
`FlowLegacyFieldLayout` and `FlowRegionFieldLayout` retain explicit object order,
wildcard checks and region row/storage context. Independent positive limits bound
metadata positions, width-table words and actual wide patterns. The body-byte
limit separately admits zero. Inactive dimensions are retained without allocating
their declared products. Every successful read reaches exact decoded EOF.

Borrowed objects and regions expose body intervals and raw patterns. Coverage
keeps raw version bytes, stored body bytes and logical decoded size separate,
alongside optional-check presence and actual physical value totals. Diagnostics
retain the runtime assumptions and engineering exclusions.

The editors replace existing object/region patterns and signed legacy table
words on isolated carriers. Explicit expected table entries reject mismatched
edits; unchecked and wildcard slots may retain negative signed values. Each edit
preserves the raw version, encodes only the body, reopens the complete typed result
under its original context and limits, compares all item spans and verifies the
full patched body. Results carry both typed checkpoints and complete physical
history. No-ops preserve the original stored bytes and revision. Raw reversals
restore the complete carrier; zlib reversals restore the decoded body without
assuming canonical compression.

`project_flow_region_width` implements the isolated documented width calculation
with a validated signed type and explicit cached/selected numeric inputs. It
retains the immutable input checkpoint, selected origin, exact count error and
native-state exclusions. Unused inputs remain uninterpreted. Binary64 conversion
checks finiteness and truncates before validating the nonnegative signed count;
the independent bit-decomposition oracle checks this path without sharing that
floating-point implementation.

The implementation decision is exact serialized field interoperability under the
supplied layout, with four raw version bytes and an explicitly selected body codec.
The focused gate passed 18 Rust tests and nine Python checks. It covers all 203
field views, all 315 width requests, every small owned existing field, first/last
larger-array samples, 48 deterministic native edit samples, complete physical
history, fresh reversals, exact budgets, malformed boundaries and measured codec
growth. Eight workers compare all width successes/failures and complete field
edits under both layout families and transports. Each public type is checked for
`Send`/`Sync`. The separately invoked native census targets 47,153 existing fields;
its acceptance is recorded independently of this focused gate.

Runtime layout inference, selected numeric-data binding, native storage/exception
state, engineering columns and native application acceptance remain open. The
width calculation does not substitute global record-102 numeric values.
