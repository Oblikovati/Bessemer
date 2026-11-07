# Logical checkpoint components

Issue: #241. This baseline covers the five component readers documented by
`../inventor-interface/solidworks/flowcheckpointcomponents.py`: graph-one objects,
graph-two objects, auxiliary grids, trailing numeric containers, and version-zero
chunk buffers. These are logical buffers in the documented version-six context.
They do not establish a physical record-121 decoder.

The 2,899 cases authenticate all five reference examples and add owned requests
for count boundaries, raw numeric patterns, cyclic axes, presence bytes, slot
tags, and opaque chunk contents. Six `FCC1` packets retain complete reference
projections. The largest decompressed packet is 11,899,811 bytes; every packet
stays below the shared 16,000,000-byte test-fixture bound.

The examples are synthetic. The reference evidence cites native reader and
writer bindings, but this baseline does not re-execute those functions or verify
an installed version-six component fixture. Installed record 121's version-three
body remains a separate unresolved boundary.

Run the baseline with:

```text
cargo test --locked --test flow_checkpoint_component_fixtures
python -m unittest discover -s docs/features/flow-checkpoint-components -p test_components.py
```

`freeze_components.py` regenerates the owned packets from the authenticated
reference and requests. Regeneration is checked byte for byte. Rust tests walk
the frozen topology and verify the complete raw partition, scalar patterns,
arrays, row and slot presence, and encoded chunk extents. Python tests supply
independent size formulas and reject malformed counts, every small prefix, and
extensions. They also pin the source reader's permissive byte rules.

See [design.md](design.md) for the layout and implementation boundary. The
humanizer skill is unavailable; the maintainer prose pass remains for review.

## Public reader and editor

`FlowCheckpointComponent::from_bytes` requires `FlowCheckpointContext::LogicalVersionSix`,
an explicit `FlowCheckpointKind`, and `FlowCheckpointOptions`. It reads all five
documented layouts through exact EOF. Input-byte and aggregate bounds cover row
positions, indices, slots, numeric containers, wide patterns, chunks, and encoded
bytes before allocation. Zero admits an absent category. The numeric-array limits
exclude fixed scalar headers and signed members; declared raw size never drives
an allocation or substitutes for the encoded-byte bound.

For a graph-one buffer containing exactly seven values:

```rust
let limits = FlowCheckpointOptions {
    maximum_component_bytes: 56,
    maximum_row_positions: 0,
    maximum_indices: 0,
    maximum_slots: 0,
    maximum_containers: 0,
    maximum_wide_patterns: 7,
    maximum_chunks: 0,
    maximum_encoded_bytes: 0,
};
let view = FlowCheckpointComponent::from_bytes(
    &raw,
    FlowCheckpointContext::LogicalVersionSix,
    FlowCheckpointKind::Graph1,
    limits,
)?;
let field = view.graph1_patterns()?.field(0)?;
let edit = view.replace_field(&field, FlowCheckpointValue::Pattern64(42))?;
```

Borrowed views expose complete hierarchy, field intervals, and raw values.
`FlowCheckpointValue` distinguishes signed members, 32-bit patterns, 64-bit
patterns, and bytes. Field identities belong to one immutable revision; parsing
equal bytes again creates a distinct revision. Diagnostics stay bounded even
though identities retain their complete source.

`replace_field` copies and reparses the complete component under the original
kind, context, and bounds. A no-op retains the original revision. Changes preserve
all structural extents: presence bytes keep their zero/nonzero class, and slot
tags keep their bodyless/container class. Existing numeric values, signed members,
declared raw size, and encoded bytes retain their serialized types. Counts,
dimensions, versions, and encoded lengths are structural and have no field-edit
handle.

Each edit retains both exact checkpoints, corresponding source/result fields,
and a complete ordered partition of copied and derived byte intervals. Results
and field handles remain immutable and support sharing across threads. Changing
or dropping caller buffers and other edited revisions cannot mutate a published
view.

Coverage reports exact logical component framing and stored totals. Parent-record
decoding, engineering interpretation, chunk decoding, raw-size agreement, native
runtime reproduction, and application acceptance remain excluded after reads
and edits. Reader-permitted noncanonical slot tags remain explicitly qualified.

The combined local `cargo xtask pr` gate passed at `f2902881`: 5,122 tests
passed, none failed, and 267 were ignored. This covers the logical index and
checkpoint APIs together with the earlier IMS/control source. Later legacy
field APIs and hosted matrix acceptance are excluded. See the
[exact-source result](../flow-checkpoint-components/full-pr-gate-verification.json).
