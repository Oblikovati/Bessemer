# Logical index replay baseline

Issue: #241. This baseline pins `../inventor-interface/solidworks/flowindex.py`
before adding the Bessemer logical-index API. The reader now uses this baseline
for preferred terminal adoption and forced physical scanning.

The 1,070 cases produce 2,140 requested outcomes: 1,474 physical scans,
649 terminal-index adoptions, and 17 rejected preferred-index reads. All
1,070 forced scans complete. The three installed native carriers contain
2,936,303 bytes and 66 active records; all three use physical scanning.
Populated terminal adoption is covered by reference-derived synthetic cases,
not installed terminal payloads or executed native code.

Four independently bounded `FLR1` packets hold native carriers, version 7
scan cases, version 8 scan cases, and terminal cases. Each decompressed packet
stays below the existing 16,000,000-byte fixture limit. The verification
manifest records reference hashes, packet hashes, sizes, and evidence limits.

Run the baseline with:

```text
cargo test --locked --test flow_index_replay_fixtures
python -m unittest discover -s docs/features/flow-index-replay -p test_replay.py
```

`freeze_replay.py` regenerates the packets from the authenticated reference
corpus and owned requests. Regeneration is tested byte for byte. Rust tests
compare physical spans and independently folded maps with the frozen results;
terminal payload fields are also checked through the existing strict control
reader. Python tests add closed-form scan expectations and recognition errors.

See [design.md](design.md) for the replay contract and exclusions. The
humanizer skill is unavailable in this checkout; the prose pass remains for
maintainer review.

## Public reader

`FlowCarrier::logical_index` takes `FlowIndexPolicy` and `FlowIndexOptions`.
`PreferTerminalIndex` scans on a recognition miss and rejects a recognized bad
control. `ForcePhysicalScan` leaves control payloads opaque. Every call starts
with empty maps. Distinct active and saved key counts have independent bounds;
duplicate replacements remain admissible at the configured limit. Control
decoded-byte and stored-entry bounds apply only to controls that are decoded.
Zero map bounds admit empty maps; an empty adopted terminal still requires its
self entry to fit the active-map bound.

For an already validated physical carrier:

```rust
let options = FlowIndexOptions {
    maximum_active_entries: 100_000,
    maximum_saved_entries: 100_000,
    controls: FlowControlOptions {
        maximum_decoded_bytes: 2_000_000,
        maximum_entries: 100_000,
    },
};
let index = carrier.logical_index(FlowIndexPolicy::PreferTerminalIndex, options)?;
let raw_target = index.active_entries().get(&14);
```

`FlowLogicalIndex` retains the immutable physical source, requested policy,
actual replay method, sorted active and saved maps, and both selected control
views when present. Those views retain wire-order duplicates and exact
revision-bound occurrences. Replay does not modify the physical carrier or
change the behavior of `latest` or `solver_latest`.

Coverage reports physical framing separately from decoded control bytes and
map counts. It explicitly excludes ordinary payload decoding, ordinary indexed
target validation, and complete document coverage. Diagnostics also retain the
fresh-map assumption and native execution limits. Failed reads return typed
errors and publish no partial maps.
