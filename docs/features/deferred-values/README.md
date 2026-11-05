# Conditional deferred values

`deferred_values` reads and constructs the complete installed deferred-object
MsgPack writer grammar in clause 9.2, C-3590. The behavior version is
`C3590-deferred-values-v1`. The caller selects control0's big-endian profile or
control1's installed-x64 little-endian profile. The module does not infer current
control state from the file image, host platform, payload or database stream name.
A profile declares consistent control state for the entire blob; native global
mutation or callback behavior is not inferred from that declaration.

All 17 concrete native writers are represented. Their persisted outputs map to
11 value forms: null, Boolean, negative integer, unsigned integer, raw binary32
and binary64 bits, raw string and binary bytes, arrays, ordered object pairs and
extensions. Positive signed-writer values share their persisted representation
with unsigned values. The original integer constructor width and nonzero runtime
Boolean byte cannot be recovered from those identical encodings.

Integer/count/extension forms must match the selected installed minimal writer.
Other MessagePack encodings, including valid standard encodings that these
writers do not emit, return explicit unsupported-profile errors. Accepted blobs
reconstruct byte for byte; no unproved wider prefix is silently normalized.
Strings retain NUL and non-UTF-8 bytes. Floating-point bits, including signaling
NaNs, remain unchanged. Object pairs retain their order and duplicate keys.

```rust,no_run
use bessemer::deferred_values::{
    DeferredValue, DeferredValueDocument, DeferredValueLimits, DeferredValueProfile,
};
let limits = DeferredValueLimits {
    maximum_bytes: 1024, maximum_values: 10, maximum_depth: 8,
};
let value = DeferredValue::ObjectPairs(vec![
    (DeferredValue::StringBytes(b"x".to_vec()), DeferredValue::UnsignedInteger(1)),
    (DeferredValue::StringBytes(b"x".to_vec()), DeferredValue::UnsignedInteger(2)),
]);
let document = DeferredValueDocument::from_value(
    &value, DeferredValueProfile::BigEndianControl0, limits,
)?;
assert_eq!(document.value(), &value);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Byte, visited-value and child-depth budgets are explicit. Root depth is zero;
the host recursion ceiling is 64. Declared counts must also fit the remaining
minimum-width bytes before allocating container storage. Object keys count as
values. Empty arrays, objects, strings, binary bodies and extensions remain
complete values with one visited root.

Independent Python fixtures cover 548 encodings, both control profiles, all
17 writer classes and every count-width boundary through 65536. Tests compare
decoded values and newly constructed bytes to those goldens, exercise malformed
and nonminimal inputs, and check source ownership and eight shared writers. Two
additional nested goldens cover repeated recursive keys and mixed value forms.
A constructed known-format value is written through an existing opaque storage
slot and a complete containing document in both profiles. This demonstrates byte
storage and reopening; it does not identify the original slot's producer or
assign MsgPack semantics to its original value.

Run `cargo test --test deferred_values`, `cargo test --lib deferred_values`, and
`python -B docs/features/deferred-values/fixture_bytes.py`.
Source/capture hashes and the mandatory public-dump/private-dispatcher label
qualification are retained here. Native writer failures, tree/provider behavior,
archive-mode admission, database engine/commit outcomes and application/geometry
acceptance remain separate. The humanizer skill is unavailable; prose review
remains with the maintainer.
