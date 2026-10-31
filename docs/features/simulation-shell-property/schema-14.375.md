### 14.375 Shared-base curve records and combined replay preservation

*Status: populated base curve grammar recovered and bounded synthetically; all prior affected native successes preserved · Evidence: C-755*

Shared-base reader `180B754B0` delegates populated final curve records to **180D59E50**. Its read/write order is one CwPersistStream counted UTF-16 text (destination +C0), then exactly **fifteen interleaved pairs**, each **raw double (8 bytes), signed word (4 bytes)**. Doubles target +08 through +78 and words +80 through +B8. There is no pair count or per-record version gate in this delegate; the fixed pair portion occupies **180 bytes** after the text. C-755 binds native function and decompilation hashes to the installed module.

The parent path begins from version 890011 inside the 890009 shared-base continuation, after signed mode +2E0 and count. Native mode must lie in -100..100 for records to be loaded, and the native retained count clamps to 0..50. The independent parsers now consume the recovered text/180-byte records for counts 0..50 and reject negative/over-limit counts or a populated out-of-range mode. They do not silently clamp serialized excess and pretend all records were consumed. These checks retain an explicit boundary for inputs whose complete historical wire behavior is unproven.

The bounded addition applies to the shared-base portions of Shells/MidSurfShells Property, MeshControl, ContactComponent and ContactPair decoders. Synthetic tests construct a populated curve with nonempty text and verify its endpoint before following derived fields; truncation/remainder and native retention/mode boundaries remain rejected. No marker scanning, raw-length guessing, or model-context spoofing is used.

C-755 performs a **fresh combined native replay**, not merely a retained-evidence reconciliation, over all five affected stream sets: **378 occurrences / 1489023 bytes**. Every occurrence reconstructs its full SHA-256 identity and reaches exact EOF. Every earlier successful candidate is matched by sample path, stream, version, endpoint and reconstruction hash and remains successful. Ten focused tests pass across the changed parsers and new curve tests. All retained native base curve counts are empty; populated validation is **static/synthetic only**, and the native complete occurrence count does not increase from this extension.

The separately traced newest shell-layer delegate **180D48D90** has a different body: **one raw double at +08, then counted UTF-16 text at +10**, with no internal version gate. Its caller gates record flags from 891106 and may invoke it independently for bits 1 and 2. This static layout is now established, but the existing shell parser still stops at that unrepresented newest-layer branch until caller/model-dependent presence handling is separately bounded and verified. These records are not aliases of the fifteen-pair base curves or Material's delegated point tables.

No target executes. Populated native curve fixtures, newest layer-flag handling, nonzero virtual +50 records, historical mismatches, field meanings and complete installed-format coverage remain open. Global semantic coverage credit remains zero.

Evidence: C-755.
