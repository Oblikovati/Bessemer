# Flow grouped-word record 404 (#241)

`FlowCarrier::record_404_groups` implements the serialized profile in specification
14.31. Callers explicitly select raw or zlib compression. The payload version is
independent of the carrier's two-byte word. Versions 3–6 are admitted; older and
newer delegated layouts fail explicitly. Group counts, total member counts and
complete decoded bytes are bounded before publication, with exact payload EOF.

```rust,ignore
let options = FlowGroupOptions {
    compression: FlowGroupCompression::Zlib,
    maximum_decoded_bytes: 64 * 1024 * 1024,
    maximum_groups: 100_000,
    maximum_members: 1_000_000,
};
let record = carrier.record_404_groups(&record_id, options)?;
let edit = record.replace_field(
    0, FlowGroupFieldEdit::WideWord { member: 0, value: replacement_bits },
)?;
let output = edit.result().source().as_bytes();
```

The immutable record retains its source carrier and exact occurrence identity.
Group views expose decoded spans, complete ordered u32/u64 values, optional i32
auxiliaries and optional flag iterators. Raw u64 values retain every bit, including
non-finite binary64 patterns. No engineering type, unit or value domain is inferred.
Version 4 has an auxiliary word even in empty groups. Versions 3/4 omit wire flags;
versions 5/6 retain present-empty flag arrays for empty groups. Generated native
flags remain outside the wire view. Version 6 interleaves thirteen-byte members;
versions 3–5 use separate arrays, with no invented runtime padding.

`replace_field` edits one existing word, wide word, stored flag or auxiliary on
an isolated copy. It reparses the whole output carrier and grouped payload, then
requires that only the selected field's decoded bytes changed. Complete carrier
history, selected-field history and the original checkpoint are retained. No-ops
keep original bytes and record identities. Raw opposite edits recover original
wire bytes; zlib opposite edits may use a different encoding, which the edit
diagnostic states explicitly. Exact original bytes remain available from the
source checkpoint. Missing fields, invalid indexes and failed output limits
publish no partial result.

Coverage distinguishes `ExactRawGroupedFrames` and `ExactZlibGroupedFrames`,
reporting stored/decoded bytes, every group and member, and zero achieved byte
error. Diagnostics retain unknown member semantics, caller-supplied compression
context and unverified native application acceptance. The reader never derives
record 433's runtime count from record 404's group count. Count/version changes,
new-record construction, owner-state reconstruction and wider Flow schemas remain
required work.

The frozen corpus contains 26 cases: both installed native carriers and 24 owned
cases covering versions 3–6, zero groups, empty groups and multiple members under
both compression contexts. The two native carriers share one version-5 payload
variant: 4,056 groups, one member per group and 68,960 decoded bytes. Their different
outer carrier bytes remain intact. The packet is 1,769,971 uncompressed bytes.
Literal multi-member oracles distinguish separate and interleaved layouts, which
the native single-member groups alone cannot do.

The transport extraction landed separately after pinning original record-433
error contracts. Twenty-five frozen tests and the 11,904-cell census passed again.
Two post-extraction processes each reproduced the pre-extraction 1,405,107-byte
FAR1 stream exactly. Python corpus regeneration uses a temporary injected root.
The retained native-byte report authenticates seven Source ranges and the shared
compression-proof hash; it adds no native runtime or application acceptance credit.

```text
cargo test --test flow_groups --test flow_group_fixtures --locked
cargo test --test flow_groups --locked every_native_group_word -- --ignored --nocapture
python docs/features/flow-groups/test_groups.py
```

The humanizer skill is unavailable; its prose pass remains deferred to review.
