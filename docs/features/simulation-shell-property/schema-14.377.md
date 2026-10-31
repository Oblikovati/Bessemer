### 14.377 Newest shell layer flags with explicit runtime presence

*Status: descriptor grammar implemented as a conditional profile; populated native layer fixture remains absent · Evidence: C-757*

From version **891106**, shell serializer 181345A50 obtains the just-created layer through helper **18133E980** and reads a flags word only when the returned pointer is nonnull. Native lookup checks a one-based index against the vector at outer shell +388/+390 and returns that stored pointer. Layer insertion helper **181346AA0** accepts indices 1..loaded +36C, allocates a layer and inserts/replaces a pointer. Allocation/runtime state therefore affects whether the caller consumes a flag word; the file does not contain an extra presence byte encoding that decision. C-757 binds insertion function bytes/decompilation, lookup bytes, caller hash and version-gate bytes.

The independent shell decoder now accepts optional explicit **layer_presence**: a list/tuple of booleans with exactly the saved layer count. At the newest composite-layer branch, an unknown or invalid vector remains an explicit stop. `False` skips the flag word, matching a null lookup; `True` reads the signed four-byte flag word and follows its two known bits. This is an externally supplied conditional profile, not a heuristic default inferred from version or remaining bytes. Empty/older branches do not require a vector because they make no such wire decision.

For a present layer, **bit 1** and **bit 2** each independently introduce one body through **180D48D90**: raw eight-byte double, then CwPersistStream counted UTF-16 text. If both bits are set, bit-1 body precedes bit-2 body. Other flag bits are preserved but cause no additional reads in the retained native caller. The double's missing read destination in decompilation does not remove its eight wire bytes. Descriptor bodies have no internal count or version field and are distinct from shared-base fifteen-pair curves and inline Material records.

Synthetic tests cover flags 0/1/2/3/0x80, explicit null-layer behavior, vector length/type boundaries, raw double bits, and descriptor/parent truncation. They verify the parent continues at its +480/+488 tail after descriptors. Default parsing keeps the presence-dependent branch unresolved rather than automatically choosing the profile that reaches EOF.

C-757 freshly replays all **378 affected retained native streams / 1489023 bytes** and preserves every prior successful version attempt by identity, endpoint and reconstruction hash. Fourteen focused tests pass. No native populated newest-layer flag fixture is established by these results, and runtime presence cannot be reconstructed universally from this audit alone. The newest branch is now documented/decodable with explicit context; unconditional standalone schema completion remains unproven.

No target executes. Historical mismatches, native populated profile fixtures, field meanings and complete coverage of all installed formats remain open. Global semantic coverage credit remains zero.

Evidence: C-757.
