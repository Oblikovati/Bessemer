### 14.364 Shell composite streams and asymmetric reader/writer framing

*Status: 19 of 20 retained composite payloads framed by explicit profiles; one historical stream remains incomplete · Evidence: C-742*

The Shells child constructor `181334DA0` installs primary table **18205A7D0**. Its virtual +260 slot (**18205AA30**) points to **181345A50**, binding the retained composite serializer to the manager's child call. C-742 checks the installed pointer, constructor/function bytes, decompilation hashes, and literals **CompositeFlipTurn** (`18205B028`) and **CompositeExtra** (`18205B050`). These are independent streams opened by the child, not inline shell Property tails.

CompositeFlipTurn contains a signed four-byte count followed by that many records of **four signed four-byte words**. The reader passes the four values to `181346F90` with the record index. Their complete business meanings remain unassigned. An independent parser rejects negative/over-limit counts and incomplete records, preserves raw bytes and rejects remainder. All **11 retained streams / 220 bytes** frame exactly.

CompositeExtra has materially different native read and write behavior:

| Profile | Fields and conditions |
|---|---|
| Reader | Three words (+37C/+3F0/+3F4); only when +3F4 equals 1, read mode +3F8 and six doubles (+400 through +428); if that mode equals 4, read the delegated persist ID. |
| Writer | Always write all four words and six doubles; append persist ID only when mode +3F8 equals 4 **and** in-memory pointer +430 is nonnull. |

Thus the fixed writer prefix is **64 bytes** even when the reader selector equals zero. The reader then consumes only 12 bytes and closes that stream; discarded bytes are not absent from the file. `cosworks_composite_layout.py` exposes explicit `reader` and `writer` profiles rather than automatically choosing whichever reaches EOF. In the writer profile the optional persist-ID body is parsed when bytes follow mode 4; its absence is allowed by the native null-pointer condition. That inference from stream extent is a framing profile, not proof of the original runtime pointer value. Both profiles preserve/reject remainder explicitly and keep delegated ID/archive boundaries bounded.

Of **nine CompositeExtra occurrences / 516 bytes**, eight 64-byte streams / 512 bytes frame completely under the writer profile; their reader profiles retain the expected 52-byte remainder. The remaining stream is **four bytes**, has candidate **840030**, and contains initial signed word 1. It is too short for either installed profile; no guessed short historical form or padding is enabled. The native composite inventory therefore has **19/20 occurrences / 732/736 bytes** completely framed under at least one named profile, not 19 native reader-consumption successes.

The audit freshly checks source envelope and nested payload identities, retains both profile attempts, and validates reconstruction for complete attempts. Ten tests pass, including explicit rejection of a 64-byte writer body by the selector-zero reader profile, populated count boundaries, truncation/remainder, and existing embedded ID/history guards. The composite selector/mode-4 persist-ID branches have synthetic coverage where no retained populated fixture establishes them.

Shell child Property, populated manager groups, the four-byte historical CompositeExtra, physical field meanings, and other installed formats remain open. No target executes and global semantic coverage credit remains zero.

Evidence: C-742.
