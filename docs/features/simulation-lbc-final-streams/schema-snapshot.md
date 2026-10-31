### 14.336 LBC name-order lists and legacy midsurface-map fallback

*Status: all 336 retained name-order lists and five empty legacy maps complete; populated legacy maps and persistent-ID maps remain partial · Evidence: C-712*

Static analysis of the final two LBC manager reader calls binds **0x18108BFB0–0x18108C0D8** to **LbcListOrder** at literal **0x18201A240**, and **0x18108C0E0–0x18108C8D5** to **PersistMidsurfMap** at **0x18201A260**, with **MidsurfMap** fallback at **0x18201A288**. C-712 verifies the installed module hash, literal bytes, function-span hashes and retained decompilations. These are distinct named carriers; basename similarity does not make their bodies interchangeable.

**LbcListOrder.** The stream contains a signed four-byte count followed by exactly that many persisted strings using 14.262. There is no leading selection string as in the study-order format. The native reader collects the strings in wire order and, after reading, passes the collection to helper **0x18108D3E0**. That helper's matching/reordering behavior is not recovered here. The manager's retained writer emits its runtime count followed by each entry's virtual **+0xC0** string result; the connection to every load subtype's name/identity remains open.

`read_lbc_order(data, max_items=10000, max_string_bytes=0x100000)` bounds counts/text, retains raw count and every original text header/payload, and requires exact EOF. It preserves duplicates, null strings and empty strings independently; it does not deduplicate or sort. The native positive-loop behavior for a negative count is not emulated: the independent decoder rejects it.

Fresh hash-checked extraction of **Cosworks/StudyManager/(StudyNumber-N|SimStudy-N)/LBCManager/LbcListOrder** completes all **336 streams / 32396 bytes**. They contain **1137 strings**, with **no zero-item lists** and a maximum of **48 items**. Every input reconstructs byte-for-byte. This includes the two SimStudy ancestry occurrences, rather than generalizing from StudyNumber alone.

**MidsurfMap fallback.** The second reader enters this map-loading path only when its owning-study selector at **+0xC0** equals **1 or 3**. It tries `PersistMidsurfMap` first and opens `MidsurfMap` only when that first stream is unavailable. The legacy fallback reads **three groups**, each a signed four-byte count followed by that many records of **four signed four-byte words**. All three groups have the same wire shape; their runtime destinations and physical meanings are not thereby identical. There is no version-dependent primitive width in this legacy branch.

`read_legacy_midsurf_map(data, max_records=10000)` returns the three ordered groups, preserving all four values per record, bounds each count against the remaining input, rejects negative/over-limit counts and requires exact EOF. The decoder does not implement the newer persistent-ID format, runtime map insertion or the native handling of repeated keys. It must not be applied to `PersistMidsurfMap`.

All **five legacy MidsurfMap streams / 60 bytes** in the scoped corpus complete. Each is exactly **three zero count words**, so no native populated legacy map record is claimed. Populated groups with signed values are covered by static reader evidence and a synthetic byte-for-byte reconstruction test only. The inventory contains **zero PersistMidsurfMap occurrences** at these ancestries; absence of a fixture does not complete that format's schema.

Four focused tests pass for duplicate/null/empty names, list bounds and truncations, populated legacy groups, exact reconstruction and map bounds/truncations. The existing twelve primitive decoder tests also pass. C-712 retains every raw native payload, hash, list text span or map group, installed binding and source digest in `cosworks-lbc-final-native-evidence.json`. No target code or native writer executes. Native framing for these **341 occurrences / 32456 bytes** is complete; helper semantics, populated-map behavior, persistent-ID maps, other ancestry contexts and full installed-format coverage remain incomplete. No global semantic byte-coverage credit is assigned.

Evidence: C-712.
