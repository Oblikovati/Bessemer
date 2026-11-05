# Flow grouped-word record 404 (#241)

This directory pins specification 14.31 before the typed record-404 decoder is
implemented. Bessemer currently preserves these complete physical carriers and
payloads through `FlowCarrier`. The tests here reconstruct every reference payload
from independently frozen normalized fields; they do not claim that Bessemer
already exposes grouped-word decoding or editing.

The corpus contains 26 cases: both installed native carriers and 24 owned cases
covering payload versions 3–6, no groups, empty groups and multiple members under
both explicit raw and zlib contexts. The native files contain one identical
version-5 payload variant, not two independent schemas: each has 4,056 groups,
one member per group and a 68,960-byte decoded payload. Their different surrounding
carrier bytes remain intact. The fixture packet is 1,769,971 uncompressed bytes.

The version-5 and version-6 literal oracles use two distinct members to distinguish
separate arrays from interleaved thirteen-byte records. The native single-member
groups alone cannot distinguish those layouts. Version 4 retains its auxiliary
i32 even in empty groups. Versions 3/4 have no wire flags; versions 5/6 retain a
present empty flag array when the group has no members. Raw u32/u64 words include
high bits and non-finite binary64 patterns without assigning a floating-point type.

The upcoming reader must require an explicit raw/zlib context, validate payload
version independently of the carrier word, bound groups and total members before
allocation, and require exact decoded EOF. It must preserve auxiliary/flag presence,
member order and every bit. The first typed edit operation must retain complete
source/result carrier history and prove that only the selected field changed.
Unsupported delegated versions must remain explicit errors. No native flag defaults,
units, value domains or array-433 count derivations are inferred from this record.

The transport extraction was a separate structural change. The original record-433
operation labels, entities, classes, messages, decoded values, carrier outputs and
histories remain unchanged. Twenty-five existing/frozen tests passed before and
after extraction; the complete 11,904-cell edit/reversal census passed again.
Two post-extraction processes each reproduced the pre-extraction 1,405,107-byte
FAR1 output stream exactly. The shared helper now provides revision-bound payload
selection and strict zlib transport without adding a public record-404 surface.

Run the current baseline checks with:

```text
cargo test --test flow_group_fixtures --locked
python docs/features/flow-groups/test_groups.py
```

Python regeneration uses a temporary injected root and never rewrites reviewed
fixtures during tests. `native-byte-verification.json` authenticates seven retained
native ranges against the original viewer DLL and verifies the shared compression
proof hash. Byte authenticity does not establish native owner context or application
acceptance. Typed record-404 implementation, malformed-input gates, field edits,
concurrency/process verification and the newer full repository gate remain required.

The humanizer skill is unavailable; its prose pass remains deferred to review.
