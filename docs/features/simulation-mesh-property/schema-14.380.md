### 14.380 Nested Mesh direct-record extension grammar

*Status: populated nested body bounded with static/synthetic evidence; native Mesh successes preserved · Evidence: C-760*

Direct Mesh record reader 181119560 calls **1811197D0** for each entry of either enabled nested extension collection. Those records are read directly from IStream and contain no tagged-object prefix or CFB stream header. C-760 binds function-byte/decompilation hashes and all three nested-body threshold values to the installed module.

The fixed prefix occupies **268 bytes**: one four-byte word at +00, followed by thirty-three raw eight-byte fields. Their native destinations are +08; +18/+20/+28; nine fields +30 through +70; +78/+80/+138; +148/+150/+158; nine fields +160 through +1A0; and +1A8/+1B0/+1B8/+1C0/+1C8. Destination gaps are memory layout only and insert no padding into the stream. These direct eight-byte reads are preserved as raw values; physical types/coordinate meanings are not inferred solely from widths.

Continue in wire order:

| Inclusive minimum version | Nested continuation |
|---|---|
| 850029 | nine raw eight-byte fields at +C0..+100; word +108; signed count and that many raw eight-byte entries |
| 850029 | nine raw eight-byte fields at +1F0..+230; word +238; signed count and that many raw eight-byte entries |
| 850035 | eleven raw eight-byte fields: +88/+90/+98/+A0/+1D0/+A8/+1D8/+B0/+1E0/+B8/+1E8 |
| 890502 | raw eight-byte fields +10/+140 |

Native assignments below 850035 or 890502 initialize omitted memory values and consume no bytes. The current independent decoder rejects negative/over-limit counts, preserves both arrays independently, and validates available bytes before each read. Zero-array complete widths are 268, 428, 516 and 532 bytes across the successive profiles.

`cosworks_mesh_nested_record.py` supplies strict standalone parsing and explicit offset/prefix mode for a parent stream. The parent `cosworks_mesh_layout.py` now consumes populated +2C/+38 extension records through this verified grammar instead of stopping. Tests cover each threshold, both populated arrays, strict standalone remainder rejection, offset/prefix endpoints, invalid counts, and a populated nested record followed by the remaining Mesh Property suffix. No length scanning or synthetic padding is used to locate the parent continuation.

C-760 freshly replays **all 357 native Mesh Property streams / 94876 bytes** and proves every C-759 successful candidate remains identical by path, stream, payload hash, version, endpoint and reconstruction hash. Eighteen tests pass across nested Mesh and dependencies. Native extension collections remain empty in this corpus, so populated-body validation is static/synthetic only; this extension does not increase the retained native complete count.

The populated >=891103 newest Mesh map remains a separate explicit stop, and field meanings, historical carrier mismatches and complete installed-format coverage remain open. No target executes; global semantic coverage credit remains zero.

Evidence: C-760.
