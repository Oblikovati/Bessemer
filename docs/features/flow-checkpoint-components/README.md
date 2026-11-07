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

No public component API is shipped by this baseline. See [design.md](design.md)
for the layout and implementation boundary. The humanizer skill is unavailable;
the maintainer prose pass remains for review.
