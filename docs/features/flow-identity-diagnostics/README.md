# Bounded Flow identity diagnostics

Bug: #248.

Flow record identities retain an immutable carrier revision. Derived debug output
previously expanded that revision's complete byte buffer and span list. Identity
errors in record-301 messages and other field/profile views could therefore grow
with whole carrier sizes and include unrelated payload bytes.

`FlowRecordId` now formats only its numeric identifier, physical ordinal, carrier
version, wire length and record count. Compact and pretty debug output remain
bounded as opaque payload sizes grow. No payload data or span arrays are included.
Existing error call sites inherit this behavior without changing identity checks.

Revision equality still uses the same parsed checkpoint identity. Cloning a
checkpoint preserves its identities; parsing identical bytes again creates foreign
identities. Compact debug metadata is diagnostic context, not a serialized identity
or a claim that two checkpoints are interchangeable.

`cargo test --test flow_identity_diagnostics` checks payloads through 512 KiB,
compact/pretty bounds, clone identity, foreign reparsing and source isolation.
No native payloads are needed for these checks.

The Flow module is absent from `main`; this fix applies to the unreleased feature
line before publication. It does not add Flow functionality to the patch line.
