### 14.120 Simulation signed node body-group tag and shared references

*Status: Native signed tag/boundary consumer and complete installed run inventory verified; tag is not an exclusive body-ownership filter · Evidence: C-496*

The 52-byte GEN node record contains a **signed 16-bit body-group tag at offset 22 / 0x16**. In **cosworks.dll!0x180A58740..0x180A58A39**, the auxiliary producer follows the current element's body-selection ID and scans node tags to determine a group boundary. The tag load is explicitly **`MOVSX` from a word**, followed by comparison with the current body ID. This establishes signed interpretation for this consumer; it is not an unsigned alias of the separate element-body field at offset 28.

The producer accesses nodes through `CNodeContainer::operator[]`. **cwmeshcontainer.dll!0x180002B40..0x180002B5C** selects a 16,384-record allocation chunk and returns a record with **52-byte** stride, binding the consumed word to the stored GEN node layout. The tag is distinct from the activity byte at offset 0 and the coordinate components at offsets 32/36/40.

For each encountered element-body transition, the inspected producer records a prospective first node as its prior zero-based cursor plus one, then advances from that cursor until a signed node tag differs from the current body ID. A mismatch records the boundary index in the intermediate entry; the final entry can close at the supplied node count. The native tag scan itself does not filter on node activity. Its body ordering, cursor and enclosing mesh context remain relevant; this is not a general set-membership query over all matching tags.

`solidworks/simulationmesh.py` now exposes both `body_group_tag` (i16) and `body_group_tag_bits` (u16 raw representation) in `node`, preserving the complete record. Its `node_group_runs` helper returns diagnostic contiguous runs with one-based first/last node ordinals. It scans every supplied record, including inactive ones, preserves repeated separated runs and handles allocation-chunk boundaries as continuous stored record order. This diagnostic is not a complete emulator of the producer's element-driven traversal and does not relabel CAD bodies.

The installed node blocks have these complete runs:

| Fixture | Tag | First..last node | Count |
|---|---|---|---|
| Each composite fixture | 1 | 1..5333 | 5333 |
| Each composite fixture | 2 | 5334..10553 | 5220 |
| Benchmark | 1 | 1..24140 | 24140 |

Across all **45,246 nodes**, these fields occupy **90,492 bytes**. Only tags 1 and 2 occur in the installed corpus; zero and negative tag patterns remain synthetic tests. A negative i16 pattern is sign-extended by the native consumer, so, for example, raw `0xFFFF` does not compare equal to a zero-extended element body ID 65535. The full tag domain and CAD identity mapping remain open.

**A node's tag does not restrict which body's elements may reference it.** `GetNodesByBodyId` in 14.115 checks element body IDs, node-list references and node activity; it does not require the referenced node's tag to equal the queried body ID. In each composite fixture, the supplied mode-3 body-2 query selects **5,296 nodes**, including **76 tagged 1** in addition to the 5,220 tagged 2. Body-1 selects the 5,333 tag-1 nodes. The benchmark selection matches its 24,140 tag-1 nodes. Across the two composite fixtures there are **152 cross-tag selected nodes**. These relationships refine the meaning of the field without inventing exclusive ownership or discarding shared references.

`simulation_node_group_evidence.py` retains **two native producer/accessor ranges**, the signed load, module/source hashes, linked body-query evidence, complete run/histogram data and cross-tag selected node IDs in `simulation-node-group-native-evidence.json`. Six focused tests cover signed/raw patterns, activity-independent diagnostic runs, separated repeated tags, a run crossing the 16,384-node chunk boundary, body selection across a tag boundary, source preservation and whole-record guards. No installed target was executed. The preceding trailing-word search retains candidates without assigning meanings to unrelated stack offsets; those words and positive-length auxiliary-vector semantics remain unresolved. This increment does not regrade global/all-installed-project percentages.

All **1,634 regression tests pass** (207.823 seconds). The native/corpus verifier passes; specification lint reports zero findings (682 headings, 96 anchors).

Evidence: C-496.
