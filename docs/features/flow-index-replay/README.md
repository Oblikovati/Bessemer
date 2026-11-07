# Logical index replay baseline

Issue: #241. This baseline pins `../inventor-interface/solidworks/flowindex.py`
before adding a Bessemer logical-index API. It adds fixtures and tests only.

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
